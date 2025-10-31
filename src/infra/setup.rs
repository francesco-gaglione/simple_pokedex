use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

use crate::adapters::clients::funtranslations::FunsTranslationsApiClient;
use crate::adapters::clients::pokeapi::PokeApiClient;
use crate::adapters::http::app_state::AppState;
use crate::adapters::repositories::pokemon_repository_impl::PokemonRepositoryImpl;
use crate::adapters::repositories::translation_repository_impl::TranslationRepositoryImpl;
use crate::application::use_cases::pokemon_use_cases::PokemonUseCases;
use crate::infra::config::AppConfig;

pub async fn init_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let config = AppConfig::from_env();

    let pokeapi_client = Arc::new(PokeApiClient::new());
    let fun_translations_client = Arc::new(FunsTranslationsApiClient::new());

    let pokemon_repository = Arc::new(PokemonRepositoryImpl::new(pokeapi_client));
    let translation_repository = Arc::new(TranslationRepositoryImpl::new(fun_translations_client));

    let pokemon_use_cases =
        PokemonUseCases::new(pokemon_repository.clone(), translation_repository.clone());

    Ok(AppState {
        config,
        pokemon_use_cases: Arc::new(pokemon_use_cases),
    })
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "axum_trainer=debug,tower_http=debug".into());

    let console_layer = fmt::layer()
        .with_target(false) // don’t show target (module path)
        .with_level(true) // show log level
        .pretty(); // human-friendly, with colors

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .try_init()
        .ok();
}
