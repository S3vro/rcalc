use crate::parser::{Expr, UnaryOp, BinaryOp};

pub fn evaluate(root_expr: Expr) -> i32 {
    match root_expr {
        Expr::Int(n) => n as i32,
        Expr::Unary(op, expr) => evalute_unary(op, *expr),
        Expr::Binary(op, lhs, rhs) => evaluate_binary(op, *lhs, *rhs),
    }
}

fn evalute_unary(op: UnaryOp, expr: Expr) -> i32 {
    match op {
        UnaryOp::Neg => - evaluate(expr),
    }
}

fn evaluate_binary(op: BinaryOp, lhs: Expr, rhs: Expr) -> i32 {
    match op {
        BinaryOp::Add => evaluate(lhs) + evaluate(rhs),
        BinaryOp::Sub => evaluate(lhs) - evaluate(rhs),
        BinaryOp::Mul => evaluate(lhs) * evaluate(rhs),
        BinaryOp::Div => evaluate(lhs) / evaluate(rhs),
    }
}
