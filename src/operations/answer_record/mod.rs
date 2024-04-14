mod insert;
mod retrieve;

use crate::db_operate;

db_operate!(AnswerRecordOperate);

use crate::entities::answer_record as model;
