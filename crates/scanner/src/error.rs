use std::num::ParseIntError;

use alloy::transports::{RpcError, TransportErrorKind};

#[derive(thiserror::Error, Debug)]
pub enum ScannerError {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    RpcTransportError(#[from] RpcError<TransportErrorKind>),

    #[error(transparent)]
    DbError(#[from] database::sea_orm::error::DbErr),

    #[error(transparent)]
    ParseError(#[from] ParseIntError),
}
