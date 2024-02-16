use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

use super::{
    model::{Entity, Model},
    Retrieve,
};

impl Retrieve {
    pub async fn by_id(&self, db: &impl ConnectionTrait, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
}
