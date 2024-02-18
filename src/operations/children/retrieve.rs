use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QuerySelect};

use super::{
    model::{Entity, Model},
    Retrieve,
};

impl Retrieve {
    pub async fn by_id(&self, db: &impl ConnectionTrait, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
    pub async fn all(&self,db:&impl ConnectionTrait,page:u64,page_size:u64)->Result<Vec<Model>,DbErr>{
        Entity::find().limit(page_size).offset((page-1) * page_size).all(db).await
    }

    
}
