use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Ins<'a>(&'a str, i64);

impl<'a> Ins<'a> {
    fn toggle(&mut self) {
        match self.0 {
            "jmp" => self.0 = "nop",
            "nop" => self.0 = "jmp",
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    acc: i64,
    pc: usize,
}

impl Default for Machine {
    fn default() -> Self {
        Machine { acc: 0, pc: 0 }
    }
}

impl Machine {
    fn exec(&mut self, instruction: &Ins) {
        match instruction {
            Ins("nop", _) => {
                self.pc += 1;
            }
            Ins("acc", value) => {
                self.acc += value;
                self.pc += 1;
            }
            Ins("jmp", offset) => {
                self.pc = ((self.pc as i64) + offset) as usize;
            }
            Ins(op, arg) => panic!("Unknown instruction ({}, {})", op, arg),
        }
    }
}

fn exec_to_halt(instruction_list: Vec<Ins>) -> Result<Machine, Machine> {
    let mut visited: Vec<usize> = vec![];
    let mut machine = Machine::default();
    loop {
        if visited.contains(&machine.pc) {
            // loop detected, won't halt
            return Result::Err(machine);
        }
        visited.push(machine.pc);
        match instruction_list.get(machine.pc) {
            Some(ins) => machine.exec(ins),
            // halted
            None => return Result::Ok(machine),
        }
    }
}

fn part_1(instruction_list: &Vec<Ins>) -> i64 {
    exec_to_halt(instruction_list.clone()).unwrap_err().acc
}

fn part_2(instruction_list: &Vec<Ins>) -> i64 {
    instruction_list
        .iter()
        .enumerate()
        .filter(|(_, Ins(op, _))| *op == "jmp" || *op == "nop")
        .map(|(idx_to_change, _)| {
            let mut to_modify = instruction_list.clone();
            to_modify[idx_to_change].toggle();
            to_modify
        })
        .map(exec_to_halt)
        .filter(|machine| machine.is_ok())
        .map(|machine| machine.unwrap().acc)
        .exactly_one()
        .unwrap()
}

fn parse_line(input: &str) -> Ins {
    let (op, arg) = input.split(' ').collect_tuple().unwrap();
    let arg = arg.parse::<i64>().unwrap();
    Ins(op, arg)
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let instruction_list: Vec<Ins> = input.lines().map(|line| parse_line(line)).collect();
    println!("Part 1: {}", part_1(&instruction_list));
    println!("Part 2: {}", part_2(&instruction_list));
}
