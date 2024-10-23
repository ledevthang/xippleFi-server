use std::str::FromStr;

use alloy::primitives::U256;
use contracts::xipple_fi::VirtualXippleFi::{Borrow, InterestRateMode, Repay, Supply, Withdraw};
use database::{
    repositories::txn::{self, CreateTxnParams},
    sea_orm::{
        prelude::{DateTimeUtc, Decimal},
        DatabaseConnection,
    },
};

use crate::error::ScannerError;

pub async fn handle_borrow(
    db: &DatabaseConnection,
    hash: String,
    date: DateTimeUtc,
    payload: Borrow,
) -> Result<(), ScannerError> {
    let amount = uint256_to_decimal(payload.amount)?;
    let borrow_rate = uint256_to_decimal(payload.borrowRate)?;
    let mode = to_interest_mode_db_enums(payload.interestRateMode)?;

    let params = CreateTxnParams {
        hash,
        sender: payload.user.to_string(),
        date: date.into(),
        amount,
        reserve: payload.reserve.to_string(),
        full_payload: serde_json::json!(payload),
    };

    txn::create_borrow_txn(
        db,
        params,
        mode,
        payload.onBehalfOf.to_string(),
        borrow_rate,
    )
    .await?;

    Ok(())
}

pub async fn handle_repay(
    db: &DatabaseConnection,
    hash: String,
    date: DateTimeUtc,
    payload: Repay,
) -> Result<(), ScannerError> {
    let amount = uint256_to_decimal(payload.amount)?;

    let params = CreateTxnParams {
        hash,
        sender: payload.user.to_string(),
        date: date.into(),
        amount,
        reserve: payload.reserve.to_string(),
        full_payload: serde_json::json!(payload),
    };

    txn::create_repay_txn(db, params, payload.repayer.to_string()).await?;

    Ok(())
}

pub async fn handle_supply(
    db: &DatabaseConnection,
    hash: String,
    date: DateTimeUtc,
    payload: Supply,
) -> Result<(), ScannerError> {
    let amount = uint256_to_decimal(payload.amount)?;

    let params = CreateTxnParams {
        hash,
        sender: payload.user.to_string(),
        date: date.into(),
        amount,
        reserve: payload.reserve.to_string(),
        full_payload: serde_json::json!(payload),
    };

    txn::create_supply_txn(db, params, payload.onBehalfOf.to_string()).await?;

    Ok(())
}

pub async fn handle_withdraw(
    db: &DatabaseConnection,
    hash: String,
    date: DateTimeUtc,
    payload: Withdraw,
) -> Result<(), ScannerError> {
    let amount = uint256_to_decimal(payload.amount)?;

    let params = CreateTxnParams {
        hash,
        sender: payload.user.to_string(),
        date: date.into(),
        amount,
        reserve: payload.reserve.to_string(),
        full_payload: serde_json::json!(payload),
    };

    txn::create_withdraw_txn(db, params, payload.to.to_string()).await?;

    Ok(())
}

fn uint256_to_decimal(val: U256) -> Result<Decimal, ScannerError> {
    Decimal::from_str(&val.to_string()).map_err(|e| ScannerError::Custom(e.to_string()))
}

fn to_interest_mode_db_enums(
    value: InterestRateMode,
) -> Result<database::enums::InterestRateMode, ScannerError> {
    match value {
        InterestRateMode::NONE => Ok(database::enums::InterestRateMode::None),
        InterestRateMode::STABLE => Ok(database::enums::InterestRateMode::Stable),
        InterestRateMode::VARIABLE => Ok(database::enums::InterestRateMode::Variable),
        InterestRateMode::__Invalid => Err(ScannerError::Custom(
            "invalid inerest mode from log".to_string(),
        )),
    }
}
