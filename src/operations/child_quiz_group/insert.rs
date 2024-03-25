use super::model::{ActiveModel, Entity};
use sea_orm::ActiveValue::Set;
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

impl super::Insert {
    pub async fn add(
        &self,
        db: &impl ConnectionTrait,
        child_id: i32,
        quiz_group: i32,
    ) -> Result<(), DbErr> {
        let active = ActiveModel {
            cid: Set(child_id),
            gid: Set(quiz_group),
        };

        Entity::insert(active).exec(db).await?;
        Ok(())
    }
}
