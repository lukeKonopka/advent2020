use std::fs::read_to_string;

struct Ship {
    position: (i32, i32),
    waypoint_offset: (i32, i32),
}

impl Ship {
    fn with_waypoint(waypoint_offset: (i32, i32)) -> Self {
        Ship {
            position: (0, 0),
            waypoint_offset,
        }
    }

    fn rotate(&mut self, angle: i32) {
        let angle_radians = (angle as f32).to_radians();
        let (x, y) = self.waypoint_offset;
        let (x, y) = (x as f32, y as f32);
        let rotated_x = x * angle_radians.cos() + y * angle_radians.sin();
        let rotated_y = -x * angle_radians.sin() + y * angle_radians.cos();
        self.waypoint_offset.0 = rotated_x.round() as i32;
        self.waypoint_offset.1 = rotated_y.round() as i32;
    }

    fn go_correct(&mut self, instruction: &(char, usize)) {
        match *instruction {
            ('N', v) => self.waypoint_offset.1 -= v as i32,
            ('S', v) => self.waypoint_offset.1 += v as i32,
            ('E', v) => self.waypoint_offset.0 += v as i32,
            ('W', v) => self.waypoint_offset.0 -= v as i32,
            ('L', v) => self.rotate(v as i32),
            ('R', v) => self.rotate(-(v as i32)),
            ('F', v) => {
                self.position.0 += self.waypoint_offset.0 * v as i32;
                self.position.1 += self.waypoint_offset.1 * v as i32;
            }
            _ => panic!("Unknown instruction: {:?}", instruction),
        }
    }

    fn go_wrong(&mut self, instruction: &(char, usize)) {
        match *instruction {
            ('N', v) => self.position.1 -= v as i32,
            ('S', v) => self.position.1 += v as i32,
            ('E', v) => self.position.0 += v as i32,
            ('W', v) => self.position.0 -= v as i32,
            _ => self.go_correct(instruction),
        }
    }
}

fn part_1(instructions: &Vec<(char, usize)>) -> usize {
    let mut ship = Ship::with_waypoint((1, 0));
    for instr in instructions.iter() {
        ship.go_wrong(instr);
    }
    (ship.position.0.abs() + ship.position.1.abs()) as usize
}

fn part_2(instructions: &Vec<(char, usize)>) -> usize {
    let mut ship = Ship::with_waypoint((10, -1));
    for instr in instructions.iter() {
        ship.go_correct(instr);
    }
    (ship.position.0.abs() + ship.position.1.abs()) as usize
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let instructions: Vec<(char, usize)> = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let cmd = chars.next().unwrap();
            let arg = chars.collect::<String>().parse::<usize>().unwrap();
            (cmd, arg)
        })
        .collect();
    println!("Part 1: {}", part_1(&instructions));
    println!("Part 2: {}", part_2(&instructions));
}
