use crate::entities::prelude::Children;
use crate::entities::{answer_record, children, quiz_groups, quizes};
use sea_orm::sea_query::{Asterisk, Expr};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, DerivePartialModel, EntityTrait,
    FromQueryResult, JoinType, QueryFilter, QuerySelect, RelationTrait, Select,
};
use serde::Serialize;

#[derive(Clone, Copy)]
pub enum ResentType {
    Days(u32),
    Quiz(u32),
}

trait ResentFilter {
    fn modify_filter(self, resent: ResentType) -> Self;
}

impl ResentFilter for Condition {
    fn modify_filter(self, resent: ResentType) -> Self {
        match resent {
            ResentType::Days(day) => self.add(
                Expr::col(answer_record::Column::Date)
                    .gte(Expr::cust_with_expr("DATE(NOW())-$1", Expr::val(day))),
            ),
            ResentType::Quiz(_) => self,
        }
    }
}

trait ResentLimit {
    fn add_limit(self, resent_type: ResentType) -> Self;
}

impl<E: EntityTrait> ResentLimit for Select<E> {
    fn add_limit(self, resent_type: ResentType) -> Self {
        match resent_type {
            ResentType::Days(_) => self,
            ResentType::Quiz(num) => self.limit(num as u64),
        }
    }
}

use crate::entities::prelude::QuizGroups;
#[derive(Debug, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "QuizGroups")]
pub struct ChildStaticalItem {
    #[sea_orm(from_col = "gid")]
    pub quiz_ty_id: i32,
    #[sea_orm(from_col = "name")]
    pub quiz_ty: String,
    #[sea_orm(from_expr = "Expr::col(Asterisk).count()")]
    pub total: i64,
    #[sea_orm(from_expr = "Expr::cust_with_exprs(\"$1 FILTER (WHERE $2)\",\
    [Expr::col(Asterisk).count(),answer_record::Column::Correct.eq(true)])")]
    pub correct: i64,
    #[sea_orm(from_expr = "Expr::cust_with_exprs(\"$1 FILTER (WHERE $2)\",\
    [Expr::col(Asterisk).count(),answer_record::Column::Correct.eq(false)])")]
    pub wrong: i64,
    #[sea_orm(from_expr = "Expr::cust_with_exprs(\
        \"CAST($1 FILTER (WHERE $2) AS DOUBLE PRECISION)/CAST($1 AS DOUBLE PRECISION)\",\
        [Expr::col(Asterisk).count(),answer_record::Column::Correct.eq(false)])")]
    pub correct_rate: f64,
}

impl super::ChildQuizService {
    /// 孩子统计信息
    ///
    /// 统计内容
    /// 1. 孩子最近的正确率
    /// 2. 孩子各个题型做题数量（占比）
    /// 3. 孩子的各个题型正确率
    /// 4. 孩子近期ability变化（TODO）
    pub async fn child_statical(
        db: &impl ConnectionTrait,
        child_id: i32,
        resent: ResentType,
    ) -> Result<Vec<ChildStaticalItem>, DbErr> {
        let query = Children::find_by_id(child_id)
            .filter(Condition::all().modify_filter(resent))
            .join(JoinType::Join, children::Relation::AnswerRecord.def())
            .join(JoinType::Join, answer_record::Relation::Quizes.def())
            .join(JoinType::Join, quizes::Relation::QuizGroups.def())
            .group_by(quiz_groups::Column::Gid)
            .add_limit(resent)
            .into_partial_model::<ChildStaticalItem>()
            .all(db)
            .await?;
        Ok(query)
    }
}

#[cfg(test)]
mod test {
    use crate::service::child_statical::ResentType;
    use crate::service::ChildQuizService;
    use sea_orm::{ConnectOptions, Database};

    #[tokio::test]
    async fn test() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let ret = ChildQuizService::child_statical(&conn, 5, ResentType::Quiz(20))
            .await
            .expect("error");

        println!("{:#?}", ret)
    }
}
