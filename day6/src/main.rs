use std::fs::read_to_string;

#[derive(Debug)]
struct Group {
    people: Vec<Vec<char>>,
}

impl From<&str> for Group {
    fn from(input: &str) -> Self {
        let people = input.lines().map(|line| line.chars().collect()).collect();
        Group { people }
    }
}

impl Group {
    fn uniq_questions(&self) -> impl Iterator<Item = &char> {
        let mut all_questions: Vec<&char> = self.people.iter().flatten().collect();
        all_questions.sort();
        all_questions.dedup();
        all_questions.into_iter()
    }

    fn every_questions(&self) -> impl Iterator<Item = &char> + '_ {
        self.uniq_questions()
            .filter(move |q| self.people.iter().all(|person| person.contains(q)))
    }
}

fn part_1(groups: &Vec<Group>) -> usize {
    groups
        .iter()
        .map(|group| group.uniq_questions().count())
        .sum()
}

fn part_2(groups: &Vec<Group>) -> usize {
    groups
        .iter()
        .map(|group| group.every_questions().count())
        .sum()
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let groups = input
        .split("\n\n")
        .map(|group_str| group_str.into())
        .collect();
    println!("Part 1: {}", part_1(&groups));
    println!("Part 2: {}", part_2(&groups));
}
