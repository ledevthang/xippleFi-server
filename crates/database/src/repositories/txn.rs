use sea_orm::{
    prelude::{DateTimeWithTimeZone, Decimal},
    DatabaseConnection, DbErr, EntityTrait, Set,
};

use crate::{
    entities::{borrow_txn, repay_txn, supply_txn, withdraw_txn},
    enums::InterestRateMode,
};

pub struct CreateTxnParams {
    pub hash: String,
    pub sender: String,
    pub date: DateTimeWithTimeZone,
    pub amount: Decimal,
    pub reserve: String,
}

pub async fn create_borrow_txn(
    db: &DatabaseConnection,
    params: CreateTxnParams,
    mode: InterestRateMode,
    be_half_of: String,
    borrow_rate: Decimal,
) -> Result<(), DbErr> {
    let model = borrow_txn::ActiveModel {
        hash: Set(params.hash),
        amount: Set(params.amount),
        date: Set(params.date),
        reserve: Set(params.reserve),
        sender: Set(params.sender),
        borrow_rate: Set(borrow_rate),
        be_half_of: Set(be_half_of),
        mode: Set(mode),
    };

    borrow_txn::Entity::insert(model).exec(db).await?;

    Ok(())
}

pub async fn create_repay_txn(
    db: &DatabaseConnection,
    params: CreateTxnParams,
    repayer: String,
) -> Result<(), DbErr> {
    let model = repay_txn::ActiveModel {
        hash: Set(params.hash),
        amount: Set(params.amount),
        date: Set(params.date),
        reserve: Set(params.reserve),
        sender: Set(params.sender),
        repayer: Set(repayer),
    };

    repay_txn::Entity::insert(model).exec(db).await?;

    Ok(())
}

pub async fn create_withdraw_txn(
    db: &DatabaseConnection,
    params: CreateTxnParams,
    to: String,
) -> Result<(), DbErr> {
    let model = withdraw_txn::ActiveModel {
        hash: Set(params.hash),
        amount: Set(params.amount),
        date: Set(params.date),
        reserve: Set(params.reserve),
        sender: Set(params.sender),
        to: Set(to),
    };

    withdraw_txn::Entity::insert(model).exec(db).await?;

    Ok(())
}

pub async fn create_supply_txn(
    db: &DatabaseConnection,
    params: CreateTxnParams,
    be_half_of: String,
) -> Result<(), DbErr> {
    let model = supply_txn::ActiveModel {
        hash: Set(params.hash),
        amount: Set(params.amount),
        date: Set(params.date),
        reserve: Set(params.reserve),
        sender: Set(params.sender),
        be_half_of: Set(be_half_of),
    };

    supply_txn::Entity::insert(model).exec(db).await?;

    Ok(())
}
