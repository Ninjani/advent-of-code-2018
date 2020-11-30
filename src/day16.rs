use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Instruction {
    opcode: u8,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        let parts: Vec<_> = line
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        Instruction {
            opcode: parts[0] as u8,
            a: parts[1],
            b: parts[2],
            c: parts[3],
        }
    }
}

pub struct Sample {
    registers_before: [usize; 4],
    instruction: Instruction,
    registers_after: [usize; 4],
}

impl Sample {
    pub fn new(part: &str) -> Self {
        let lines: Vec<_> = part.split('\n').collect();
        let registers_re = Regex::new(r".+\[([0-9]+), ([0-9]+), ([0-9]+), ([0-9]+)]").unwrap();
        let re_match = registers_re.captures(lines[0]).unwrap();
        let mut registers_before = [0; 4];
        for i in 0..4 {
            registers_before[i] = re_match
                .get(i + 1)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        }

        let instruction = Instruction::new(lines[1]);

        let re_match = registers_re.captures(lines[2]).unwrap();
        let mut registers_after = [0; 4];
        for i in 0..4 {
            registers_after[i] = re_match
                .get(i + 1)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        }

        Sample {
            registers_before,
            instruction,
            registers_after,
        }
    }
}

fn get_samples(lines: &str) -> Vec<Sample> {
    lines.split("\n\n").map(|part| Sample::new(part)).collect()
}
fn get_test_program(lines: &str) -> Vec<Instruction> {
    lines
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| Instruction::new(line))
        .collect()
}

#[aoc_generator(day16, part1)]
pub fn generate_day16_part1(input: &str) -> Vec<Sample> {
    let lines = input.split("\n\n\n").next().unwrap();
    get_samples(lines)
}

#[aoc_generator(day16, part2)]
pub fn generate_day16_part2(input: &str) -> Box<(Vec<Sample>, Vec<Instruction>)> {
    let (samples, test_program) = input.split("\n\n\n").collect_tuple().unwrap();
    Box::new((get_samples(samples), get_test_program(test_program)))
}

#[derive(EnumIter, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum OpCode {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

impl OpCode {
    pub fn execute(&self, instruction: &Instruction, registers: &[usize; 4]) -> [usize; 4] {
        let mut output = registers.clone();
        match self {
            OpCode::AddR => {
                output[instruction.c] = registers[instruction.a] + registers[instruction.b]
            }
            OpCode::AddI => output[instruction.c] = registers[instruction.a] + instruction.b,
            OpCode::MulR => {
                output[instruction.c] = registers[instruction.a] * registers[instruction.b]
            }
            OpCode::MulI => output[instruction.c] = registers[instruction.a] * instruction.b,
            OpCode::BanR => {
                output[instruction.c] = registers[instruction.a] & registers[instruction.b]
            }
            OpCode::BanI => output[instruction.c] = registers[instruction.a] & instruction.b,
            OpCode::BorR => {
                output[instruction.c] = registers[instruction.a] | registers[instruction.b]
            }
            OpCode::BorI => output[instruction.c] = registers[instruction.a] | instruction.b,
            OpCode::SetR => output[instruction.c] = registers[instruction.a],
            OpCode::SetI => output[instruction.c] = instruction.a,
            OpCode::GtIR => {
                output[instruction.c] = if instruction.a > registers[instruction.b] {
                    1
                } else {
                    0
                }
            }
            OpCode::GtRI => {
                output[instruction.c] = if registers[instruction.a] > instruction.b {
                    1
                } else {
                    0
                }
            }
            OpCode::GtRR => {
                output[instruction.c] = if registers[instruction.a] > registers[instruction.b] {
                    1
                } else {
                    0
                }
            }
            OpCode::EqIR => {
                output[instruction.c] = if instruction.a == registers[instruction.b] {
                    1
                } else {
                    0
                }
            }
            OpCode::EqRI => {
                output[instruction.c] = if registers[instruction.a] == instruction.b {
                    1
                } else {
                    0
                }
            }
            OpCode::EqRR => {
                output[instruction.c] = if registers[instruction.a] == registers[instruction.b] {
                    1
                } else {
                    0
                }
            }
        }
        output
    }
}

#[aoc(day16, part1)]
pub fn solve_day16_part1(samples: &[Sample]) -> usize {
    let mut count = 0;
    for sample in samples {
        if OpCode::iter()
            .map(|opcode| opcode.execute(&sample.instruction, &sample.registers_before))
            .filter(|opcode_register| opcode_register == &sample.registers_after)
            .count()
            >= 3
        {
            count += 1;
        }
    }
    count
}

fn find_opcodes(samples: &[Sample]) -> HashMap<u8, OpCode> {
    let mut set_mapping = HashMap::new();
    let mut found = HashMap::new();
    for i in 0..16 {
        set_mapping.insert(i, HashSet::new());
    }
    loop {
        for sample in samples {
            let opcode_number = sample.instruction.opcode;
            let opcodes: HashSet<OpCode> = OpCode::iter()
                .filter_map(|opcode| {
                    if opcode.execute(&sample.instruction, &sample.registers_before)
                        == sample.registers_after
                        && !found.contains_key(&opcode_number)
                    {
                        Some(opcode)
                    } else {
                        None
                    }
                })
                .collect();
            let mut values = set_mapping.remove(&opcode_number).unwrap();
            if values.is_empty() {
                values = values.union(&opcodes).cloned().collect();
                set_mapping.insert(
                    opcode_number,
                    values
                        .difference(&found.values().cloned().collect())
                        .cloned()
                        .collect(),
                );
            } else if values.len() > 1 {
                values = values.intersection(&opcodes).cloned().collect();
                set_mapping.insert(
                    opcode_number,
                    values
                        .difference(&found.values().cloned().collect())
                        .cloned()
                        .collect(),
                );
            } else {
                set_mapping.insert(opcode_number, values);
            }
            if set_mapping[&opcode_number].len() == 1 {
                found.insert(
                    opcode_number,
                    set_mapping[&opcode_number].iter().next().unwrap().clone(),
                );
            }
        }
        if (0..16).all(|n| set_mapping[&n].len() == 1) {
            break;
        }
    }
    found
}

#[aoc(day16, part2)]
pub fn solve_day16_part2(input: &(Vec<Sample>, Vec<Instruction>)) -> usize {
    let (samples, test_program) = input;
    let opcode_mapping = find_opcodes(samples);
    let mut registers = [0; 4];
    for instruction in test_program {
        registers = opcode_mapping[&instruction.opcode].execute(instruction, &registers);
    }
    registers[0]
}
