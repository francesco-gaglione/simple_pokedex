use crate::{
    adapters::clients::dto::translations::{TranslationRequest, TranslationResponse},
    application::app_error::{AppError, AppResult},
};

pub struct FunsTranslationsApiClient;

impl FunsTranslationsApiClient {
    pub fn new() -> Self {
        FunsTranslationsApiClient {}
    }

    pub async fn shakespeare(&self, text: &str) -> AppResult<String> {
        let url = "https://api.funtranslations.com/translate/shakespeare.json";

        let request_body = TranslationRequest {
            text: text.to_string(),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                AppError::GenericError(format!("Failed to fetch from FunTranslations API: {}", e))
            })?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let translation_response =
                    response.json::<TranslationResponse>().await.map_err(|e| {
                        AppError::GenericError(format!(
                            "Failed to parse FunTranslations response: {}",
                            e
                        ))
                    })?;

                tracing::debug!("Translated: {}", translation_response.contents.translated);

                Ok(translation_response.contents.translated)
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(AppError::GenericError(
                "Rate limit exceeded for FunTranslations API".to_string(),
            )),
            reqwest::StatusCode::BAD_REQUEST => Err(AppError::BadRequest(
                "Invalid request to FunTranslations API".to_string(),
            )),
            status => Err(AppError::GenericError(format!(
                "FunTranslations API error: {}",
                status
            ))),
        }
    }

    pub async fn yoda(&self, text: &str) -> AppResult<String> {
        let url = "https://api.funtranslations.com/translate/yoda.json";

        let request_body = TranslationRequest {
            text: text.to_string(),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                AppError::GenericError(format!("Failed to fetch from FunTranslations API: {}", e))
            })?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let translation_response =
                    response.json::<TranslationResponse>().await.map_err(|e| {
                        AppError::GenericError(format!(
                            "Failed to parse FunTranslations response: {}",
                            e
                        ))
                    })?;

                tracing::debug!("Translated: {}", translation_response.contents.translated);

                Ok(translation_response.contents.translated)
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(AppError::GenericError(
                "Rate limit exceeded for FunTranslations API".to_string(),
            )),
            reqwest::StatusCode::BAD_REQUEST => Err(AppError::BadRequest(
                "Invalid request to FunTranslations API".to_string(),
            )),
            status => Err(AppError::GenericError(format!(
                "FunTranslations API error: {}",
                status
            ))),
        }
    }
}
