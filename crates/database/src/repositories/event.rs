use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::entities::event;

pub async fn find_by_hash(
    db: &DatabaseConnection,
    hash: &str,
) -> Result<Option<event::Model>, DbErr> {
    event::Entity::find_by_id(hash).one(db).await
}
