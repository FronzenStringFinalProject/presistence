use crate::entities::{answer_record, quiz_groups, quizes};
use crate::service::DatabaseServiceTrait;
use chrono::NaiveDate;
use sea_orm::prelude::Expr;
use sea_orm::{
    ColumnTrait, Condition, DbErr, DerivePartialModel, EntityTrait, FromQueryResult,
    IntoSimpleExpr, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
use serde::Serialize;

#[derive(Debug, FromQueryResult, Serialize, DerivePartialModel)]
#[sea_orm(entity = "answer_record::Entity")]
pub struct WrongQuizItem {
    qid: i32,
    #[sea_orm(from_expr = "Expr::col(quizes::Column::Quiz).into_simple_expr()")]
    quiz: String,
    date: NaiveDate,
    #[sea_orm(from_expr = "Expr::col(quiz_groups::Column::Name).into_simple_expr()")]
    group: String,
}

impl super::ChildQuizService {
    /// 获取错题列表
    ///
    /// ```sql
    /// SELECT
    ///     quizes.qid,
    ///     quiz,
    ///     date,
    ///     quiz_groups.name
    /// FROM public.answer_record
    /// join quizes on quizes.qid = answer_record.qid
    /// join quiz_groups on quiz_groups.gid = quizes.group
    /// where cid =501 and not correct
    /// order by date,gid
    /// ```
    pub async fn get_wrong_quiz_list(&self, child_id: i32) -> Result<Vec<WrongQuizItem>, DbErr> {
        let ret = answer_record::Entity::find()
            .filter(
                Condition::all()
                    .add(answer_record::Column::Cid.eq(child_id))
                    .add(answer_record::Column::Correct.into_simple_expr()),
            )
            .order_by_desc(answer_record::Column::Date)
            .order_by_desc(quiz_groups::Column::Gid)
            .join(JoinType::Join, answer_record::Relation::Quizes.def())
            .join(JoinType::Join, quizes::Relation::QuizGroups.def())
            .into_partial_model::<WrongQuizItem>()
            .all(self.db())
            .await?;

        Ok(ret)
    }
}
