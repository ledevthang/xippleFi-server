use sea_orm::{sea_query::OnConflict, DatabaseConnection, DbErr, EntityTrait, Set};

use crate::entities::setting;

pub enum Config {
    LatestScannedBlock,
}

pub async fn find(db: &DatabaseConnection, key: &Config) -> Result<Option<String>, DbErr> {
    let value = setting::Entity::find_by_id(key.as_str_key())
        .one(db)
        .await?
        .map(|record| record.val);

    Ok(value)
}

pub async fn set(db: &DatabaseConnection, key: &Config, val: String) -> Result<(), DbErr> {
    setting::Entity::insert(setting::ActiveModel {
        key: Set(key.as_str_key().to_string()),
        val: Set(val),
    })
    .on_conflict(
        OnConflict::column(setting::Column::Key)
            .update_column(setting::Column::Val)
            .to_owned(),
    )
    .exec(db)
    .await?;

    Ok(())
}

impl Config {
    fn as_str_key(&self) -> &str {
        match self {
            Config::LatestScannedBlock => "latest_scanned_block",
        }
    }
}
