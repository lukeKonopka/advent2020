use crate::ast::{Ast, Op};
use nom::{
    branch::alt,
    character::complete::{char, digit1, space0},
    combinator::{eof, map, map_res},
    error::Error,
    sequence::{delimited, pair, terminated},
    Err, IResult,
};
use std::str::FromStr;

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        map(char('+'), |_| Op::Add),
        map(char('-'), |_| Op::Sub),
        map(char('*'), |_| Op::Mul),
    ))(input)
}

fn parens(i: &str) -> IResult<&str, Ast> {
    delimited(space0, delimited(char('('), operation, char(')')), space0)(i)
}

fn number(i: &str) -> IResult<&str, Ast> {
    alt((
        map(
            map_res(delimited(space0, digit1, space0), FromStr::from_str),
            Ast::Number,
        ),
        parens,
    ))(i)
}

// We read an initial factor and for each time we find
// a * or / operator followed by another factor, we do
// the math by folding everything
fn operation(i: &str) -> IResult<&str, Ast> {
    let (i, init) = number(i)?;

    fold_many0(pair(parse_op, number), init, |acc, (op, val)| {
        Ast::Operation {
            operator: op,
            left: Box::new(acc),
            right: Box::new(val),
        }
    })(i)
}

pub fn parse(input: &str) -> Result<Ast, Err<Error<&str>>> {
    terminated(operation, eof)(input).map(|tuple| tuple.1)
}
pub fn fold_many0<I, O, E, F, G, R>(
    mut f: F,
    init: R,
    mut g: G,
) -> impl FnOnce(I) -> IResult<I, R, E>
where
    I: Clone + PartialEq,
    F: nom::Parser<I, O, E>,
    G: FnMut(R, O) -> R,
    E: nom::error::ParseError<I>,
{
    move |i: I| {
        let mut res = init;
        let mut input = i;

        loop {
            let i_ = input.clone();
            match f.parse(i_) {
                Ok((i, o)) => {
                    // loop trip must always consume (otherwise infinite loops)
                    if i == input {
                        return Err(Err::Error(E::from_error_kind(
                            input,
                            nom::error::ErrorKind::Many0,
                        )));
                    }

                    res = g(res, o);
                    input = i;
                }
                Err(Err::Error(_)) => {
                    return Ok((input, res));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}
