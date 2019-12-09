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

#[aoc_generator(day9)]
#[aoc_generator(day7)]
#[aoc_generator(day5)]
pub fn parse_intcode(code: &str) -> Vec<i64> {
    code.split(",").map(|c| c.parse().unwrap()).collect()
}

macro_rules! define_opcodes_impl {
    (
        @define    ($($d_stack:ident),*),
        @arg_count ($($k:path => $v:tt),*),
        @from_code ($($c:expr => $op:path),*),
        $name:ident {
            args: $arg_count:expr,
            opcode: $opcode:expr $(,)?
        }
        $($rest:tt)*
    )=> {
        define_opcodes_impl!{
            @define    ($($d_stack,)* $name),
            @arg_count ($($k => $v,)* Opcode::$name => $arg_count),
            @from_code ($($c => $op,)* $opcode => Opcode::$name),
            $($rest)*
        }
    };
    (
        @define    ($($d_stack:ident),*),
        @arg_count ($($k:path => $v:tt),*),
        @from_code ($($c:expr => $op:path),*),
    ) => {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        enum Opcode {
            $(
                $d_stack
            ),*
        }
        impl Opcode {
            fn arg_count(&self) -> usize {
                match &self {
                $(
                    $k => $v
                ),*
                }
            }
            fn from_code(code: i64) -> Result<Self, ()> {
                match code {
                $(
                    $c => Ok($op)
                ),*,
                    _ => return Err(()),
                }
            }
        }
    };
}

macro_rules! define_opcodes {
    ($($rest:tt)*) => {
        define_opcodes_impl!{@define (), @arg_count (), @from_code (), $($rest)*}
    };
}

define_opcodes! {
    Add {
        args: 3,
        opcode: 01,
    }
    Mult {
        args: 3,
        opcode: 02,
    }
    Input {
        args: 1,
        opcode: 03,
    }
    Output {
        args: 1,
        opcode: 04,
    }
    JumpIfTrue {
        args: 2,
        opcode: 05,
    }
    JumpIfFalse {
        args: 2,
        opcode: 06,
    }
    LessThan {
        args: 3,
        opcode: 07,
    }
    Equals {
        args: 3,
        opcode: 08,
    }
    RelativeUpdate {
        args: 1,
        opcode: 09,
    }
    Exit {
        args: 0,
        opcode: 99,
    }
}

