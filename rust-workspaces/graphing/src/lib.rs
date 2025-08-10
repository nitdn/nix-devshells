use std::sync::OnceLock;

use pest::Parser;
use pest::iterators::Pairs;

use pest::pratt_parser::PrattParser;

pub mod ui_state;

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
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
            Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            Rule::var_x => Expr::VarX,
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
