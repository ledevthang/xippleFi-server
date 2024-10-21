use axum::http::StatusCode;
use axum_derive_error::ErrorResponse;

#[allow(dead_code)]
#[derive(ErrorResponse, thiserror::Error)]
pub enum ServerError {
    // http rejected error
    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    PathRejection(#[from] axum::extract::rejection::PathRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    FormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    QueryRejection(#[from] axum::extract::rejection::QueryRejection),

    #[error(transparent)]
    #[status(StatusCode::BAD_REQUEST)]
    BodyRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error("{0:#?}")]
    #[status(StatusCode::BAD_REQUEST)]
    BadRequest(String),

    #[error("{0:#?}")]
    #[status(StatusCode::UNAUTHORIZED)]
    Unauthorized(String),

    #[error("{0:#?}")]
    Internal(String),

    // exection error
    #[error("{0}")]
    EnvError(String),

    #[error("Http custom error>> {0:#?}")]
    Custom(String),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Database(#[from] database::sea_orm::error::DbErr),

    #[error(transparent)]
    RedisPool(#[from] deadpool_redis::PoolError),

    #[error(transparent)]
    RedisCreatePool(#[from] deadpool_redis::CreatePoolError),

    #[error(transparent)]
    Redis(#[from] deadpool_redis::redis::RedisError),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl From<shared::EnvError> for ServerError {
    fn from(value: shared::EnvError) -> Self {
        Self::EnvError(value.0)
    }
}
