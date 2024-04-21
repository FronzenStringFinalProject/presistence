use crate::entities::answer_record as model;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{DerivePartialModel, FromQueryResult, IntoSimpleExpr};

#[derive(Debug, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "model::Entity")]
pub struct AnsRecordItem {
    pub cid: i32,
    #[sea_orm(from_expr = "array_agg(model::Column::Qid,model::Column::Date)")]
    pub answered_quiz: Vec<i32>,
    #[sea_orm(from_expr = "array_agg(model::Column::Correct,model::Column::Date)")]
    pub answer_results: Vec<bool>,
}
fn array_agg(col: model::Column, order: model::Column) -> SimpleExpr {
    Expr::cust_with_exprs(
        "ARRAY_AGG($1 ORDER BY $2 ASC)",
        [col.into_simple_expr(), order.into_simple_expr()],
    )
}
