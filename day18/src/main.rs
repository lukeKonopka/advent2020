use std::fs::read_to_string;

use ast::Ast;
use nom::{error::Error, Err};

mod ast;
mod parser_advanced;
mod parser_basic;

fn eval_and_sum<'a>(
    expressions: &[&str],
    mut parser: impl FnMut(&str) -> Result<Ast, Err<Error<&str>>>,
) -> i64 {
    expressions
        .iter()
        .map(move |expr| -> Ast { parser(expr).unwrap() })
        .map(|ast| ast.eval())
        .sum()
}

fn part_1(expressions: &[&str]) -> i64 {
    eval_and_sum(expressions, parser_basic::parse)
}

fn part_2(expressions: &[&str]) -> i64 {
    eval_and_sum(expressions, parser_advanced::parse)
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    println!("Part 1: {}", part_1(&lines));
    println!("Part 2: {}", part_2(&lines));
}
