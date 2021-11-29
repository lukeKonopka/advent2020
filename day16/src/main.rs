use std::fs::read_to_string;

use parser::{Field, Input};

use crate::parser::{parse, Rule, Ticket};

mod parser;

fn invalid_value(fields: &[Field], ticket: &Ticket) -> Option<usize> {
    let rules = fields
        .iter()
        .flat_map(|f| {
            let rules = f.rules;
            [rules.0, rules.1]
        })
        .collect::<Vec<_>>();
    for value in ticket.field_values.iter() {
        if !rules
            .iter()
            .any(|Rule(from, to)| value > from && value < to)
        {
            return Some(*value as usize);
        }
    }
    None
}

fn part_1(input: &Input) -> usize {
    input
        .nearby_tickets
        .iter()
        .filter_map(|ticket| invalid_value(&input.fields, ticket))
        .sum::<usize>()
}

fn part_2(input: &Input) -> usize {
    let valid_tickets = input
        .nearby_tickets
        .iter()
        .filter(|t| invalid_value(&input.fields, t).is_none())
        .collect::<Vec<_>>();
    let field_names = input
        .fields
        .iter()
        .map(|field| field.name)
        .collect::<Vec<_>>();

    todo!()
}

enum FieldIdentity<'a> {
    PossibleValues(Vec<&'a str>),
    ConcreteValue(Vec<&'a str>),
}

fn main() {
    let input_str = read_to_string("./input").unwrap();
    let parsed = parse(&input_str).unwrap();
    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}
