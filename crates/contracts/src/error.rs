#[derive(thiserror::Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    TypeError(#[from] alloy::sol_types::Error),

    #[error(transparent)]
    ContractIntegrationError(#[from] alloy::contract::Error),

    #[error(transparent)]
    PendingTxError(#[from] alloy::providers::PendingTransactionError),
}
