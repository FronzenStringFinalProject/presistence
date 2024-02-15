use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

use crate::input_models::parents::NewParent;

impl super::Insert {
    /// 新家长注册
    pub async fn new_parent(
        db: &impl ConnectionTrait,
        NewParent {
            identity,
            name,
            password,
            secret,
        }: NewParent,
    ) -> Result<(), sea_orm::DbErr> {
        let active = super::model::ActiveModel {
            name: Set(name),
            unique_id: Set(identity),
            password: Set(password),
            secret: Set(secret),
            ..Default::default()
        };
        active.save(db).await?;
        Ok(())
    }
}
