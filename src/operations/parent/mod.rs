mod retrieve;
mod update;
use crate::db_operate;

mod insert;

db_operate!(ParentOperate);

pub(self) use crate::entities::parent as model;
