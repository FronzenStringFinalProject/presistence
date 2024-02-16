use sea_orm::sea_query::{Expr, SimpleExpr};

use crate::entities::quizes;

pub fn predict_correct_expr(ability: Expr) -> SimpleExpr {
    Expr::col(quizes::Column::Lambda).add(
        Expr::expr(Expr::val(1).sub(Expr::col(quizes::Column::Lambda))).div(
            Expr::val(1).add(Expr::cust_with_expr(
                "exp($1)",
                Expr::val(-1.702)
                    .mul(Expr::col(quizes::Column::Disc))
                    .mul(ability.sub(Expr::col(quizes::Column::Diff))),
            )),
        ),
    )
}
