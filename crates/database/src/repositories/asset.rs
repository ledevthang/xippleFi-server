use chrono::{TimeDelta, Utc};
use sea_orm::{
    prelude::{Decimal, Expr},
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use shared::constants::Asset;

use crate::entities::asset;

pub async fn create(
    db: &DatabaseConnection,
    asset: &Asset,
    price: Decimal,
) -> Result<asset::Model, DbErr> {
    let config = asset.as_config();

    let model = asset::ActiveModel {
        id: Set(config.coincap_id.to_string()),
        address: Set(config.address.to_string()),
        apy: Set(config.apy),
        price: Set(price),
        real_time_price: Set(price),
        symbol: Set(config.symbol.to_string()),
        timeline: Set(chrono::Utc::now().into()),
        updated_at: Default::default(),
    };

    asset::Entity::insert(model).exec_with_returning(db).await
}

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: &Asset,
) -> Result<Option<asset::Model>, DbErr> {
    asset::Entity::find_by_id(id.as_config().coincap_id)
        .one(db)
        .await
}

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<asset::Model>, DbErr> {
    asset::Entity::find().all(db).await
}

pub async fn update_realtime_price(
    db: &DatabaseConnection,
    id: &Asset,
    price: Decimal,
) -> Result<(), DbErr> {
    asset::Entity::update_many()
        .col_expr(asset::Column::RealTimePrice, Expr::value(price))
        .filter(asset::Column::Id.eq(id.as_config().coincap_id))
        .exec(db)
        .await
        .map(|_| ())
}

// update realtime price and timeline extra
pub async fn udate_price(db: &DatabaseConnection, id: &Asset, price: Decimal) -> Result<(), DbErr> {
    let asset_config = id.as_config();

    let timeline = Utc::now()
        .checked_add_signed(TimeDelta::seconds(asset_config.heartbeat.into()))
        .ok_or(DbErr::Custom(
            "can not adding timeline from heartbeat seconds".to_string(),
        ))?;

    asset::Entity::update_many()
        .col_expr(asset::Column::RealTimePrice, Expr::value(price))
        .col_expr(asset::Column::Price, Expr::value(price))
        .col_expr(asset::Column::Timeline, Expr::value(timeline))
        .filter(asset::Column::Id.eq(asset_config.coincap_id))
        .exec(db)
        .await
        .map(|_| ())
}

pub async fn find_if_timeline_reached(db: &DatabaseConnection) -> Result<Vec<asset::Model>, DbErr> {
    asset::Entity::find()
        .filter(asset::Column::Timeline.lte(Utc::now()))
        .all(db)
        .await
}
