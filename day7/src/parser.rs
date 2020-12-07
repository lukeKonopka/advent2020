use std::fmt::Debug;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::space0,
    character::complete::{digit1, space1},
    combinator::map,
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, terminated},
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(PartialEq, Eq, Clone, Hash, Ord, PartialOrd)]
pub struct BagId(pub String, pub String);

impl Debug for BagId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

impl From<(&str, &str)> for BagId {
    fn from((kind, color): (&str, &str)) -> Self {
        BagId(kind.into(), color.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BagRule {
    pub bag_id: BagId,
    pub contents: Vec<(u8, BagId)>,
}

fn u8_parser(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse::<u8>()
}

fn u8_number(input: &str) -> IResult<&str, u8> {
    map_res(digit1, u8_parser)(input)
}

fn bag_id(input: &str) -> IResult<&str, BagId> {
    map(
        separated_pair(alpha1, space1, alpha1),
        |(kind, color): (&str, &str)| -> BagId { BagId(kind.into(), color.into()) },
    )(input)
}

fn bag_amount(input: &str) -> IResult<&str, (u8, BagId)> {
    map(
        tuple((
            u8_number,
            delimited(space0, bag_id, space0),
            alt((tag("bags"), tag("bag"))),
        )),
        |(amount, bag_id, _)| (amount, bag_id),
    )(input)
}

fn bag_empty(input: &str) -> IResult<&str, Vec<(u8, BagId)>> {
    map(tag("no other bags"), |_| vec![])(input)
}

fn bag_contents(input: &str) -> IResult<&str, Vec<(u8, BagId)>> {
    alt((separated_list1(tag(", "), bag_amount), bag_empty))(input)
}

pub fn bag_rule(input: &str) -> IResult<&str, BagRule> {
    map(
        terminated(
            separated_pair(
                bag_id,
                alt((tag(" bags contain "), tag(" bag contain "))),
                bag_contents,
            ),
            tag("."),
        ),
        |(bag_id, contents)| BagRule { bag_id, contents },
    )(input)
}

#[test]
fn test_parser() {
    assert_eq!(
        bag_rule("dark orange bags contain 3 dark chartreuse bags.")
            .unwrap()
            .1,
        BagRule {
            bag_id: ("dark", "orange").into(),
            contents: vec![(3, ("dark", "chartreuse").into())]
        },
    );
}
