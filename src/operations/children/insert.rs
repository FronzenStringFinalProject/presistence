use sea_orm::{ConnectionTrait, DbErr, EntityTrait, Set};

use super::model::ActiveModel;

impl super::Insert {
    pub async fn new_child(
        &self,
        db: &impl ConnectionTrait,
        child_name: String,
        parent_id: i32,
    ) -> Result<i32, DbErr> {
        let active = ActiveModel {
            name: Set(child_name),
            parent: Set(parent_id),
            ..Default::default()
        };

        super::model::Entity::insert(active)
            .exec(db)
            .await
            .map(|ret| ret.last_insert_id)
    }
}
