use std::sync::Arc;

use crate::{
    application::{
        app_error::AppResult,
        traits::{PokemonRepository, TranslationRepository},
    },
    domain::entities::pokemon::Pokemon,
};

#[derive(Clone)]
pub struct PokemonUseCases {
    pokemon_repository: Arc<dyn PokemonRepository>,
    translation_repository: Arc<dyn TranslationRepository>,
}

impl PokemonUseCases {
    pub fn new(
        pokemon_repository: Arc<dyn PokemonRepository>,
        translation_repository: Arc<dyn TranslationRepository>,
    ) -> Self {
        Self {
            pokemon_repository,
            translation_repository,
        }
    }

    pub async fn get_pokemon(&self, name: &str) -> AppResult<Pokemon> {
        self.pokemon_repository.get_pokemon(name).await
    }

    pub async fn get_pokemon_translated(&self, name: &str) -> AppResult<Pokemon> {
        let mut pokemon = self.pokemon_repository.get_pokemon(name).await?;

        let translation = if pokemon.is_cave() || pokemon.is_legendary() {
            tracing::debug!("Using Yoda translation");
            self.translation_repository
                .get_yoda(pokemon.description())
                .await
        } else {
            tracing::debug!("Using Shakespeare translation");
            self.translation_repository
                .get_shakespeare(pokemon.description())
                .await
        };

        if let Ok(translation) = translation {
            pokemon.set_translated_description(translation);
        } else {
            tracing::warn!("Failed to translate pokemon description using standard translation");
            pokemon.set_translated_description(pokemon.description().to_string());
        }

        Ok(pokemon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::{
        app_error::AppError,
        traits::{MockPokemonRepository, MockTranslationRepository},
    };
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_get_pokemon_calls_repository() {
        let mut mock_pokemon_repo = MockPokemonRepository::new();
        let mock_translation_repo = MockTranslationRepository::new();

        let expected_pokemon = Pokemon::new(
            "Bulbasaur".to_string(),
            "Seed pokemon".to_string(),
            "grassland".to_string(),
            false,
        );

        mock_pokemon_repo
            .expect_get_pokemon()
            .with(eq("bulbasaur"))
            .times(1)
            .returning(move |_| Ok(expected_pokemon.clone()));

        let use_case =
            PokemonUseCases::new(Arc::new(mock_pokemon_repo), Arc::new(mock_translation_repo));

        let result = use_case.get_pokemon("bulbasaur").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().name(), "Bulbasaur");
    }

    #[tokio::test]
    async fn test_legendary_pokemon_gets_yoda_translation() {
        let mut mock_pokemon_repo = MockPokemonRepository::new();
        let mut mock_translation_repo = MockTranslationRepository::new();

        let legendary = Pokemon::new(
            "Lugia".to_string(),
            "Legendary bird".to_string(),
            "sea".to_string(),
            true,
        );

        mock_pokemon_repo
            .expect_get_pokemon()
            .returning(move |_| Ok(legendary.clone()));

        mock_translation_repo
            .expect_get_yoda()
            .with(eq("Legendary bird"))
            .times(1)
            .returning(|_| Ok("Legendary, a bird is".to_string()));

        let use_case =
            PokemonUseCases::new(Arc::new(mock_pokemon_repo), Arc::new(mock_translation_repo));

        let result = use_case.get_pokemon_translated("lugia").await.unwrap();

        assert_eq!(result.description(), "Legendary, a bird is");
    }

    #[tokio::test]
    async fn test_cave_pokemon_gets_yoda_translation() {
        let mut mock_pokemon_repo = MockPokemonRepository::new();
        let mut mock_translation_repo = MockTranslationRepository::new();

        let cave_pokemon = Pokemon::new(
            "Zubat".to_string(),
            "Small bat pokemon".to_string(),
            "cave".to_string(),
            false,
        );

        mock_pokemon_repo
            .expect_get_pokemon()
            .returning(move |_| Ok(cave_pokemon.clone()));

        mock_translation_repo
            .expect_get_yoda()
            .times(1)
            .returning(|_| Ok("Small, a bat pokemon is".to_string()));

        let use_case =
            PokemonUseCases::new(Arc::new(mock_pokemon_repo), Arc::new(mock_translation_repo));

        let result = use_case.get_pokemon_translated("zubat").await.unwrap();

        assert_eq!(result.description(), "Small, a bat pokemon is");
    }

    #[tokio::test]
    async fn test_regular_pokemon_gets_shakespeare_translation() {
        let mut mock_pokemon_repo = MockPokemonRepository::new();
        let mut mock_translation_repo = MockTranslationRepository::new();

        let regular = Pokemon::new(
            "Pidgeot".to_string(),
            "Bird pokemon".to_string(),
            "sky".to_string(),
            false,
        );

        mock_pokemon_repo
            .expect_get_pokemon()
            .returning(move |_| Ok(regular.clone()));

        mock_translation_repo
            .expect_get_shakespeare()
            .with(eq("Bird pokemon"))
            .times(1)
            .returning(|_| Ok("A creature of the winged persuasion".to_string()));

        let use_case =
            PokemonUseCases::new(Arc::new(mock_pokemon_repo), Arc::new(mock_translation_repo));

        let result = use_case.get_pokemon_translated("pidgeot").await.unwrap();

        assert_eq!(result.description(), "A creature of the winged persuasion");
    }

    #[tokio::test]
    async fn test_translation_failure_fallback() {
        let mut mock_pokemon_repo = MockPokemonRepository::new();
        let mut mock_translation_repo = MockTranslationRepository::new();

        let pokemon = Pokemon::new(
            "Pikachu".to_string(),
            "Electric mouse".to_string(),
            "forest".to_string(),
            false,
        );

        mock_pokemon_repo
            .expect_get_pokemon()
            .returning(move |_| Ok(pokemon.clone()));

        mock_translation_repo
            .expect_get_shakespeare()
            .returning(|_| Err(AppError::GenericError("Rate limit".to_string())));

        let use_case =
            PokemonUseCases::new(Arc::new(mock_pokemon_repo), Arc::new(mock_translation_repo));

        let result = use_case.get_pokemon_translated("pikachu").await.unwrap();

        assert_eq!(result.description(), "Electric mouse");
    }
}
