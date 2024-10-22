use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "withdraw_txn")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub hash: String,
    pub reserve: String,
    pub date: DateTimeWithTimeZone,
    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub amount: Decimal,
    pub sender: String,
    pub to: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
