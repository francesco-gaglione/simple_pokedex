use utoipa::OpenApi;

use crate::adapters::http::dto::pokemon_dto::PokemonResponseDto;

pub const POKEMON_TAG: &str = "Pokemon";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Simple Pokedex",
        version = "0.1.0"
    ),
    tags(
        (name = POKEMON_TAG, description = "Pokemon APIs"),
    ),
    paths(
        crate::adapters::http::routes::pokemon::pokemon_api,
        crate::adapters::http::routes::pokemon::pokemon_translated_api
    ),
    components(
        schemas(PokemonResponseDto),
        schemas(PokemonResponseDto),
   )
)]
pub struct ApiDoc;
