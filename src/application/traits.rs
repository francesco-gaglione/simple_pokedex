use crate::{application::app_error::AppResult, domain::entities::pokemon::Pokemon};
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PokemonRepository: Send + Sync {
    async fn get_pokemon<'a>(&'a self, name: &'a str) -> AppResult<Pokemon>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TranslationRepository: Send + Sync {
    async fn get_shakespeare(&self, description: &str) -> AppResult<String>;
    async fn get_yoda(&self, description: &str) -> AppResult<String>;
}
