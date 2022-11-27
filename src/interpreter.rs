use std::{
    fmt::{Display, Formatter},
    io::{Read, Write},
};

const MEMORY_SIZE: usize = 30_000;

#[derive(Debug)]
pub struct Interpreter<R: Read, W: Write> {
    program_counter: usize,
    memory_address: usize,
    memory: [u8; MEMORY_SIZE],
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            program_counter: 0,
            memory_address: 0,
            memory: [0; MEMORY_SIZE],
            reader,
            writer,
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), InterpreterError> {
        let mut program = Program::new(code);
        let result = loop {
            match program.instruction(self.program_counter) {
                Some(instruction) => self.exec(instruction)?,
                None => break Ok(()),
            }
            self.program_counter += 1;
        };
        self.reset();
        result
    }

    fn exec(&mut self, instruction: &Instruction) -> Result<(), InterpreterError> {
        match instruction {
            Instruction::MoveRight => {
                if self.memory_address == 30_000 {
                    return Err(InterpreterError::MemoryAddrsOverflow);
                }
                self.memory_address += 1;
            }
            Instruction::MoveLeft => {
                if self.memory_address == 0 {
                    return Err(InterpreterError::MemoryAddrsOverflow);
                }
                self.memory_address -= 1;
            }
            Instruction::Add => {
                if self.memory[self.memory_address] == u8::MAX {
                    return Err(InterpreterError::DataOverflow);
                }
                self.memory[self.memory_address] += 1;
            }
            Instruction::Sub => {
                if self.memory[self.memory_address] == 0 {
                    return Err(InterpreterError::DataOverflow);
                }
                self.memory[self.memory_address] -= 1;
            }
            Instruction::Write => {
                let data = self.memory[self.memory_address];
                if let Err(_) = self.writer.write(&[data]) {
                    return Err(InterpreterError::IoWriteFail);
                }
            }
            Instruction::Read => {
                let mut input = [0];
                if let Err(_) = self.reader.read_exact(&mut input) {
                    return Err(InterpreterError::IoWriteFail);
                }
                self.memory[self.memory_address] = input[0];
            }
            Instruction::JumpIfZero(index) => {
                let Some(index) = index else {
                    return Err(InterpreterError::MissingRightBracket);
                };
                if self.memory[self.memory_address] == 0 {
                    self.program_counter = *index;
                }
            }
            Instruction::Jump(index) => {
                let Some(index) = index else {
                    return Err(InterpreterError::MissingLeftBracket);
                };
                self.program_counter = *index - 1;
            }
            Instruction::Noop => {}
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.program_counter = 0;
        self.memory_address = 0;
        self.memory = [0; MEMORY_SIZE];
    }
}

#[derive(Debug)]
enum Instruction {
    MoveRight,
    MoveLeft,
    Add,
    Sub,
    Write,
    Read,
    JumpIfZero(Option<usize>),
    Jump(Option<usize>),
    Noop,
}

#[derive(Debug)]
struct Program(Vec<Instruction>);

impl Program {
    fn new(code: &str) -> Self {
        let mut lbracket_stack = Vec::new();
        let mut instructions = Vec::new();
        for (i, c) in code.as_bytes().iter().enumerate() {
            match c {
                b'>' => instructions.push(Instruction::MoveRight),
                b'<' => instructions.push(Instruction::MoveLeft),
                b'+' => instructions.push(Instruction::Add),
                b'-' => instructions.push(Instruction::Sub),
                b'.' => instructions.push(Instruction::Write),
                b',' => instructions.push(Instruction::Read),
                b'[' => {
                    lbracket_stack.push(i);
                    instructions.push(Instruction::JumpIfZero(None))
                }
                b']' => {
                    let Some(lbracket_index) = lbracket_stack.pop() else {
                        instructions.push(Instruction::Jump(None));
                        continue;
                    };
                    let Some(instruction) = instructions.get_mut(lbracket_index) else {
                        // Variable lbracket_index will allways
                        // point to a valid position inside the
                        // instructions vec
                        unreachable!()
                    };

                    // Go back to the corresponding `[`
                    // and update its jump index
                    *instruction = Instruction::JumpIfZero(Some(i));

                    // Set current `]` to jump back to
                    // its corresponding `[`
                    instructions.push(Instruction::Jump(Some(lbracket_index)));
                }
                _ => instructions.push(Instruction::Noop),
            };
        }
        Self(instructions)
    }

    fn instruction(&mut self, index: usize) -> Option<&Instruction> {
        self.0.get(index)
    }
}

#[derive(Debug)]
pub enum InterpreterError {
    IoReadFail,
    IoWriteFail,
    MemoryAddrsOverflow,
    DataOverflow,
    MissingLeftBracket,
    MissingRightBracket,
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let err_text = match self {
            Self::IoReadFail => "Failed to read user input",
            Self::IoWriteFail => "Failed to write data to output",
            Self::MemoryAddrsOverflow => "Memory address out of boundaries",
            Self::DataOverflow => "Memory data greater than 255 or less than 0",
            Self::MissingLeftBracket => "Missing left bracket",
            Self::MissingRightBracket => "Missing right bracket",
        };
        write!(f, "{}", err_text)
    }
}
