use sea_orm::{
    prelude::{DateTimeWithTimeZone, Decimal},
    DatabaseConnection, DbErr, EntityTrait, Set, TransactionTrait,
};

use crate::{
    entities::{borrow_txn, event, repay_txn, supply_txn, withdraw_txn},
    enums::{EventName, InterestRateMode},
};

pub struct CreateTxnParams {
    pub hash: String,
    pub sender: String,
    pub date: DateTimeWithTimeZone,
    pub amount: Decimal,
    pub reserve: String,
    pub full_payload: serde_json::Value,
}

pub async fn create_borrow_txn(
    db: &DatabaseConnection,
    params: CreateTxnParams,
    mode: InterestRateMode,
    be_half_of: String,
    borrow_rate: Decimal,
) -> Result<(), DbErr> {
    let tx = db.begin().await?;

    let txn = borrow_txn::ActiveModel {
        hash: Set(params.hash.clone()),
        amount: Set(params.amount),
        date: Set(params.date),
        reserve: Set(params.reserve),
        sender: Set(params.sender),
        borrow_rate: Set(borrow_rate),
        be_half_of: Set(be_half_of),
        mode: Set(mode),
    };

    let event = event::ActiveModel {
        hash: Set(params.hash),
        date: Set(params.date),
        event_data: Set(params.full_payload),
        name: Set(EventName::Borrow),
    };

    event::Entity::insert(event).exec(&tx).await?;
    borrow_txn::Entity::insert(txn).exec(&tx).await?;

    tx.commit().await?;

    Ok(())
}

pub async fn create_repay_txn(
    db: &DatabaseConnection,
    params: CreateTxnParams,
    repayer: String,
) -> Result<(), DbErr> {
    let tx = db.begin().await?;

    let model = repay_txn::ActiveModel {
        hash: Set(params.hash.clone()),
        amount: Set(params.amount),
        date: Set(params.date),
        reserve: Set(params.reserve),
        sender: Set(params.sender),
        repayer: Set(repayer),
    };

    let event = event::ActiveModel {
        hash: Set(params.hash),
        date: Set(params.date),
        event_data: Set(params.full_payload),
        name: Set(EventName::Repay),
    };

    event::Entity::insert(event).exec(&tx).await?;
    repay_txn::Entity::insert(model).exec(&tx).await?;

    tx.commit().await?;

    Ok(())
}

pub async fn create_withdraw_txn(
    db: &DatabaseConnection,
    params: CreateTxnParams,
    to: String,
) -> Result<(), DbErr> {
    let tx = db.begin().await?;

    let model = withdraw_txn::ActiveModel {
        hash: Set(params.hash.clone()),
        amount: Set(params.amount),
        date: Set(params.date),
        reserve: Set(params.reserve),
        sender: Set(params.sender),
        to: Set(to),
    };

    let event = event::ActiveModel {
        hash: Set(params.hash),
        date: Set(params.date),
        event_data: Set(params.full_payload),
        name: Set(EventName::Withdraw),
    };

    event::Entity::insert(event).exec(&tx).await?;
    withdraw_txn::Entity::insert(model).exec(&tx).await?;

    tx.commit().await?;

    Ok(())
}

pub async fn create_supply_txn(
    db: &DatabaseConnection,
    params: CreateTxnParams,
    be_half_of: String,
) -> Result<(), DbErr> {
    let tx = db.begin().await?;

    let model = supply_txn::ActiveModel {
        hash: Set(params.hash.clone()),
        amount: Set(params.amount),
        date: Set(params.date),
        reserve: Set(params.reserve),
        sender: Set(params.sender),
        be_half_of: Set(be_half_of),
    };

    let event = event::ActiveModel {
        hash: Set(params.hash),
        date: Set(params.date),
        event_data: Set(params.full_payload),
        name: Set(EventName::Suppy),
    };

    event::Entity::insert(event).exec(&tx).await?;
    supply_txn::Entity::insert(model).exec(&tx).await?;

    tx.commit().await?;

    Ok(())
}
