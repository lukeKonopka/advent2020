use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Field {
    Tree,
    Empty,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Tree,
            '.' => Self::Empty,
            _ => panic!("Unknown char {}", value),
        }
    }
}

struct TreeMap {
    map: Vec<Vec<Field>>,
}

impl TreeMap {
    fn from_file(path: &str) -> Self {
        let input = read_to_string(path).unwrap();
        Self::from_str(&input)
    }

    fn from_str(input: &str) -> Self {
        let map: Vec<Vec<Field>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        TreeMap { map }
    }

    fn slope(&self, (slope_right, slope_down): (usize, usize)) -> impl Iterator<Item = Field> + '_ {
        self.map
            .iter()
            .enumerate()
            .step_by(slope_down)
            .map(move |(row_idx, row)| {
                let slope = slope_right as f32 / slope_down as f32;
                let column_idx = (row_idx as f32 * slope) as usize;
                *row.into_iter().cycle().skip(column_idx).next().unwrap()
            })
            .skip(1)
    }
}

fn part_1(input: &TreeMap) -> usize {
    input
        .slope((3, 1))
        .filter(|&field| field == Field::Tree)
        .count()
}

fn part_2(input: &TreeMap) -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|slope_val| {
            input
                .clone()
                .slope(slope_val)
                .filter(|&field| field == Field::Tree)
                .count()
        })
        .fold(1, |a, b| a * b)
}

fn main() {
    let tree_map = TreeMap::from_file("./src/input");
    println!("Part 1: {}", part_1(&tree_map));
    println!("Part 2: {}", part_2(&tree_map));
}
