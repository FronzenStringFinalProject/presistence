mod retrieve;
mod update;

use crate::db_operate;

mod insert;

db_operate!(ParentOperate);

use crate::entities::parent as model;
