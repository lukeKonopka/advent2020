use std::num::ParseIntError;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

pub enum Instruction {
    SetMask(String),
    WriteMem(u64, u64),
}

fn set_mask(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("mask = "), alphanumeric1), |mask| {
        Instruction::SetMask(String::from(mask))
    })(input)
}

fn write_mem(input: &str) -> IResult<&str, Instruction> {
    map_res(
        separated_pair(
            preceded(tag("mem"), delimited(tag("["), digit1, tag("]"))),
            tag(" = "),
            digit1,
        ),
        |(addr, value): (&str, &str)| -> Result<Instruction, ParseIntError> {
            let addr = addr.parse()?;
            let value = value.parse()?;
            Ok(Instruction::WriteMem(addr, value))
        },
    )(input)
}

pub fn instruction(input: &str) -> Result<Instruction, String> {
    alt((set_mask, write_mem))(input)
        .map(|(_, i)| i)
        .map_err(|_| format!("cannot parse {}", input))
}
