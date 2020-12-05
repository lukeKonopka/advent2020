use itertools::Itertools;
use std::fs::read_to_string;

fn split_range(range: (u8, u8)) -> ((u8, u8), (u8, u8)) {
    let midpoint = (range.1 - range.0) / 2 + range.0;
    ((range.0, midpoint), (midpoint + 1, range.1))
}

fn take_range(range: (u8, u8), half_char: char) -> (u8, u8) {
    let (lower, upper) = split_range(range);
    match half_char {
        'F' | 'L' => lower,
        'B' | 'R' => upper,
        _ => panic!("unknown char {}", half_char),
    }
}

fn path_to_seat_id(path: &str) -> usize {
    let (row_path, column_path) = path.split_at(path.len() - 3);
    let (row, _) = row_path
        .chars()
        .fold((0, 127), |range, c| take_range(range, c));
    let (column, _) = column_path
        .chars()
        .fold((0, 7), |range, c| take_range(range, c));
    row as usize * 8 + column as usize
}

fn part_1(paths: &Vec<&str>) -> usize {
    paths
        .iter()
        .map(|&path| path_to_seat_id(path))
        .max()
        .expect("there is max seat_id")
}

fn part_2(paths: &Vec<&str>) -> usize {
    let mut seat_ids: Vec<usize> = paths.iter().map(|&path| path_to_seat_id(path)).collect();
    seat_ids.sort();
    let (left_seat, _) = seat_ids
        .iter()
        .tuple_windows()
        .find(|(&a, &b)| a + 2 == b)
        .expect("two seats with one space between them");
    left_seat + 1
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let paths = input.lines().collect();
    println!("Part 1: {}", part_1(&paths));
    println!("Part 2: {}", part_2(&paths));
}

#[test]
fn split_range_test() {
    assert_eq!(split_range((0, 127)), ((0, 63), (64, 127)));
    assert_eq!(split_range((0, 63)), ((0, 31), (32, 63)));
    assert_eq!(split_range((32, 63)), ((32, 47), (48, 63)));
    assert_eq!(split_range((32, 47)), ((32, 39), (40, 47)));
    assert_eq!(split_range((40, 47)), ((40, 43), (44, 47)));
    assert_eq!(split_range((44, 47)), ((44, 45), (46, 47)));
    assert_eq!(split_range((44, 45)), ((44, 44), (45, 45)));
}

#[test]
fn path_to_seat_test() {
    assert_eq!(path_to_seat_id("BFFFBBFRRR"), 567);
    assert_eq!(path_to_seat_id("FFFBBBFRRR"), 119);
    assert_eq!(path_to_seat_id("BBFFBBFRLL"), 820);
}
