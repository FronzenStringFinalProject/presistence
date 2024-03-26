use crate::entities::child_check::Entity;
use sea_orm::prelude::Date;
use sea_orm::{DerivePartialModel, FromQueryResult};
use serde::Serialize;

#[derive(Debug, FromQueryResult, DerivePartialModel, Eq, PartialEq, Serialize)]
#[sea_orm(entity = "Entity")]
pub struct MonthlyCheckItem {
    #[sea_orm(from_col = "date")]
    pub check_date: Date,
}
