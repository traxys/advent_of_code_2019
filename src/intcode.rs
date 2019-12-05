#[aoc_generator(day2)]
pub fn get_intcode(input: &str) -> Vec<u64> {
    input.split(",").map(|u| u.parse().unwrap()).collect()
}

fn run_intcode(code: &mut [u64]) -> Result<u64, String> {
    let mut position = 0;
    while position < code.len() {
        match code[position] {
            1 => {
                let source_a = code[position + 1] as usize;
                let source_b = code[position + 2] as usize;
                let dest = code[position + 3] as usize;
                code[dest] = code[source_a] + code[source_b];
            }
            2 => {
                let source_a = code[position + 1] as usize;
                let source_b = code[position + 2] as usize;
                let dest = code[position + 3] as usize;
                code[dest] = code[source_a] * code[source_b];
            }
            99 => return Ok(code[0]),
            i => return Err(format!("invalid opcode: {}", i)),
        }
        position += 4;
    }
    return Err("did not land on 99".to_owned());
}

#[aoc(day2, part1)]
pub fn execute_intcode(code: &[u64]) -> Result<u64, String> {
    let mut code = Vec::from(code);
    code[1] = 12;
    code[2] = 2;
    run_intcode(&mut code)
}

#[aoc(day2, part2)]
pub fn find_good_code(initial_memory: &[u64]) -> Result<u64, String> {
    let target_value = 19690720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut code = Vec::from(initial_memory);
            code[1] = noun;
            code[2] = verb;
            if run_intcode(&mut code)? == target_value {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err("No pair found".to_owned())
}

#[aoc_generator(day5)]
pub fn parse_intcode(code: &str) -> Vec<i64> {
    code.split(",").map(|c| c.parse().unwrap()).collect()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum InstructionMode {
    Position,
    Immediate,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Opcode {
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Mult,
    Add,
    Input,
    Output,
    Exit,
}
impl Opcode {
    fn arg_count(&self) -> usize {
        match self {
            Opcode::Mult | Opcode::Add | Opcode::LessThan | Opcode::Equals => 3,
            Opcode::Input | Opcode::Output => 1,
            Opcode::JumpIfFalse | Opcode::JumpIfTrue => 2,
            Opcode::Exit => 0,
        }
    }
    fn more(&self) -> bool {
        match self {
            Opcode::Exit => false,
            _ => true,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Instruction {
    op: Opcode,
    args: Vec<Parameter>,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Parameter {
    value: i64,
    mode: InstructionMode,
}

fn get_mode(param_num: usize, modes: &[InstructionMode]) -> InstructionMode {
    *modes.get(param_num).unwrap_or(&InstructionMode::Position)
}
fn get_arg(param_num: usize, modes: &[InstructionMode], code: &[i64]) -> Parameter {
    Parameter {
        value: code[param_num + 1],
        mode: get_mode(param_num, modes),
    }
}
fn get_args(count: usize, modes: &[InstructionMode], code: &[i64]) -> Vec<Parameter> {
    let mut args = Vec::new();
    for i in 0..count {
        args.push(get_arg(i, modes, code));
    }
    args
}
fn extract_from_opcode(
    opcode: Opcode,
    modes: &[InstructionMode],
    code: &[i64],
) -> (Instruction, usize) {
    (
        Instruction {
            op: opcode,
            args: get_args(opcode.arg_count(), &modes, code),
        },
        opcode.arg_count() + 1,
    )
}

fn get_instruction(code: &[i64]) -> Result<(Instruction, usize), String> {
    let instr = code[0];
    let opcode = match instr % 100 {
        01 => Opcode::Add,
        02 => Opcode::Mult,
        03 => Opcode::Input,
        04 => Opcode::Output,
        05 => Opcode::JumpIfTrue,
        06 => Opcode::JumpIfFalse,
        07 => Opcode::LessThan,
        08 => Opcode::Equals,
        99 => Opcode::Exit,
        i => return Err(format!("No such opcode: {}", i)),
    };
    let mut modes_int = instr / 100;
    let mut modes = Vec::new();
    while modes_int != 0 {
        match modes_int % 10 {
            0 => modes.push(InstructionMode::Position),
            1 => modes.push(InstructionMode::Immediate),
            i => return Err(format!("Invalid mode: {}", i)),
        }
        modes_int /= 10;
    }
    Ok(extract_from_opcode(opcode, &modes, code))
}
fn resolve_arg(param: Parameter, code: &[i64]) -> i64 {
    match param.mode {
        InstructionMode::Immediate => param.value,
        InstructionMode::Position => code[param.value as usize],
    }
}

use std::collections::VecDeque;

fn exec_instr(
    param: Instruction,
    code: &mut [i64],
    input: &mut VecDeque<i64>,
    ouput: &mut Vec<i64>,
) -> (bool, Option<usize>) {
    let new_ip = match param.op {
        Opcode::Add => {
            code[param.args[2].value as usize] =
                resolve_arg(param.args[0], code) + resolve_arg(param.args[1], code);
            None
        }
        Opcode::Mult => {
            code[param.args[2].value as usize] =
                resolve_arg(param.args[0], code) * resolve_arg(param.args[1], code);
            None
        }
        Opcode::Input => {
            let value = input.pop_front().expect("No input");
            code[param.args[0].value as usize] = value;
            None
        }
        Opcode::Output => {
            ouput.push(resolve_arg(param.args[0], code));
            None
        }
        Opcode::Exit => None,
        Opcode::JumpIfTrue => {
            if resolve_arg(param.args[0], code) != 0 {
                Some(resolve_arg(param.args[1], code) as usize)
            } else {
                None
            }
        }
        Opcode::JumpIfFalse => {
            if resolve_arg(param.args[0], code) == 0 {
                Some(resolve_arg(param.args[1], code) as usize)
            } else {
                None
            }
        }
        Opcode::LessThan => {
            code[param.args[2].value as usize] =
                if resolve_arg(param.args[0], code) < resolve_arg(param.args[1], code) {
                    1
                } else {
                    0
                };
            None
        }
        Opcode::Equals => {
            code[param.args[2].value as usize] =
                if resolve_arg(param.args[0], code) == resolve_arg(param.args[1], code) {
                    1
                } else {
                    0
                };
            None
        }
    };
    (param.op.more(), new_ip)
}

fn run_intcode_program(mut memory: Vec<i64>, mut input: VecDeque<i64>) -> Vec<i64> {
    let mut output = Vec::new();
    
    let mut instruction_pointer = 0;
    loop {
        let (instr, offset) = get_instruction(&memory[instruction_pointer..]).expect("invalid instruction");
        let (cont, new_ip) = exec_instr(instr, &mut memory, &mut input, &mut output);
        if !cont {
            break;
        }
        match new_ip {
            Some(i) => instruction_pointer = i,
            None => instruction_pointer += offset,
        };
    }

    output
}

#[aoc(day5, part1)]
pub fn execute_better_intcode(code: &[i64]) -> i64 {
    let memory = Vec::from(code);
    let input = VecDeque::from(vec![1]);
    let output = run_intcode_program(memory, input);

    *output.last().unwrap()
}

#[aoc(day5, part2)]
pub fn intcode_thermal_radiators(code: &[i64]) -> i64 {
    let memory = Vec::from(code);
    let input = VecDeque::from(vec![5]);
    let output = run_intcode_program(memory, input);

    *output.last().unwrap()
}
