use std::fs::read_to_string;

use parser::{bag_rule, BagId, BagRule};

mod parser;

fn all_that_contain(rules: &Vec<BagRule>, initial: &BagId) -> Vec<BagId> {
    let mut stack: Vec<BagId> = vec![initial.clone()];
    let mut stack_idx = 0;
    while stack_idx != stack.len() {
        let current_id: BagId = stack.get(stack_idx).unwrap().clone();
        let mut parent_bag_ids: Vec<BagId> = rules
            .iter()
            .filter(|rule| {
                rule.contents
                    .iter()
                    .find(|(_, id)| id == &current_id)
                    .is_some()
            })
            .map(|rule| rule.bag_id.clone())
            .collect();

        stack.append(&mut parent_bag_ids);
        stack_idx += 1;
    }
    stack.sort();
    stack.dedup();
    stack.into_iter().filter(|id| id != initial).collect()
}

fn all_contained_in(rules: &Vec<BagRule>, initial: &BagId) -> Vec<(usize, BagId)> {
    let mut stack: Vec<(usize, BagId)> = vec![(1, initial.clone())];
    let mut stack_idx = 0;
    while stack_idx != stack.len() {
        let current_count = stack.get(stack_idx).unwrap().clone();
        let current_rule = rules
            .iter()
            .find(|&rule| rule.bag_id == current_count.1)
            .unwrap()
            .clone();
        let mut to_append = current_rule
            .contents
            .iter()
            .map(|(n, id)| (*n as usize * current_count.0, id.clone()))
            .collect();
        stack.append(&mut to_append);
        stack_idx += 1;
    }
    stack.into_iter().filter(|(_, id)| id != initial).collect()
}

fn part_1(rules: &Vec<BagRule>) -> usize {
    all_that_contain(&rules, &("shiny", "gold").into()).len()
}

fn part_2(rules: &Vec<BagRule>) -> usize {
    all_contained_in(&rules, &("shiny", "gold").into())
        .iter()
        .map(|&(c, _)| c as usize)
        .sum()
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let rules: Vec<BagRule> = input
        .lines()
        .map(|line| bag_rule(line).unwrap().1)
        .collect();
    println!("Part 1: {}", part_1(&rules));
    println!("Part 2: {}", part_2(&rules));
}
