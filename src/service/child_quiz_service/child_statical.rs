use crate::entities::prelude::{AnswerRecord, Children, QuizGroups};
use crate::entities::{answer_record, children, quiz_groups, quizes};
use crate::service::DatabaseServiceTrait;
use sea_orm::sea_query::{Asterisk, Expr};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, DerivePartialModel, EntityTrait,
    FromQueryResult, JoinType, QueryFilter, QuerySelect, RelationTrait, Select, Value,
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
                    // pg date 操作 https://www.postgresql.org/docs/current/functions-datetime.html
                    // pg 将timestamp 转换为 data https://postgresql-tutorial.com/postgresql-how-to-convert-timestamp-to-date/#:~:text=You%20can%20convert%20a%20timestamp%20to%20a%20date,to%20a%20date%3A%20SELECT%20DATE%20%28order_ts%29%20FROM%20orders%3B
                    .gte(Expr::cust_with_expr(
                        "DATE(NOW())-$1",
                        Expr::val(Value::Int(Some(day as i32))),
                    )),
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

#[derive(Debug, Serialize, FromQueryResult, DerivePartialModel)]
/// TODO: DerivePartialModel : better entity support  
#[sea_orm(entity = "QuizGroups")]
pub struct ChildQuizGroupStaticalItem {
    #[sea_orm(from_col = "gid")]
    pub quiz_ty_id: i32,
    #[sea_orm(from_col = "name")]
    pub quiz_ty: String,
    #[sea_orm(from_expr = "Expr::col(Asterisk).count()")]
    pub total: i64,
    // pgsql 只统计true行 https://dba.stackexchange.com/questions/205012/how-to-count-boolean-values-in-postgresql
    #[sea_orm(from_expr = "Expr::cust_with_exprs(\"$1 FILTER (WHERE $2)\",\
    [Expr::col(Asterisk).count(),answer_record::Column::Correct.eq(true)])")]
    pub correct: i64,
    #[sea_orm(from_expr = "Expr::cust_with_exprs(\"$1 FILTER (WHERE $2)\",\
    [Expr::col(Asterisk).count(),answer_record::Column::Correct.eq(false)])")]
    pub wrong: i64,
    // pgsql cast 函数 https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-cast/
    // pgsql count 函数 https://www.postgresqltutorial.com/postgresql-aggregate-functions/postgresql-count-function/
    #[sea_orm(from_expr = "Expr::cust_with_exprs(\
        \"CAST($1 FILTER (WHERE $2) AS DOUBLE PRECISION)/CAST($1 AS DOUBLE PRECISION)\",\
        [Expr::col(Asterisk).count(),answer_record::Column::Correct.eq(true)])")]
    pub correct_rate: f64,
}
#[derive(Debug, Serialize, FromQueryResult, DerivePartialModel)]
/// TODO: DerivePartialModel : better entity support
#[sea_orm(entity = "AnswerRecord")]
pub struct ChildResentCorrectStaticalItem {
    #[sea_orm(from_col = "date")]
    pub date: chrono::NaiveDate,
    #[sea_orm(from_expr = "Expr::col(Asterisk).count()")]
    pub total: i64,
    // pgsql 只统计true行 https://dba.stackexchange.com/questions/205012/how-to-count-boolean-values-in-postgresql
    #[sea_orm(from_expr = "Expr::cust_with_exprs(\"$1 FILTER (WHERE $2)\",\
    [Expr::col(Asterisk).count(),answer_record::Column::Correct.eq(true)])")]
    pub correct: i64,
    #[sea_orm(from_expr = "Expr::cust_with_exprs(\"$1 FILTER (WHERE $2)\",\
    [Expr::col(Asterisk).count(),answer_record::Column::Correct.eq(false)])")]
    pub wrong: i64,
    // pgsql cast 函数 https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-cast/
    // pgsql count 函数 https://www.postgresqltutorial.com/postgresql-aggregate-functions/postgresql-count-function/
    #[sea_orm(from_expr = "Expr::cust_with_exprs(\
        \"CAST($1 FILTER (WHERE $2) AS DOUBLE PRECISION)/CAST($1 AS DOUBLE PRECISION)\",\
        [Expr::col(Asterisk).count(),answer_record::Column::Correct.eq(true)])")]
    pub correct_rate: f64,
}
impl<D> super::ChildQuizService<D>
where
    D: ConnectionTrait,
{
    /// 孩子统计信息
    ///
    /// 统计内容
    /// 1. 孩子最近的正确率
    /// 2. 孩子各个题型做题数量（占比）
    /// 3. 孩子的各个题型正确率
    /// 4. 孩子近期ability变化（TODO）
    pub async fn child_quiz_group_statical(
        &self,
        child_id: i32,
        resent: ResentType,
    ) -> Result<Vec<ChildQuizGroupStaticalItem>, DbErr> {
        let query = Children::find_by_id(child_id)
            .filter(Condition::all().modify_filter(resent))
            .join(JoinType::Join, children::Relation::AnswerRecord.def())
            .join(JoinType::Join, answer_record::Relation::Quizes.def())
            .join(JoinType::Join, quizes::Relation::QuizGroups.def())
            .group_by(quiz_groups::Column::Gid)
            .add_limit(resent)
            .into_partial_model::<ChildQuizGroupStaticalItem>()
            .all(self.db())
            .await?;
        Ok(query)
    }

    pub async fn child_resent_correct_statical(
        &self,
        child_id: i32,
        resent: ResentType,
    ) -> Result<Vec<ChildResentCorrectStaticalItem>, DbErr> {
        let query = AnswerRecord::find()
            .filter(
                Condition::all()
                    .add(answer_record::Column::Cid.eq(child_id))
                    .modify_filter(resent),
            )
            .group_by(answer_record::Column::Date)
            .add_limit(resent)
            .into_partial_model::<ChildResentCorrectStaticalItem>()
            .all(self.db())
            .await?;
        Ok(query)
    }
}

#[cfg(test)]
mod test {
    use crate::service::child_quiz_service::child_statical::ResentType;
    use crate::service::{ChildQuizService, DatabaseServiceTrait};
    use sea_orm::{ConnectOptions, Database};

    #[tokio::test]
    async fn test() {
        let conn = Database::connect(ConnectOptions::new(
            "postgres://JACKY:wyq020222@localhost/quiz-evaluate",
        ))
        .await
        .expect("cannot connect Db");

        let ret = ChildQuizService::with_db(conn)
            .child_quiz_group_statical(5, ResentType::Days(100))
            .await
            .expect("error");

        println!("{:#?}", ret)
    }
}
