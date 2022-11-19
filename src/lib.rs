use std::io::{Read, Write};

#[derive(Debug)]
pub struct Interpreter<R: Read, W: Write> {
    program_counter: usize,
    memory_addrs: usize,
    memory: [u8; 30_000],
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            program_counter: 0,
            memory_addrs: 0,
            memory: [0; 30_000],
            reader,
            writer,
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), InterpreterError> {
        let mut program = Program::new(code);
        let result = loop {
            match program.instruction(self.program_counter) {
                Some(instruction) => self.exec(&instruction)?,
                None => break Ok(()),
            }
        };
        self.reset();
        result
    }

    fn exec(&mut self, instruction: &Instruction) -> Result<(), InterpreterError> {
        match instruction {
            Instruction::MoveRight => {
                if self.memory_addrs == 30_000 {
                    return Err(InterpreterError::MemoryAddrsOverflow);
                }
                self.memory_addrs += 1;
            }
            Instruction::MoveLeft => {
                if self.memory_addrs == 0 {
                    return Err(InterpreterError::MemoryAddrsOverflow);
                }
                self.memory_addrs -= 1;
            }
            Instruction::Add => {
                if self.memory[self.memory_addrs] == u8::MAX {
                    println!("Maior que u8::MAX");
                    return Err(InterpreterError::DataOverflow);
                }
                self.memory[self.memory_addrs] += 1;
            }
            Instruction::Sub => {
                if self.memory[self.memory_addrs] == 0 {
                    println!("Menor que zero");
                    return Err(InterpreterError::DataOverflow);
                }
                self.memory[self.memory_addrs] -= 1;
            }
            Instruction::Write => {
                let data = self.memory[self.memory_addrs];
                if let Err(_) = self.writer.write(&[data]) {
                    return Err(InterpreterError::IoWriteFail);
                }
            }
            Instruction::Read => {
                let mut input = [0];
                if let Err(_) = self.reader.read_exact(&mut input) {
                    return Err(InterpreterError::IoWriteFail);
                }
                self.memory[self.memory_addrs] = input[0];
            }
            Instruction::JumpIfZero(index) => {
                let Some(index) = index else {
                    return Err(InterpreterError::MissingRightBracket);
                };
                if self.memory[self.memory_addrs] == 0 {
                    self.program_counter = *index;
                }
            }
            Instruction::Jump(index) => {
                let Some(index) = index else {
                    return Err(InterpreterError::MissingLeftBracket);
                };
                self.program_counter = *index;
            }
            Instruction::Noop => {}
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.program_counter = 0;
        self.memory_addrs = 0;
        self.memory = [0; 30_000];
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
                    let instruction = match lbracket_stack.pop() {
                        Some(lbracket_index) => {
                            match instructions.get_mut(lbracket_index) {
                                Some(instruction) => {
                                    *instruction = Instruction::JumpIfZero(Some(i));
                                }
                                None => {} // Unreachable??
                            };
                            Instruction::Jump(Some(lbracket_index))
                        }
                        None => Instruction::Jump(None),
                    };
                    instructions.push(instruction);
                }
                _ => instructions.push(Instruction::Noop),
            };
        }
        instructions.reverse();
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
