#[macro_use]
extern crate lazy_static;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"^#([a-f0-9]{6})$").unwrap();
    static ref EYE_COLOR_REGEX: Regex =
        Regex::new(r"^((amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth))$").unwrap();
}

fn passport_valid(entry: &HashMap<&str, Vec<&str>>) -> bool {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_keys.iter().all(|&key| entry.contains_key(key))
}

fn passport_field_valid(key: &str, value: &str) -> bool {
    let validate_year = |v: &str, min, max| {
        v.len() == 4 && v.parse::<usize>().unwrap() >= min && v.parse::<usize>().unwrap() <= max
    };

    let validate_height = |v: &str| -> Option<bool> {
        if !HEIGHT_REGEX.is_match(v) {
            return None;
        }

        let (_, height, unit) = HEIGHT_REGEX
            .captures(v)
            .unwrap()
            .iter()
            .map(|m| m.unwrap().as_str())
            .collect_tuple::<(_, _, _)>()?;
        let height = (height as &str).parse::<usize>().ok()?;
        match unit {
            "cm" => Some(height >= 150 && height <= 193),
            "in" => Some(height >= 59 && height <= 76),
            _ => None,
        }
    };

    match key {
        "byr" => validate_year(value, 1920, 2002),
        "iyr" => validate_year(value, 2010, 2020),
        "eyr" => validate_year(value, 2020, 2030),
        "hgt" => validate_height(value).unwrap_or(false),
        "hcl" => HAIR_COLOR_REGEX.is_match(value),
        "ecl" => EYE_COLOR_REGEX.is_match(value),
        "pid" => value.len() == 9 && value.parse::<usize>().is_ok(),
        _ => false,
    }
}

fn passport_valid_2(entry: &HashMap<&str, Vec<&str>>) -> bool {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_keys.iter().all(|&key| {
        let value = *entry.get(key).and_then(|vals| vals.get(0)).unwrap_or(&"");
        entry.contains_key(key) && passport_field_valid(key, value)
    })
}

fn part_1(entries: &Vec<HashMap<&str, Vec<&str>>>) -> usize {
    entries
        .iter()
        .filter(|&entry| passport_valid(entry))
        .count()
}

fn part_2(entries: &Vec<HashMap<&str, Vec<&str>>>) -> usize {
    entries
        .iter()
        .filter(|&entry| passport_valid_2(entry))
        .count()
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let entries = input
        .split("\n\n")
        .map(|entry| {
            entry
                .split_whitespace()
                .map(|field| field.split(':').collect_tuple::<(_, _)>().unwrap())
                .into_group_map()
        })
        .collect();
    println!("Part 1: {}", part_1(&entries));
    println!("Part 2: {}", part_2(&entries));
}
