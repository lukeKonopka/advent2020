use std::fs::read_to_string;

use seating_plan::{Node, SeatingPlan};

mod seating_plan;

fn short_sight(vision_line: &Vec<Node>) -> Node {
    *vision_line.first().unwrap_or(&Node::Floor)
}

fn perfect_vision(vision_line: &Vec<Node>) -> Node {
    *vision_line
        .iter()
        .skip_while(|&node| *node == Node::Floor)
        .next()
        .unwrap_or(&Node::Floor)
}

fn part_1(plan: &SeatingPlan) -> usize {
    plan.evolve_until_stable(&short_sight, 4)
        .count_all_occupied()
}

fn part_2(plan: &SeatingPlan) -> usize {
    plan.evolve_until_stable(&perfect_vision, 5)
        .count_all_occupied()
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let initial_plan = SeatingPlan::from_input(input);
    println!("Part 1: {}", part_1(&initial_plan));
    println!("Part 2: {}", part_2(&initial_plan));
}
