use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TranslationRequest {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct TranslationResponse {
    #[allow(dead_code)]
    pub success: Option<Success>,
    pub contents: Contents,
}

#[derive(Debug, Deserialize)]
pub struct Success {
    #[allow(dead_code)]
    pub total: i32,
}

#[derive(Debug, Deserialize)]
pub struct Contents {
    pub translated: String,
    #[allow(dead_code)]
    pub text: String,
    #[allow(dead_code)]
    pub translation: String,
}
