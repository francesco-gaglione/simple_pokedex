use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapters::clients::funtranslations::FunsTranslationsApiClient,
    application::{app_error::AppResult, traits::TranslationRepository},
};

pub struct TranslationRepositoryImpl {
    fun_translations_client: Arc<FunsTranslationsApiClient>,
}

impl TranslationRepositoryImpl {
    pub fn new(fun_translations_client: Arc<FunsTranslationsApiClient>) -> Self {
        Self {
            fun_translations_client,
        }
    }
}

#[async_trait]
impl TranslationRepository for TranslationRepositoryImpl {
    async fn get_shakespeare(&self, description: &str) -> AppResult<String> {
        self.fun_translations_client.shakespeare(description).await
    }

    async fn get_yoda(&self, description: &str) -> AppResult<String> {
        self.fun_translations_client.yoda(description).await
    }
}
