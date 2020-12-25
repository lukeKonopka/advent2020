use arrayvec::ArrayVec;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    hash::Hash,
};

#[derive(Debug)]
struct PocketDim<Coord>(HashSet<Coord>);
trait Coord: Eq + Hash + Sized {
    type Neighbors: IntoIterator<Item = Self>;
    fn from_xy(x: i32, y: i32) -> Self;

    fn neighbors(&self) -> Self::Neighbors;
}

impl Coord for (i32, i32, i32) {
    fn from_xy(x: i32, y: i32) -> Self {
        (x, y, 0)
    }

    type Neighbors = ArrayVec<[Self; 26]>;

    fn neighbors(&self) -> Self::Neighbors {
        (-1..=1)
            .flat_map(|x| {
                (-1..=1).flat_map(move |y| {
                    (-1..=1).filter_map(move |z| {
                        if x == 0 && y == 0 && z == 0 {
                            None
                        } else {
                            Some((x + self.0, y + self.1, z + self.2))
                        }
                    })
                })
            })
            .collect()
    }
}

impl Coord for (i32, i32, i32, i32) {
    fn from_xy(x: i32, y: i32) -> Self {
        (x, y, 0, 0)
    }

    type Neighbors = ArrayVec<[Self; 80]>;

    fn neighbors(&self) -> Self::Neighbors {
        (-1..=1)
            .flat_map(|x| {
                (-1..=1).flat_map(move |y| {
                    (-1..=1).flat_map(move |z| {
                        (-1..=1).filter_map(move |w| {
                            if x == 0 && y == 0 && z == 0 && w == 0 {
                                None
                            } else {
                                Some((x + self.0, y + self.1, z + self.2, w + self.3))
                            }
                        })
                    })
                })
            })
            .collect()
    }
}

impl<C: Coord> PocketDim<C> {
    fn form_initial(initial_layer: &str) -> Self {
        let mut map = HashSet::<C>::new();
        for (y, line) in initial_layer.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    map.insert(C::from_xy(x as i32, y as i32));
                }
            }
        }
        Self(map)
    }

    fn cycle(&mut self) {
        let mut neighbor_count_map: HashMap<C, u32> = HashMap::new();
        for coord in &self.0 {
            for n_coord in coord.neighbors() {
                neighbor_count_map
                    .entry(n_coord)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
        }

        let mut new_set = HashSet::new();

        for (coord, neighbor_count) in neighbor_count_map.into_iter() {
            if neighbor_count == 3 || (neighbor_count == 2 && self.0.contains(&coord)) {
                new_set.insert(coord);
            }
        }

        self.0 = new_set;
    }
}

fn part_1(mut state: PocketDim<(i32, i32, i32)>) -> usize {
    for _ in 0..6 {
        state.cycle();
    }
    state.0.len()
}

fn part_2(mut state: PocketDim<(i32, i32, i32, i32)>) -> usize {
    for _ in 0..6 {
        state.cycle();
    }
    state.0.len()
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    println!("Part 1: {}", part_1(PocketDim::form_initial(&input)));
    println!("Part 2: {}", part_2(PocketDim::form_initial(&input)));
}
