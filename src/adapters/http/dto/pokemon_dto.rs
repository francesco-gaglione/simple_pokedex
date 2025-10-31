use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PokemonResponseDto {
    name: String,
    description: String,
    habitat: String,
    is_legendary: bool,
}

impl PokemonResponseDto {
    pub fn new(name: String, description: String, habitat: String, is_legendary: bool) -> Self {
        Self {
            name,
            description,
            habitat,
            is_legendary,
        }
    }
}
