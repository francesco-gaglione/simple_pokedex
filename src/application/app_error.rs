use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Generic error: {0}")]
    GenericError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

pub type AppResult<T> = Result<T, AppError>;
