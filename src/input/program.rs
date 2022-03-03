use std::fmt::{Display, Formatter};

use crate::emulator::instructions::Instruction;
use crate::emulator::emulator::Result;

pub struct Program {
    bytes: [u8; 3584],
    instructions: Vec<Instruction>,
    script: Option<String>
}

#[derive(Debug)]
pub enum ProgramError {
    SpaceExceeded(usize)
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramError::SpaceExceeded(size) => write!(f, "Program too large! Max Bytes: [3584], Given: [{}]", size)
        }
    }
}

impl std::error::Error for ProgramError {}

impl Default for Program {
    fn default() -> Self {
        Self {
            bytes: [0; 3584],
            instructions: vec![],
            script: None
        }
    }
}

impl Program {
    pub fn save_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        if bytes.len() > 3584 {
            return Err(Box::new(ProgramError::SpaceExceeded(bytes.len())));
        }
        self.bytes.copy_from_slice(&bytes[..]);
        Ok(())
    }
}