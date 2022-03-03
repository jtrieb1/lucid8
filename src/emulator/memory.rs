use super::emulator::{MemoryAddress, Result};
use std::fmt::{Display, Formatter};

const HEX_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80
];

#[derive(Debug)]
pub struct Memory {
    buffer: [u8; 4096]
}

impl Default for Memory {
    fn default() -> Self {
        let mut memory: [u8; 4096] = [0; 4096];
        memory[..80].copy_from_slice(&HEX_SPRITES[..]);
        Self {
            buffer: memory
        }
    }
}

impl Memory {
    pub fn clear(&mut self) {
        self.buffer[0x200..].clone_from_slice(&[0; 3584]);
    }

    pub fn get_range(&self, start_addr: MemoryAddress, length: usize) -> Result<&[u8]> {
        if start_addr as usize + length >= self.buffer.len() {
            return Err(Box::new(MemoryError::OutOfBounds(start_addr, length)))
        }
        Ok(&self.buffer[start_addr as usize .. start_addr as usize + length])
    }

    pub fn set_range(&mut self, start_addr: MemoryAddress, data: &[u8]) -> Result<()> {
        let data_len = data.len();
        if start_addr as usize + data_len > self.buffer.len() {
            return Err(Box::new(MemoryError::OutOfMemory(start_addr, start_addr as usize + data_len - self.buffer.len())));
        }
        self.buffer[start_addr as usize..start_addr as usize + data.len()].copy_from_slice(data);
        Ok(())
    }

    pub fn set_byte(&mut self, addr: MemoryAddress, byte: u8) -> Result<()> {
        if addr as usize >= self.buffer.len() || addr < 0 {
            return Err(Box::new(MemoryError::OutOfBounds(addr, 0)));
        }
        self.buffer[addr as usize] = byte;
        Ok(())
    }
}

#[derive(Debug)]
pub enum MemoryError {
    OutOfBounds(MemoryAddress, usize),
    OutOfMemory(MemoryAddress, usize)
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::OutOfBounds(addr, length) => write!(f, "The memory range accessed is out of bounds (max 4096): [{} ... {}]", addr, *addr + *length as u16),
            MemoryError::OutOfMemory(addr, excess_bytes) => write!(f, "Attempting to write memory past bounds, starting at {}. [Excess bytes: {}]", addr, excess_bytes)
        }
    }
}

impl std::error::Error for MemoryError {}