use crate::{application::use_cases::pokemon_use_cases::PokemonUseCases, infra::config::AppConfig};
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pokemon_use_cases: Arc<PokemonUseCases>,
}

impl FromRef<AppState> for Arc<PokemonUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.pokemon_use_cases.clone()
    }
}
