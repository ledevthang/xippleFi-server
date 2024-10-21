use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};

use crate::entities::user;

pub async fn create_if_not_exist(db: &DatabaseConnection, address: String) -> Result<(), DbErr> {
    user::Entity::insert(user::ActiveModel {
        address: Set(address.to_string()),
    })
    .on_conflict_do_nothing()
    .exec(db)
    .await?;

    Ok(())
}
