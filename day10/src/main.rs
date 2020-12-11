use itertools::Itertools;
use std::{fs::read_to_string, iter::once};

fn get_gaps(adapters: &Vec<usize>) -> Vec<usize> {
    let max_adapter = adapters.iter().max().unwrap();
    let device = max_adapter + 3 as usize;
    let with_socket_and_device = once(&0).chain(adapters.into_iter().chain(once(&device)));
    with_socket_and_device
        .sorted()
        .tuple_windows()
        .map(|(&a, &b)| b - a)
        .collect()
}

fn possible_combinations_count(gaps: &Vec<usize>) -> usize {
    match gaps.as_slice() {
        [1, 1] => 2,
        [1, 1, 1] => 4,
        [1, 1, 1, 1] => 7,
        _ => 1,
    }
}

fn part_1(adapters: &Vec<usize>) -> usize {
    let gaps = get_gaps(adapters);
    let gap_one_count = gaps.iter().copied().filter(|&d| d == 1).count();
    let gap_three_count = gaps.iter().copied().filter(|&d| d == 3).count();
    gap_one_count * gap_three_count
}

fn part_2(adapters: &Vec<usize>) -> usize {
    let gaps = get_gaps(adapters);
    gaps.iter()
        .group_by(|&v| v)
        .into_iter()
        .map(|(_, gaps)| possible_combinations_count(&gaps.into_iter().copied().collect()))
        .product()
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let adapters: Vec<usize> = input
        .lines()
        .map(|adap| adap.parse::<usize>().unwrap())
        .collect();
    println!("Part 1: {}", part_1(&adapters));
    println!("Part 2: {}", part_2(&adapters));
}
