use thiserror::Error;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Invalid session")]
    InvalidSession,

    #[error("Session expired")]
    SessionExpired,

    #[error("Vault not found")]
    VaultNotFound,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Transaction signing failed: {0}")]
    SigningError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Suspicious activity detected")]
    SuspiciousActivity,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Solana error: {0}")]
    SolanaError(String),

    #[error("Internal server error")]
    InternalError,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found")]
    NotFound,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidSession => StatusCode::BAD_REQUEST,
            AppError::SessionExpired => StatusCode::UNAUTHORIZED,
            AppError::VaultNotFound => StatusCode::NOT_FOUND,
            AppError::InsufficientFunds => StatusCode::BAD_REQUEST,
            AppError::CryptoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SigningError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            AppError::SuspiciousActivity => StatusCode::FORBIDDEN,
            AppError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            AppError::SolanaError(_) => StatusCode::BAD_GATEWAY,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "error": self.to_string(),
            "code": self.status_code().as_u16(),
        }))
    }
}
