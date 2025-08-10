use std::sync::OnceLock;

use pest::iterators::Pairs;

use pest::pratt_parser::PrattParser;

pub mod ui_state;

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    VarX,
    BinOp {
        lhs: Box<Self>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(pest_derive::Parser)]
#[grammar = "calculator.pest"]
pub struct ExprParser;

static PRATT_PARSER: OnceLock<PrattParser<Rule>> = OnceLock::new();

pub fn init_pratt() -> PrattParser<Rule> {
    use Rule::*;
    use pest::pratt_parser::{Assoc::*, Op};

    // Precedence is defined lowest to highest
    PrattParser::new()
        // Addition and subtract have equal precedence
        .op(Op::infix(add, Left) | Op::infix(subtract, Left))
        .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .get_or_init(init_pratt)
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<i64>().unwrap()),
            Rule::var_x => Expr::VarX,
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .parse(pairs)
}

pub fn inorder_eval(expr: &Expr, var_x: i64) -> i64 {
    match expr {
        Expr::Integer(i) => *i,
        Expr::VarX => var_x,
        Expr::BinOp { lhs, op, rhs } => {
            let lhs = inorder_eval(lhs, var_x);
            let rhs = inorder_eval(rhs, var_x);
            match op {
                Op::Add => lhs + rhs,
                Op::Subtract => lhs - rhs,
                Op::Multiply => lhs * rhs,
                Op::Divide => lhs / rhs,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn eval_parse_tree() {
        let input = "2 + 3 * ( 2 + 3)";
        let pairs = ExprParser::parse(Rule::equation, input)
            .unwrap()
            .next()
            .unwrap()
            .into_inner();
        let expr = &parse_expr(pairs);
        assert_eq!(inorder_eval(expr, 1), 17)
    }

    #[test]
    fn eval_with_x() {
        let x = 2;

        let input = "2 + 3 * ( 2 - 3) * x";
        let pairs = ExprParser::parse(Rule::equation, input)
            .unwrap()
            .next()
            .unwrap()
            .into_inner();
        let expr = &parse_expr(pairs);
        assert_eq!(inorder_eval(expr, x), -4)
    }
}
