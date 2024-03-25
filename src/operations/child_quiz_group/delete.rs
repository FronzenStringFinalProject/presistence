use super::model::Entity;
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

impl super::Delete {
    pub async fn one(
        &self,
        db: &impl ConnectionTrait,
        child_id: i32,
        group_id: i32,
    ) -> Result<(), DbErr> {
        Entity::delete_by_id((child_id, group_id)).exec(db).await?;
        Ok(())
    }
}
