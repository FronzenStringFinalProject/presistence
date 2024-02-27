use crate::entities::children;
use crate::entities::prelude::Children;
use crate::service::parent_child_service::ParentChildService;
use crate::service::DatabaseServiceTrait;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, DerivePartialModel, EntityTrait, FromQueryResult,
    QueryFilter,
};
use serde::Serialize;
#[derive(Debug, FromQueryResult, DerivePartialModel, Serialize)]
#[sea_orm(entity = "Children")]
pub struct ChildItem {
    pub cid: i32,
    pub name: String,
    pub ability: f64,
}

impl<D> ParentChildService<D> {
    pub async fn all_children(&self, parent_id: i32) -> Result<Vec<ChildItem>, DbErr>
    where
        D: ConnectionTrait,
    {
        let all_children = Children::find()
            .filter(children::Column::Parent.eq(parent_id))
            .into_partial_model::<ChildItem>()
            .all(self.db())
            .await?;

        Ok(all_children)
    }
}
