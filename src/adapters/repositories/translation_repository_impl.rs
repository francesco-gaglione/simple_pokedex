use async_trait::async_trait;
use std::sync::Arc;

use crate::adapters::cache::translation_cache::TranslationCache;
use crate::adapters::clients::funtranslations::FunsTranslationsApiClient;
use crate::application::app_error::AppResult;
use crate::application::traits::TranslationRepository;

pub struct TranslationRepositoryImpl {
    fun_translations_client: Arc<FunsTranslationsApiClient>,
    cache: TranslationCache,
}

impl TranslationRepositoryImpl {
    pub fn new(fun_translations_client: Arc<FunsTranslationsApiClient>) -> Self {
        let cache = TranslationCache::new(300);

        Self {
            fun_translations_client,
            cache,
        }
    }
}

#[async_trait]
impl TranslationRepository for TranslationRepositoryImpl {
    async fn get_shakespeare(&self, description: &str) -> AppResult<String> {
        let cache_key = format!("shakespeare_{}", description);

        if let Some(cached) = self.cache.get(&cache_key).await {
            tracing::info!("Shakespeare translation cache hit");
            return Ok(cached);
        }

        tracing::info!("Shakespeare translation cache miss");

        let translation = self
            .fun_translations_client
            .shakespeare(description)
            .await?;

        self.cache.insert(cache_key, translation.clone()).await;

        Ok(translation)
    }

    async fn get_yoda(&self, description: &str) -> AppResult<String> {
        let cache_key = format!("yoda_{}", description);

        if let Some(cached) = self.cache.get(&cache_key).await {
            tracing::info!("Yoda translation cache hit");
            return Ok(cached);
        }

        tracing::info!("Yoda translation cache miss");

        let translation = self.fun_translations_client.yoda(description).await?;
        self.cache.insert(cache_key, translation.clone()).await;

        Ok(translation)
    }
}
