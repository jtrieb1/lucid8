use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use super::emulator::{MemoryAddress, RegisterAddress};

#[derive(Debug)]
pub enum Instruction {
    CLS,
    RET,
    JP(MemoryAddress),
    CALL(MemoryAddress),
    SE(RegisterAddress, u8),
    SNE(RegisterAddress, u8),
    SEV(RegisterAddress, RegisterAddress),
    LD(RegisterAddress, u8),
    ADD(RegisterAddress, u8),
    LDV(RegisterAddress, RegisterAddress),
    OR(RegisterAddress, RegisterAddress),
    AND(RegisterAddress, RegisterAddress),
    XOR(RegisterAddress, RegisterAddress),
    ADDV(RegisterAddress, RegisterAddress),
    SUB(RegisterAddress, RegisterAddress),
    SHR(RegisterAddress),
    SUBN(RegisterAddress, RegisterAddress),
    SHL(RegisterAddress),
    SNEV(RegisterAddress, RegisterAddress),
    LDI(MemoryAddress),
    JPV(MemoryAddress),
    RND(RegisterAddress, u8),
    DRW(RegisterAddress, RegisterAddress, u8),
    SKP(RegisterAddress),
    SKNP(RegisterAddress),
    LDD(u8),
    LDK(u8),
    LDDV(RegisterAddress),
    LDS(RegisterAddress),
    ADDI(RegisterAddress),
    LDF(RegisterAddress),
    LDB(RegisterAddress),
    LDIV,
    LDVI
}

#[derive(Debug)]
pub enum InstructionError {
    BadCode
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::BadCode => write!(f, "Invalid instruction code!")
        }
    }
}

impl std::error::Error for InstructionError {}

impl TryFrom<u16> for Instruction {
    type Error = Box<InstructionError>;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let parts: [u8; 4] = [((value & 0xF000) >> 12) as u8, ((value & 0x0F00) >> 8) as u8, ((value & 0x00F0) >> 4) as u8, (value & 0x000F) as u8];
        match parts {
            [0,0,0xE,0] => Ok(Instruction::CLS),
            [0,0,0xE,0xE] => Ok(Instruction::RET),
            [1, a, b, c] => Ok(Instruction::JP(a as u16 * 16 * 16 + b as u16 * 16 + c as u16)),
            [2, a, b, c] => Ok(Instruction::CALL(a as u16 * 16 * 16 + b as u16 * 16 + c as u16)),
            [3, a, b, c] => Ok(Instruction::SE(a, b * 16 + c)),
            [4, a, b, c] => Ok(Instruction::SNE(a, b * 16 + c)),
            [5, a, b, 0] => Ok(Instruction::SEV(a, b)),
            [6, a, b, c] => Ok(Instruction::LD(a, b * 16 + c)),
            [7, a, b, c] => Ok(Instruction::ADD(a, b * 16 + c)),
            [8, a, b, 0] => Ok(Instruction::LDV(a, b)),
            [8, a, b, 1] => Ok(Instruction::OR(a, b)),
            [8, a, b, 2] => Ok(Instruction::AND(a, b)),
            [8, a, b, 3] => Ok(Instruction::XOR(a, b)),
            [8, a, b, 4] => Ok(Instruction::ADDV(a, b)),
            [8, a, b, 5] => Ok(Instruction::SUB(a, b)),
            [8, a, _, 6] => Ok(Instruction::SHR(a)),
            [8, a, b, 7] => Ok(Instruction::SUBN(a, b)),
            [8, a, _, 0xE] => Ok(Instruction::SHL(a)),
            [9, a, b, 0] => Ok(Instruction::SNEV(a, b)),
            [0xA, a, b, c] => Ok(Instruction::LDI(a as u16 * 16 * 16 + b as u16 * 16 + c as u16)),
            [0xB, a, b, c] => Ok(Instruction::JPV(a as u16 * 16 * 16 + b as u16 * 16 + c as u16)),
            [0xC, a, b, c] => Ok(Instruction::RND(a, b * 16 + c)),
            [0xD, a, b, c] => Ok(Instruction::DRW(a, b, c)),
            [0xE, a, 9, 0xE] => Ok(Instruction::SKP(a)),
            [0xE, a, 0xA, 1] => Ok(Instruction::SKNP(a)),
            [0xF, a, 0, 7] => Ok(Instruction::LDD(a)),
            [0xF, a, 0, 0xA] => Ok(Instruction::LDK(a)),
            [0xF, a, 1, 5] => Ok(Instruction::LDDV(a)),
            [0xF, a, 1, 8] => Ok(Instruction::LDS(a)),
            [0xF, a, 1, 0xE] => Ok(Instruction::ADDI(a)),
            [0xF, a, 2, 9] => Ok(Instruction::LDF(a)),
            [0xF, a, 3, 3] => Ok(Instruction::LDB(a)),
            [0xF, _, 5, 5] => Ok(Instruction::LDIV),
            [0xF, _, 6, 5] => Ok(Instruction::LDVI),
            _ => Err(Box::new(InstructionError::BadCode))
        }
    }
}

impl TryFrom<[u8; 2]> for Instruction {
    type Error = Box<InstructionError>;
    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        let code = u16::from_be_bytes(value);
        Instruction::try_from(code)
    }
}

impl TryFrom<&[u8]> for Instruction {
    type Error = Box<InstructionError>;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut bytes: [u8; 2] = Default::default();
        bytes.copy_from_slice(&value[0..2]);
        Instruction::try_from(bytes)
    }
}