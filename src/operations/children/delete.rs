use super::model::{ActiveModel, Entity};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, Set};

impl super::Delete {
    pub async fn by_id(&self, db: &impl ConnectionTrait, child_id: i32) -> Result<(), DbErr> {
        Entity::delete(ActiveModel {
            cid: Set(child_id),
            ..Default::default()
        })
        .exec(db)
        .await?;
        Ok(())
    }
}
