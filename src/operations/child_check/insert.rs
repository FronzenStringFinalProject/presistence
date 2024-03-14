use super::{model, Insert};
use sea_orm::ActiveValue::Set;
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

impl Insert {
    pub async fn check(&self, db: &impl ConnectionTrait, child_id: i32) -> Result<(), DbErr> {
        let active = model::ActiveModel {
            cid: Set(child_id),
            ..Default::default()
        };
        model::Entity::insert(active).exec(db).await?;
        Ok(())
    }
}
