use itertools::Itertools;
use std::fs::read_to_string;

fn check_if_sums_to(numbers: &Vec<usize>, target_value: usize) -> Option<Vec<usize>> {
    let mut sum = 0;
    let mut set = vec![];
    for num in numbers.iter() {
        sum += num;
        set.push(*num);
        if sum > target_value {
            return None;
        } else if sum == target_value {
            return Some(set);
        }
    }
    return None;
}

fn part_1(numbers: &Vec<usize>, preamble_size: usize) -> usize {
    numbers
        .iter()
        .enumerate()
        .skip(preamble_size)
        .find(|&(idx, &number)| {
            numbers.clone().as_slice()[idx - preamble_size..idx]
                .iter()
                .tuple_combinations::<(_, _)>()
                .all(|(a, b)| a + b != number)
        })
        .map(|(_, &number)| number)
        .unwrap()
}

fn part_2(numbers: &Vec<usize>, preamble_size: usize) -> usize {
    let invalid_number = part_1(&numbers, preamble_size);
    let target_set = numbers
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let rest: Vec<usize> = numbers.clone().as_slice()[idx..(numbers.len())]
                .iter()
                .map(|&a| a)
                .collect();
            check_if_sums_to(&rest, invalid_number)
        })
        .find(|opt| opt.is_some())
        .unwrap()
        .unwrap();
    let smallest_number = target_set.clone().into_iter().min().unwrap();
    let biggest_number = target_set.clone().into_iter().max().unwrap();
    smallest_number + biggest_number
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let numbers: Vec<usize> = input
        .lines()
        .map(|value| value.parse::<usize>().unwrap())
        .collect();
    println!("Part 1: {}", part_1(&numbers, 25));
    println!("Part 2: {}", part_2(&numbers, 25));
}
