use std::{
    fmt::{Display, Formatter},
    fs,
    io::{stdin, stdout},
};

use crate::interpreter::{Interpreter, InterpreterError};

#[derive(Debug)]
pub struct Cli(FileName);

pub type FileName = String;

impl Cli {
    pub fn new(args: Vec<String>) -> Result<Self, CliError> {
        let Some(file_name) = args.get(1) else {
			return Err(CliError::FileNameNotFound);
		};
        let Ok(code) = fs::read_to_string(file_name) else {
			return Err(CliError::UnableToReadFile);
		};
        Ok(Self(code))
    }

    pub fn run(&self) -> Result<(), InterpreterError> {
        let mut interpreter = Interpreter::new(stdin(), stdout());
        match interpreter.run(&self.0) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug)]
pub enum CliError {
    FileNameNotFound,
    UnableToReadFile,
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let err_text = match self {
            Self::FileNameNotFound => "File name argument not found",
            Self::UnableToReadFile => "Unable to read given file name",
        };
        write!(f, "{}", err_text)
    }
}
