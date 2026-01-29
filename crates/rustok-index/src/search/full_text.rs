use sea_orm::sea_query::{Expr, SimpleExpr};

pub fn tsvector_from_fields(fields: &[(&str, &str)]) -> SimpleExpr {
    let mut expr = None;

    for (weight, field) in fields {
        let weighted = Expr::cust_with_exprs(
            "setweight(to_tsvector('simple', COALESCE(?, '')), ?)",
            [Expr::cust(*field), Expr::cust(*weight)],
        );

        expr = Some(match expr {
            Some(current) => Expr::cust_with_exprs("? || ?", [current, weighted]),
            None => weighted,
        });
    }

    expr.unwrap_or_else(|| Expr::cust("to_tsvector('simple', '')"))
}
