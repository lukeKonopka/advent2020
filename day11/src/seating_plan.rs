#[derive(Clone, Copy, PartialEq)]
pub enum Node {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        use Node::*;
        match value {
            '.' => Floor,
            'L' => EmptySeat,
            '#' => OccupiedSeat,
            _ => panic!("Unknown character: /{}/", value),
        }
    }
}
impl Node {
    fn evolve(&self, occupied: usize, limit: usize) -> Self {
        match self {
            Node::Floor => Node::Floor,
            seat => match occupied {
                0 => Node::OccupiedSeat,
                n if n >= limit => Node::EmptySeat,
                _ => *seat,
            },
        }
    }

    fn is_occupied(&self) -> bool {
        self == &Node::OccupiedSeat
    }
}

#[derive(Clone, PartialEq)]
pub struct SeatingPlan(Vec<Vec<Node>>);

impl SeatingPlan {
    pub fn from_input(input: String) -> Self {
        let seating_plan = input
            .lines()
            .map(|line| line.chars().map(|ch| ch.into()).collect())
            .collect();
        SeatingPlan(seating_plan)
    }

    fn all_vision_lines(&self, coord: (usize, usize)) -> Vec<Vec<Node>> {
        let directions = vec![
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];
        directions
            .iter()
            .map(move |direction| {
                self.direction_iter((coord.0, coord.1), *direction)
                    .copied()
                    .collect()
            })
            .collect()
    }

    fn evolve<F>(&self, vision_fn: F, limit: usize) -> Self
    where
        F: Fn(&Vec<Node>) -> Node,
    {
        let seating_plan = self
            .0
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_idx, node)| {
                        let adjecent: Vec<Node> = self
                            .all_vision_lines((row_idx, col_idx))
                            .iter()
                            .map(|line| vision_fn(line))
                            .collect();
                        let occupied_count = adjecent.into_iter().filter(Node::is_occupied).count();
                        node.evolve(occupied_count, limit)
                    })
                    .collect()
            })
            .collect();
        SeatingPlan(seating_plan)
    }

    pub fn evolve_until_stable<F>(&self, vision_fn: &F, limit: usize) -> Self
    where
        F: Fn(&Vec<Node>) -> Node,
    {
        let mut last_plan = self.clone();
        let mut current_plan = last_plan.evolve(vision_fn, limit);
        while last_plan != current_plan {
            last_plan = current_plan;
            current_plan = last_plan.evolve(vision_fn, limit);
        }
        current_plan
    }

    pub fn count_all_occupied(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|&n| n.is_occupied()).count())
            .sum()
    }

    fn direction_iter(
        &self,
        coord: (usize, usize),
        direction: (i32, i32),
    ) -> DirectionIterator<'_> {
        DirectionIterator {
            plan: self,
            current_coord: coord,
            direction,
        }
    }
}

struct DirectionIterator<'a> {
    plan: &'a SeatingPlan,
    current_coord: (usize, usize),
    direction: (i32, i32),
}

impl<'a> Iterator for DirectionIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let (delta_col, delta_row) = self.direction;
        self.current_coord = (
            (self.current_coord.0 as i32 + delta_row) as usize,
            (self.current_coord.1 as i32 + delta_col) as usize,
        );
        self.plan
            .0
            .get(self.current_coord.0)
            .and_then(|row| row.get(self.current_coord.1))
    }
}
