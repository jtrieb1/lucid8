extern crate rand;
use rand::Rng;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type MemoryAddress = u16;
pub type RegisterAddress = u8;
pub type StackPointer = usize;

use super::display::Display;
use super::instructions::Instruction;
use super::registers::Registers;
use super::memory::Memory;
use super::timed::TimedRegister;

#[derive(Debug)]
pub struct Emulator {
    display: Display,
    pc: MemoryAddress,
    i: MemoryAddress,
    sp: StackPointer,
    stack: [MemoryAddress; 16],
    registers: Registers,
    delay_register: TimedRegister,
    sound_register: TimedRegister,
    memory: Memory
}

impl Default for Emulator {
    fn default() -> Self {
        Self {
            display: Display::default(),
            pc: 0,
            i: 0,
            sp: 0,
            stack: [0; 16],
            registers: Registers::default(),
            delay_register: TimedRegister::default(),
            sound_register: TimedRegister::default(),
            memory: Memory::default()
        }
    }
}

impl Emulator {

    fn reset(&mut self) {
        self.display.clear();
        self.pc = 0x200;
        self.i = 0;
        self.sp = 0;
        self.stack = [0; 16];
        self.registers = Registers::default();
        self.delay_register.set(0);
        self.sound_register.set(0);
        self.memory.clear();
    }

    fn load_program(&mut self, program: &[u8]) -> Result<()> {
        self.reset();
        self.memory.set_range(0x200, program)?;
        Ok(())
    }

    fn interpret(&mut self, instruction: &Instruction) -> Result<()> {
        match instruction {
            Instruction::CLS => self.display.clear(),
            Instruction::RET => {
                self.pc = self.stack[self.sp];
                self.sp -= 1;
            },
            Instruction::JP(addr) => {
                self.pc = *addr;
            },
            Instruction::CALL(addr) => {
                self.sp += 1;
                self.stack[self.sp] = self.pc;
                self.pc = *addr;
            },
            Instruction::SE(vx, y) => {
                let x = self.registers.get(*vx)?;
                if x == *y { self.pc += 2; }
            },
            Instruction::SNE(vx, y) => {
                let x = self.registers.get(*vx)?;
                if x != *y { self.pc += 2; }
            },
            Instruction::SEV(vx, vy) => {
                let (x, y) = self.registers.get_pair(*vx, *vy)?;
                if x == y { self.pc += 2; }
            },
            Instruction::LD(vx, y) => {
                self.registers.set(*vx, *y)?;
            },
            Instruction::ADD(vx, y) => {
                let x = self.registers.get(*vx)?;
                self.registers.set(*vx, x + *y)?;
            },
            Instruction::LDV(vx, vy) => {
                let y = self.registers.get(*vy)?;
                self.registers.set(*vx, y)?;
            },
            Instruction::OR(vx, vy) => {
                let (x, y) = self.registers.get_pair(*vx, *vy)?;
                self.registers.set(*vx, x | y)?;
            },
            Instruction::AND(vx, vy) => {
                let (x, y) = self.registers.get_pair(*vx, *vy)?;
                self.registers.set(*vx, x & y)?;
            },
            Instruction::XOR(vx, vy) => {
                let (x, y) = self.registers.get_pair(*vx, *vy)?;
                self.registers.set(*vx, x ^ y)?;
            },
            Instruction::ADDV(vx, vy) => {
                let (x, y) = self.registers.get_pair(*vx, *vy)?;
                self.registers.set(*vx, x + y)?;
            },
            Instruction::SUB(vx, vy) => {
                let (x, y) = self.registers.get_pair(*vx, *vy)?;
                let pos = if x > y { 1 } else { 0 };
                self.registers.set(*vx, x - y)?;
                self.registers.set(0x0f, pos)?;
            },
            Instruction::SHR(vx) => {
                let x = self.registers.get(*vx)?;
                self.registers.set(*vx, x >> 1)?;
            },
            Instruction::SUBN(vx, vy) => {
                let (x, y) = self.registers.get_pair(*vx, *vy)?;
                let pos = if y > 1 { 1 } else { 0 };
                self.registers.set(*vx, y - x)?;
                self.registers.set(0x0f, pos)?;
            },
            Instruction::SHL(vx) => {
                let x = self.registers.get(*vx)?;
                self.registers.set(*vx, x << 1)?;
            },
            Instruction::SNEV(vx, vy) => {
                let (x, y) = self.registers.get_pair(*vx, *vy)?;
                if x != y { self.pc += 2; }
            },
            Instruction::LDI(addr) => {
                self.i = *addr;
            },
            Instruction::JPV(addr) => {
                let offset = self.registers.get(0)?;
                self.pc = offset as u16 + *addr;
            },
            Instruction::RND(vx, y) => {
                let x: u8 = rand::thread_rng().gen();
                self.registers.set(*vx, x & *y)?;
            },
            Instruction::DRW(vx, vy, num_bytes) => {
                let (x, y) = self.registers.get_pair(*vx, *vy)?;
                let sprite = self.memory.get_range(self.i, *num_bytes as usize)?;
                let collision = self.display.draw_bytes_at(x, y, sprite)?;
                self.registers.set(0xf, if collision {1} else {0})?;
            },
            Instruction::SKP(_) => todo!(),
            Instruction::SKNP(_) => todo!(),
            Instruction::LDD(vx) => {
                self.registers.set(*vx, self.delay_register.get())?;
            },
            Instruction::LDK(_) => todo!(),
            Instruction::LDDV(vx) => {
                let x = self.registers.get(*vx)?;
                self.delay_register.set(x);
            },
            Instruction::LDS(vx) => {
                let x = self.registers.get(*vx)?;
                self.sound_register.set(x);
            },
            Instruction::ADDI(vx) => {
                let x = self.registers.get(*vx)?;
                self.i += x as u16;
            },
            Instruction::LDF(vx) => {
                let x = self.registers.get(*vx)?;
                self.i = (x as u16 & 0xF) * 5;
            },
            Instruction::LDB(vx) => {
                let x = self.registers.get(*vx)?;
                let ones = x % 10;
                let tens = ((x % 100) - ones) / 10;
                let hundreds = ((x as u16 % 1000) as u8 - tens - ones) / 100;
                self.memory.set_byte(self.i, hundreds)?;
                self.memory.set_byte(self.i + 1, tens)?;
                self.memory.set_byte(self.i + 2, ones)?;
            },
            Instruction::LDIV => {
                self.memory.set_range(self.i, self.registers.as_bytes())?;
            },
            Instruction::LDVI => {
                self.registers.set_bytes(&self.memory.get_range(self.i, 16)?)?;
            },
        };
        Ok(())
    }
}