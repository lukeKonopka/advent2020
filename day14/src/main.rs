use std::{collections::HashMap, fs::read_to_string};

use parser::{instruction, Instruction};

mod parser;

#[derive(Default)]
struct Memory {
    state: HashMap<u64, u64>,
    write_mask: Mask,
}

struct Mask {
    bits: [Option<u8>; 36],
}

impl Default for Mask {
    fn default() -> Self {
        Mask { bits: [None; 36] }
    }
}

impl From<&str> for Mask {
    fn from(value: &str) -> Self {
        let mut bits = [None; 36];
        let bits_vec = value.chars().map(|c| match c {
            '0' | '1' => Some(format!("{}", c).parse::<u8>().unwrap()),
            'X' => None,
            _ => panic!("Unknown character {}", c),
        });
        for (idx, b) in bits_vec.enumerate() {
            bits[idx] = b;
        }
        Mask { bits }
    }
}

impl Mask {
    fn apply_to_value(&self, input: u64) -> u64 {
        let mut output: u64 = 0;
        for (bit_idx, &mask_bit) in self.bits.iter().rev().enumerate() {
            let output_bit = match mask_bit {
                Some(v) => v as u64,
                None => (input >> bit_idx) & 1,
            };
            output |= output_bit << bit_idx;
        }
        output
    }

    fn apply_known_to_addr(&self, addr: u64) -> Vec<Option<u8>> {
        self.bits
            .iter()
            .enumerate()
            .map(|(idx, bit)| {
                bit.map(|v| match v {
                    0 => {
                        let offset = 35 - idx;
                        let shifted = addr >> offset;
                        (shifted & 1) as u8
                    }
                    1 => 1,
                    _ => panic!("Invalid bit {}", v),
                })
            })
            .collect()
    }

    fn apply_to_addr(&self, addr: u64) -> impl Iterator<Item = u64> {
        // mask known bits
        let masked = self.apply_known_to_addr(addr);

        // updates vec at specific address (immutable)
        let update_with = |bits: &Vec<Option<u8>>, idx: usize, val: u8| {
            bits.iter()
                .enumerate()
                .map(|(i, &v)| if i == idx { Some(val) } else { v })
                .collect()
        };

        // resolve floating bits
        let non_floating: Vec<Vec<u8>> = masked
            .iter()
            .enumerate()
            .fold(
                vec![masked.clone()] as Vec<Vec<Option<u8>>>,
                |acc, (idx, bit)| match &bit {
                    Some(_) => acc,
                    None => acc
                        .iter()
                        .flat_map(|bits| vec![update_with(bits, idx, 0), update_with(bits, idx, 1)])
                        .collect(),
                },
            )
            .iter()
            .map(|v| v.iter().map(|b| b.unwrap()).collect())
            .collect();
        // construct u64 address from bits
        non_floating.into_iter().map(move |bits| {
            let mut output: u64 = 0;
            for (bit_idx, &bit) in bits.iter().rev().enumerate() {
                output |= (bit as u64) << bit_idx;
            }
            output
        })
    }
}

impl Memory {
    fn write_mem(&mut self, addr: u64, value: u64) {
        self.state.insert(addr, value);
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            SetMask(mask) => self.write_mask = mask.as_str().into(),
            &WriteMem(addr, value) => {
                let masked = self.write_mask.apply_to_value(value);
                self.write_mem(addr, masked);
            }
        }
    }

    fn apply_instruction_v2(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            SetMask(mask) => self.write_mask = mask.as_str().into(),
            &WriteMem(addr, value) => {
                for masked_addr in self.write_mask.apply_to_addr(addr as u64) {
                    self.write_mem(masked_addr, value);
                }
            }
        }
    }
}

fn part_2(instructions: &Vec<Instruction>) -> u64 {
    let mut memory = Memory::default();
    for instr in instructions.iter() {
        memory.apply_instruction_v2(instr);
    }
    memory.state.values().sum()
}

fn part_1(instructions: &Vec<Instruction>) -> u64 {
    let mut memory = Memory::default();
    for instr in instructions.iter() {
        memory.apply_instruction(instr);
    }
    memory.state.values().sum()
}

fn main() {
    let input = read_to_string("./src/input").unwrap();
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| instruction(line).expect("valid instruction"))
        .collect();
    println!("Part 1: {}", part_1(&instructions));
    println!("Part 2: {}", part_2(&instructions));
}
