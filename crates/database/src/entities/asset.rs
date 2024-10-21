use sea_orm::entity::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, ToSchema)]
#[sea_orm(table_name = "asset")]
#[serde(rename_all = "camelCase")]
#[schema(as = AssetModel)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub real_time_price: Decimal,

    #[sea_orm(column_type = "Decimal(Some((90, 0)))")]
    pub price: Decimal,

    pub symbol: String,

    #[serde(skip)]
    pub updated_at: DateTimeWithTimeZone,

    #[serde(skip)]
    pub timeline: DateTimeWithTimeZone,

    #[sea_orm(column_type = "Float")]
    pub apy: f32,

    #[sea_orm(unique)]
    pub address: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
