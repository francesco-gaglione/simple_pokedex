use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TranslationRequest {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct TranslationResponse {
    pub success: Option<Success>,
    pub contents: Contents,
}

#[derive(Debug, Deserialize)]
pub struct Success {
    pub total: i32,
}

#[derive(Debug, Deserialize)]
pub struct Contents {
    pub translated: String,
    pub text: String,
    pub translation: String,
}
