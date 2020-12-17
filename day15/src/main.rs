use std::collections::HashMap;

struct MemoryGameIterator {
    history: HashMap<u32, usize>,
    starting_numbers: Vec<u32>,
    current_idx: usize,
    previous_number: Option<u32>,
}

impl MemoryGameIterator {
    fn from_starting(starting_numbers: &Vec<u32>) -> Self {
        Self {
            history: HashMap::new(),
            starting_numbers: starting_numbers.clone(),
            current_idx: 0,
            previous_number: None,
        }
    }
}

impl Iterator for MemoryGameIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next_number = if self.current_idx < self.starting_numbers.len() {
            *self.starting_numbers.get(self.current_idx).unwrap()
        } else {
            match self.previous_number {
                Some(prev) => match self.history.get(&prev) {
                    Some(past_idx) => (self.current_idx - past_idx) as u32,
                    None => 0,
                },
                None => panic!("This will not happen"),
            }
        };
        if let Some(prev) = self.previous_number {
            self.history.insert(prev, self.current_idx);
        }
        self.current_idx += 1;
        self.previous_number = Some(next_number);
        Some(next_number)
    }
}

fn part_1(initial: &Vec<u32>) -> u32 {
    let mut iter = MemoryGameIterator::from_starting(initial);
    iter.nth(2020).unwrap()
}

fn part_2(initial: &Vec<u32>) -> u32 {
    let mut iter = MemoryGameIterator::from_starting(initial);
    iter.nth(30000000).unwrap()
}

fn main() {
    let initial = vec![0, 3, 1, 6, 7, 5];
    println!("Part 1: {}", part_1(&initial));
    println!("Part 2: {}", part_2(&initial));
}
