use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapters::clients::pokeapi::PokeApiClient,
    application::{app_error::AppResult, traits::PokemonRepository},
    domain::entities::pokemon::Pokemon,
};

pub struct PokemonRepositoryImpl {
    pokeapi_client: Arc<PokeApiClient>,
}

impl PokemonRepositoryImpl {
    pub fn new(pokeapi_client: Arc<PokeApiClient>) -> Self {
        Self { pokeapi_client }
    }
}

#[async_trait]
impl PokemonRepository for PokemonRepositoryImpl {
    async fn get_pokemon<'a>(&'a self, name: &'a str) -> AppResult<Pokemon> {
        let pokemon = self.pokeapi_client.pokemon_species(name).await?;
        Ok(Pokemon::new(
            pokemon.name.clone(),
            pokemon
                .get_english_description()
                .unwrap_or_else(|| "Description not available".to_string()),
            pokemon
                .get_habitat()
                .unwrap_or_else(|| "Description not available".to_string()),
            pokemon.is_legendary,
        ))
    }
}
