use itertools::Itertools;
use std::fs::read_to_string;

fn part_1(input: &Vec<u32>) -> u32 {
    input
        .into_iter()
        .tuple_combinations()
        .filter(|(&a, &b)| a + b == 2020)
        .map(|(&a, &b)| a * b)
        .next()
        .unwrap()
}

fn part_2(input: &Vec<u32>) -> u32 {
    input
        .into_iter()
        .tuple_combinations::<(_, _, _)>()
        .filter(|(&a, &b, &c)| a + b + c == 2020)
        .map(|(&a, &b, &c)| a * b * c)
        .next()
        .unwrap()
}

fn main() {
    let input_lines = read_to_string("./src/input")
        .unwrap()
        .lines()
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    println!("Part 1: {}", part_1(&input_lines));
    println!("Part 2: {}", part_2(&input_lines));
}
