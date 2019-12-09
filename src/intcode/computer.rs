use arrayvec::ArrayVec;
use std::collections::VecDeque;

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

pub const MAX_ARG_COUNT: usize = 3;
type ArgArray = ArrayVec<[Parameter; MAX_ARG_COUNT]>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum InstructionMode {
    Position,
    Immediate,
    Relative,
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Instruction {
    op: Opcode,
    args: ArgArray,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Parameter {
    value: i64,
    mode: InstructionMode,
}

impl Parameter {
    fn new(param_num: usize, modes: &[InstructionMode], code: &[i64]) -> Parameter {
        Parameter {
            value: code[param_num + 1],
            mode: modes[param_num],
        }
    }
    fn extract_args(count: usize, modes: &[InstructionMode], code: &[i64]) -> ArgArray {
        let mut args = ArgArray::new();
        for i in 0..count {
            unsafe { args.push_unchecked(Parameter::new(i, modes, code)) };
        }
        args
    }
}

impl Instruction {
    fn create_with_op_and_modes(
        opcode: Opcode,
        modes: &[InstructionMode],
        code: &[i64],
    ) -> (Instruction, usize) {
        (
            Instruction {
                op: opcode,
                args: Parameter::extract_args(opcode.arg_count(), &modes, code),
            },
            opcode.arg_count() + 1,
        )
    }
    fn from_code(code: &[i64]) -> Result<(Instruction, usize), String> {
        let instr = code[0];
        let raw_code = instr % 100;
        let opcode =
            Opcode::from_code(raw_code).map_err(|_| format!("No such code: {}", raw_code))?;
        let mut modes_int = instr / 100;
        let mut modes = [InstructionMode::Position; MAX_ARG_COUNT];
        let mut arg_index = 0;
        while modes_int != 0 {
            match modes_int % 10 {
                0 => modes[arg_index] = InstructionMode::Position,
                1 => modes[arg_index] = InstructionMode::Immediate,
                2 => modes[arg_index] = InstructionMode::Relative,
                i => return Err(format!("Invalid mode: {}", i)),
            }
            modes_int /= 10;
            arg_index += 1;
        }
        Ok(Instruction::create_with_op_and_modes(opcode, &modes, code))
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

pub struct IntcodeComputer {
    input: VecDeque<i64>,
    pub output: Vec<i64>,
    memory: Vec<i64>,

    relative_base: i64,

    instruction_pointer: usize,
    finished: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntcodeState {
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

    pub fn new(code: Vec<i64>) -> IntcodeComputer {
        IntcodeComputer {
            relative_base: 0,
            instruction_pointer: 0,
            finished: false,
            memory: code,
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }
    pub fn run(&mut self) {
        loop {
            match self.step() {
                IntcodeState::Finished => break,
                IntcodeState::Ready | IntcodeState::Outputed => continue,
                IntcodeState::NeedsInput => panic!("Input needed"),
            }
        }
    }
    #[inline]
    pub fn output(&self) -> &[i64] {
        &self.output
    }
    pub fn add_input(&mut self, value: i64) {
        self.input.push_back(value);
    }

    pub fn step(&mut self) -> IntcodeState {
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
