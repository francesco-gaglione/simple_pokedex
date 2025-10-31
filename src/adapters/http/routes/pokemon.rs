use axum::extract::Path;

use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::pokemon_dto::PokemonResponseDto;
use crate::adapters::openapi::POKEMON_TAG;
use crate::application::app_error::AppResult;
use axum::Json;
use axum::extract::State;

#[utoipa::path(
    get,
    path = "/pokemon/{pokemon_name}",
    tag = POKEMON_TAG,
    responses(
        (status = 200, description = "Pokemon retrieved successfully", body = PokemonResponseDto),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn pokemon_api(
    Path(pokemon_name): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<PokemonResponseDto>> {
    tracing::debug!("Retrieving pokemon: {}", pokemon_name);

    let pokemon = state
        .pokemon_use_cases
        .get_pokemon(pokemon_name.as_str())
        .await?;

    Ok(Json(PokemonResponseDto::new(
        pokemon.name().to_string(),
        pokemon.description().to_string(),
        pokemon.habitat().to_string(),
        pokemon.is_legendary(),
    )))
}

#[utoipa::path(
    get,
    path = "/pokemon/translated/{pokemon_name}",
    tag = POKEMON_TAG,
    responses(
        (status = 200, description = "Pokemon translated retrieved successfully", body = PokemonResponseDto),
        (status = 500, description = "Internal server error")
    )
)]
#[axum_macros::debug_handler]
pub async fn pokemon_translated_api(
    Path(pokemon_name): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<PokemonResponseDto>> {
    tracing::debug!("Retrieving translated pokemon: {}", pokemon_name);

    let pokemon = state
        .pokemon_use_cases
        .get_pokemon_translated(pokemon_name.as_str())
        .await?;

    Ok(Json(PokemonResponseDto::new(
        pokemon.name().to_string(),
        pokemon.description().to_string(),
        pokemon.habitat().to_string(),
        pokemon.is_legendary(),
    )))
}
