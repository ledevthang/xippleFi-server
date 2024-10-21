#[derive(thiserror::Error, Debug)]
pub enum OracleError {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    SocketError(#[from] tokio_tungstenite::tungstenite::error::Error),

    #[error("{0:#?}")]
    DeserializeError(String),

    #[error(transparent)]
    DbError(#[from] database::sea_orm::error::DbErr),

    #[error(transparent)]
    ContractError(#[from] contracts::error::ContractError),
}
