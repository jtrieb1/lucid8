use  super::emulator::{Result, RegisterAddress};
use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub struct Registers {
    buffer: [u8; 16]
}

impl Registers {
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..]
    }

    pub fn set_bytes(&mut self, data: &[u8]) -> Result<()> {
        if data.len() > 16 {
            return Err(Box::new(RegisterError::InvalidRegister(data.len() as u8 - 1)));
        }
        self.buffer[..].copy_from_slice(data);
        Ok(())
    }

    pub fn get(&self, register_address: RegisterAddress) -> Result<u8> {
        if register_address < self.buffer.len() as u8 {
            Ok(self.buffer[register_address as usize])
        } else {
            Err(Box::new(RegisterError::InvalidRegister(register_address)))
        }
    }

    pub fn get_pair(&self, addr1: RegisterAddress, addr2: RegisterAddress) -> Result<(u8, u8)> {
        let a = self.get(addr1)?;
        let b = self.get(addr2)?;
        Ok((a, b))
    }

    pub fn set(&mut self, addr: RegisterAddress, value: u8) -> Result<()> {
        if addr < self.buffer.len() as u8 {
            self.buffer[addr as usize] = value;
            Ok(())
        } else {
            Err(Box::new(RegisterError::InvalidRegister(addr)))
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            buffer: [0; 16]
        }
    }
}

#[derive(Debug)]
pub enum RegisterError {
    InvalidRegister(RegisterAddress)
}

impl Display for RegisterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterError::InvalidRegister(vx) => {
                write!(f, "Invalid register access: V{:X}", vx)
            }
        }
    }
}

impl Error for RegisterError {}