use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "event_name")]
pub enum EventName {
    #[sea_orm(string_value = "borrow")]
    Borrow,
    #[sea_orm(string_value = "repay")]
    Repay,
    #[sea_orm(string_value = "suppy")]
    Suppy,
    #[sea_orm(string_value = "withdraw")]
    Withdraw,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "interest_rate_mode")]
pub enum InterestRateMode {
    #[sea_orm(string_value = "none")]
    None,
    #[sea_orm(string_value = "stable")]
    Stable,
    #[sea_orm(string_value = "variable")]
    Variable,
}
