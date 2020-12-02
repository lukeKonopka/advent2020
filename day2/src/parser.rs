use nom::{bytes, character, combinator::map_res, sequence::tuple, IResult};

use crate::Password;

fn password(input: &str) -> IResult<&str, &str> {
    character::complete::alpha1(input)
}

fn u8_parser(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse::<u8>()
}

fn u8_number(input: &str) -> IResult<&str, u8> {
    map_res(character::complete::digit1, u8_parser)(input)
}

fn policy_range(input: &str) -> IResult<&str, (u8, u8)> {
    let (input, (from, _, to)) = tuple((u8_number, bytes::complete::tag("-"), u8_number))(input)?;
    Ok((input, (from, to)))
}

fn policy(input: &str) -> IResult<&str, ((u8, u8), char)> {
    let (input, (range, _, char)) = tuple((
        policy_range,
        bytes::complete::tag(" "),
        character::complete::anychar,
    ))(input)?;

    Ok((input, (range, char)))
}

pub(crate) fn password_declaration(input: &str) -> IResult<&str, Password> {
    let (input, (policy_val, _, value)) =
        tuple((policy, bytes::complete::tag(": "), password))(input)?;
    let ((from, to), policy_letter) = policy_val;

    Ok((
        input,
        Password {
            value: String::from(value),
            policy_range: (from as usize, to as usize),
            policy_letter,
        },
    ))
}
