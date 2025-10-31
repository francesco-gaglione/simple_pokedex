use crate::{
    adapters::clients::dto::pokemon_species_dto::PokemonSpeciesResponseDto,
    application::app_error::{AppError, AppResult},
};

pub struct PokeApiClient;

impl PokeApiClient {
    pub fn new() -> Self {
        PokeApiClient {}
    }

    pub async fn pokemon_species(&self, name: &str) -> AppResult<PokemonSpeciesResponseDto> {
        let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}/", name);

        let response = reqwest::get(&url)
            .await
            .map_err(|e| AppError::GenericError(format!("Failed to fetch from PokeAPI: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::NotFound(format!(
                "Pokemon species '{}' not found",
                name
            )));
        }

        let pokemon = response
            .json::<PokemonSpeciesResponseDto>()
            .await
            .map_err(|e| {
                AppError::GenericError(format!("Failed to parse PokeAPI response: {}", e))
            })?;

        Ok(pokemon)
    }
}
