use crate::entities::child_check::Entity;
use sea_orm::prelude::Date;
use sea_orm::{DerivePartialModel, FromQueryResult};
#[derive(Debug, FromQueryResult, DerivePartialModel, Eq, PartialEq)]
#[sea_orm(entity = "Entity")]
pub struct MonthlyCheckItem {
    #[sea_orm(from_col = "date")]
    pub check_date: Date,
}
