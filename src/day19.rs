#[derive(Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    pub fn new(line: &str) -> Instruction {
        let parts: Vec<_> = line.split(' ').collect();
        Instruction {
            opcode: parts[0].parse::<OpCode>().unwrap(),
            a: parts[1].parse::<usize>().unwrap(),
            b: parts[2].parse::<usize>().unwrap(),
            c: parts[3].parse::<usize>().unwrap(),
        }
    }
}

#[derive(EnumIter, Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString)]
pub enum OpCode {
    #[strum(serialize = "addr")]
    AddR,
    #[strum(serialize = "addi")]
    AddI,
    #[strum(serialize = "mulr")]
    MulR,
    #[strum(serialize = "muli")]
    MulI,
    #[strum(serialize = "banr")]
    BanR,
    #[strum(serialize = "bani")]
    BanI,
    #[strum(serialize = "borr")]
    BorR,
    #[strum(serialize = "bori")]
    BorI,
    #[strum(serialize = "setr")]
    SetR,
    #[strum(serialize = "seti")]
    SetI,
    #[strum(serialize = "gtir")]
    GtIR,
    #[strum(serialize = "gtri")]
    GtRI,
    #[strum(serialize = "gtrr")]
    GtRR,
    #[strum(serialize = "eqir")]
    EqIR,
    #[strum(serialize = "eqri")]
    EqRI,
    #[strum(serialize = "eqrr")]
    EqRR,
}

impl OpCode {
    pub fn execute(&self, instruction: &Instruction, registers: &[usize; 6]) -> [usize; 6] {
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

#[aoc_generator(day19)]
pub fn generate_day19(input: &str) -> Box<(usize, Vec<Instruction>)> {
    let mut program = Vec::new();
    let ip_register = input
        .split('\n')
        .next()
        .unwrap()
        .split(' ')
        .collect::<Vec<_>>()[1]
        .parse::<usize>()
        .unwrap();
    for line in input.split('\n').skip(1) {
        program.push(Instruction::new(line));
    }
    Box::new((ip_register, program))
}

#[aoc(day19, part1)]
pub fn solve_day19_part1(input: &(usize, Vec<Instruction>)) -> usize {
    let mut ip = 0;
    let mut registers = [0; 6];
    let ip_register = input.0;
    let instructions = &input.1;
    let mut instruction;
    loop {
        if ip >= instructions.len() {
            break;
        }
        instruction = &instructions[ip];
        registers[ip_register] = ip;
        registers = instruction.opcode.execute(instruction, &registers);
        ip = registers[ip_register];
        ip += 1;
    }
    registers[0]
}

#[aoc(day19, part2)]
pub fn solve_day19_part2(input: &(usize, Vec<Instruction>)) -> usize {
    let mut ip = 0;
    let mut registers = [0; 6];
    registers[0] = 1;
    let ip_register = input.0;
    let instructions = &input.1;
    let mut instruction;
    let r0 = 1;
    registers = [r0, 21102576, 10551287, r0 + 1, 9, 10551288];
    ip = 9;
    loop {
        if ip >= instructions.len() {
            break;
        }
        registers[ip_register] = ip;
        instruction = &instructions[ip];
        print!("ip {} {:?} {:?} ", ip, registers, instruction);
        registers = instruction.opcode.execute(instruction, &registers);
        ip = registers[ip_register];
        println!("{:?}", registers);
        ip += 1;
        //println!("{:?}", registers);
    }
    registers[0]
}
