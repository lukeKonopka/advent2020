use std::fs::read_to_string;

fn slope(
    map: &Vec<Vec<char>>,
    (slope_right, slope_down): (usize, usize),
) -> impl Iterator<Item = char> + '_ {
    map.iter()
        .step_by(slope_down)
        .map(move |row| row.iter().cycle().step_by(slope_right))
        .enumerate()
        .map(|(row_idx, row_iter)| *row_iter.skip(row_idx).next().unwrap())
        .skip(1)
}

fn part_1(input: &Vec<Vec<char>>) -> usize {
    slope(input, (3, 1)).filter(|&field| field == '#').count()
}

fn part_2(input: &Vec<Vec<char>>) -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|slope_val| {
            slope(input, slope_val)
                .filter(|&field| field == '#')
                .count()
        })
        .fold(1, |a, b| a * b)
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let tree_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("Part 1: {}", part_1(&tree_map));
    println!("Part 2: {}", part_2(&tree_map));
}
