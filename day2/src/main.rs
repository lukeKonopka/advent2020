use std::fs::read_to_string;

mod parser;

struct Password {
    policy_letter: char,
    policy_range: (usize, usize),
    value: String,
}

impl Password {
    fn is_valid_sled(&self) -> bool {
        let policy_letter_count = self
            .value
            .chars()
            .filter(|&c| c == self.policy_letter)
            .count();
        let (from, to) = self.policy_range;
        policy_letter_count >= from && policy_letter_count <= to
    }

    fn is_valid_toboggan(&self) -> bool {
        let letters_matched = vec![self.policy_range.0, self.policy_range.1]
            .into_iter()
            .map(|letter_place| self.value.chars().nth(letter_place - 1).unwrap())
            .filter(|&letter| letter == self.policy_letter)
            .count();
        letters_matched == 1
    }
}

impl From<&str> for Password {
    fn from(other: &str) -> Self {
        parser::password_declaration(other).unwrap().1
    }
}

fn part_1(lines: &Vec<&str>) -> usize {
    lines
        .into_iter()
        .map(|&line| Password::from(line))
        .filter(|password| password.is_valid_sled())
        .count()
}

fn part_2(lines: &Vec<&str>) -> usize {
    lines
        .into_iter()
        .map(|&line| Password::from(line))
        .filter(|password| password.is_valid_toboggan())
        .count()
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let lines = input.lines().collect();
    println!("Part 1: {}", part_1(&lines));
    println!("Part 2: {}", part_2(&lines));
}