impl Opcode {
    fn more(&self) -> bool {
        match self {
            Opcode::Exit => false,
            _ => true,
        }
    }
    fn needs_input(&self) -> bool {
        match self {
            Opcode::Input => true,
            _ => false,
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum InstructionMode {
    Position,
    Immediate,
    Relative,
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

impl Instruction {
    fn from_code(code: &[i64]) -> Result<(Instruction, usize), String> {
        let instr = code[0];
        let raw_code = instr % 100;
        let opcode =
            Opcode::from_code(raw_code).map_err(|_| format!("No such code: {}", raw_code))?;
        let mut modes_int = instr / 100;
        let mut modes = Vec::new();
        while modes_int != 0 {
            match modes_int % 10 {
                0 => modes.push(InstructionMode::Position),
                1 => modes.push(InstructionMode::Immediate),
                2 => modes.push(InstructionMode::Relative),
                i => return Err(format!("Invalid mode: {}", i)),
            }
            modes_int /= 10;
        }
        Ok(extract_from_opcode(opcode, &modes, code))
    }
    fn resolve_arg(&self, index: usize, computer: &IntcodeComputer) -> i64 {
        let param = self.args[index];
        match param.mode {
            InstructionMode::Immediate => param.value,
            InstructionMode::Position => computer.get(param.value as usize),
            InstructionMode::Relative => computer.get(computer.resolve_relative(param.value)),
        }
    }
    fn arg_as_index(&self, index: usize, computer: &IntcodeComputer) -> usize {
        let param = self.args[index];
        match param.mode {
            InstructionMode::Position => param.value as usize,
            InstructionMode::Relative => computer.resolve_relative(param.value),
            mode => panic!("Invaild mode to index with: {:?}", mode),
        }
    }
}

use std::collections::VecDeque;

impl IntcodeComputer {
    fn exec_instr(&mut self, param: Instruction) -> (bool, Option<usize>) {
        let new_ip = match param.op {
            Opcode::Add => {
                self.set(
                    param.arg_as_index(2, self),
                    param.resolve_arg(0, self) + param.resolve_arg(1, self),
                );
                None
            }
            Opcode::Mult => {
                self.set(
                    param.arg_as_index(2, self),
                    param.resolve_arg(0, self) * param.resolve_arg(1, self),
                );
                None
            }
            Opcode::Input => {
                let value = self.input.pop_front().expect("No input");
                self.set(param.arg_as_index(0, self), value);
                None
            }
            Opcode::Output => {
                self.output.push(param.resolve_arg(0, self));
                None
            }
            Opcode::Exit => None,
            Opcode::JumpIfTrue => {
                if param.resolve_arg(0, self) != 0 {
                    Some(param.resolve_arg(1, self) as usize)
                } else {
                    None
                }
            }
            Opcode::JumpIfFalse => {
                if param.resolve_arg(0, self) == 0 {
                    Some(param.resolve_arg(1, self) as usize)
                } else {
                    None
                }
            }
            Opcode::LessThan => {
                self.set(
                    param.arg_as_index(2, self),
                    if param.resolve_arg(0, self) < param.resolve_arg(1, self) {
                        1
                    } else {
                        0
                    },
                );
                None
            }
            Opcode::Equals => {
                self.set(
                    param.arg_as_index(2, self),
                    if param.resolve_arg(0, self) == param.resolve_arg(1, self) {
                        1
                    } else {
                        0
                    },
                );
                None
            }
            Opcode::RelativeUpdate => {
                self.relative_base += param.resolve_arg(0, self);
                None
            }
        };
        (param.op.more(), new_ip)
    }
}

struct IntcodeComputer {
    input: VecDeque<i64>,
    pub output: Vec<i64>,
    memory: Vec<i64>,

    relative_base: i64,

    instruction_pointer: usize,
    finished: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IntcodeState {
    Ready,
    NeedsInput,
    Finished,
    Outputed,
}

impl IntcodeComputer {
    #[inline]
    fn resolve_relative(&self, offset: i64) -> usize {
        (self.relative_base + offset) as usize
    }
    #[inline]
    fn get(&self, index: usize) -> i64 {
        *self.memory.get(index).unwrap_or(&0)
    }
    #[inline]
    fn set(&mut self, index: usize, value: i64) {
        if self.memory.len() <= index {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index] = value;
    }

    fn new(code: Vec<i64>) -> IntcodeComputer {
        IntcodeComputer {
            relative_base: 0,
            instruction_pointer: 0,
            finished: false,
            memory: code,
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }
    fn run(&mut self) {
        loop {
            match self.step() {
                IntcodeState::Finished => break,
                IntcodeState::Ready | IntcodeState::Outputed => continue,
                IntcodeState::NeedsInput => panic!("Input needed"),
            }
        }
    }
    fn last_output(&self) -> Option<i64> {
        self.output.last().copied()
    }
    fn add_input(&mut self, value: i64) {
        self.input.push_back(value);
    }
    fn step(&mut self) -> IntcodeState {
        if self.finished {
            IntcodeState::Finished
        } else {
            let (instr, offset) = Instruction::from_code(&self.memory[self.instruction_pointer..])
                .expect("invalid instr");
            let op = instr.op;
            if op.needs_input() && self.input.is_empty() {
                return IntcodeState::NeedsInput;
            }
            let (cont, new_ip) = self.exec_instr(instr);
            if !cont {
                self.finished = true;
                return IntcodeState::Finished;
            }
            match new_ip {
                Some(i) => self.instruction_pointer = i,
                None => self.instruction_pointer += offset,
            }
            match op {
                Opcode::Output => IntcodeState::Outputed,
                _ => IntcodeState::Ready,
            }
        }
    }
}

#[aoc(day5, part1)]
pub fn execute_better_intcode(code: &[i64]) -> i64 {
    let mut computer = IntcodeComputer::new(Vec::from(code));
    computer.add_input(1);
    computer.run();

    *computer.output.last().unwrap()
}

#[aoc(day5, part2)]
pub fn intcode_thermal_radiators(code: &[i64]) -> i64 {
    let mut computer = IntcodeComputer::new(Vec::from(code));
    computer.add_input(5);
    computer.run();

    *computer.output.last().unwrap()
}

use itertools::Itertools;

use std::cell::RefCell;
use std::collections::HashMap;
fn run_network(computers: &[RefCell<IntcodeComputer>], links: &HashMap<usize, Vec<usize>>) {
    loop {
        let mut all_finished = true;
        for (i, computer) in computers.iter().enumerate() {
            let new_state = computer.borrow_mut().step();
            all_finished &= new_state == IntcodeState::Finished;
            if let IntcodeState::Outputed = new_state {
                let new_input = computer.borrow().last_output().unwrap();
                if let Some(linked) = links.get(&i) {
                    for linked in linked {
                        computers[*linked].borrow_mut().add_input(new_input);
                    }
                }
            }
        }
        if all_finished {
            break;
        }
    }
}

fn chain(first: usize, last: usize) -> HashMap<usize, Vec<usize>> {
    let mut links = HashMap::new();
    for x in first..last {
        links.insert(x, vec![x + 1]);
    }
    links
}

fn prepare_amps(computers: &[RefCell<IntcodeComputer>], phase_scale: &[u8]) {
    for (computer, phase) in computers.iter().zip(phase_scale) {
        computer.borrow_mut().add_input(*phase as i64)
    }
}
fn run_amps(phase_scale: &[u8], code: &[i64]) -> i64 {
    let computers: Vec<_> = (0..5)
        .map(|_| IntcodeComputer::new(Vec::from(code)))
        .map(RefCell::new)
        .collect();
    prepare_amps(&computers, phase_scale);
    computers[0].borrow_mut().add_input(0);

    run_network(&computers, &chain(0, 4));

    let x = computers[4].borrow().last_output().unwrap();
    x
}

#[aoc(day7, part1)]
pub fn amplify_the_signal(code: &[i64]) -> i64 {
    (0..5)
        .permutations(5)
        .map(|c| run_amps(&c, code))
        .max()
        .expect("No permutation")
}

fn looped(start: usize, end: usize) -> HashMap<usize, Vec<usize>> {
    let mut chain = chain(start, end);
    chain.insert(end, vec![start]);
    chain
}

fn run_feedbacked_amps(phase_scale: &[u8], code: &[i64]) -> i64 {
    let computers: Vec<_> = (0..5)
        .map(|_| IntcodeComputer::new(Vec::from(code)))
        .map(RefCell::from)
        .collect();
    prepare_amps(&computers, phase_scale);
    computers[0].borrow_mut().add_input(0);

    run_network(&computers, &looped(0, 4));

    let x = computers[4].borrow().last_output().unwrap();
    x
}

#[aoc(day7, part2)]
pub fn amplify_the_signal_with_feedback(code: &[i64]) -> i64 {
    (5..10)
        .permutations(5)
        .map(|c| run_feedbacked_amps(&c, code))
        .max()
        .expect("No permutation")
}

#[aoc(day9, part1)]
pub fn test_boost(code: &[i64]) -> i64 {
    let mut computer = IntcodeComputer::new(Vec::from(code));
    computer.add_input(1);
    computer.run();

    computer.last_output().unwrap()
}
#[aoc(day9, part2)]
pub fn find_coordinates(code: &[i64]) -> i64 {
    let mut computer = IntcodeComputer::new(Vec::from(code));
    computer.add_input(2);
    computer.run();

    computer.last_output().unwrap()
}
