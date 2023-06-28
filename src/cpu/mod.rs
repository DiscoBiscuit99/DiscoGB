use std::rc::Rc;
use std::cell::RefCell;

use crate::memory::Memory;

mod instructions;
use instructions::*;

mod registers;
use registers::Registers;

/// Represents the CPU of the GameBoy.
pub struct Cpu {
    /// The program counter.
    pub pc: u16,
    /// The stack pointer.
    pub sp: u16,
    /// The 8-bit registers.
    pub regs: Registers,
    /// The interrupt master enable flag.
    pub ime: bool,
    /// The halt flag.
    pub halt: bool,
    /// The memory.
    mem: Rc<RefCell<Memory>>,
    /// The number of cycles that have elapsed.
    cycles: u32,
}

impl Cpu {
    /// Creates a new `Cpu` instance.
    pub fn new(mem: Rc<RefCell<Memory>>) -> Self {
        Self {
            pc: 0,
            sp: 0,
            regs: Registers {
                a: 0,
                f: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
            },
            mem,
            cycles: 0,
            ime: false,
            halt: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }

    /// Returns the value of the `Z` flag.
    pub fn get_flag_z(&self) -> u8 {
        self.regs.f & 0b1000_0000
    }

    /// Returns the value of the `N` flag.
    pub fn get_flag_n(&self) -> u8 {
        self.regs.f & 0b0100_0000
    }

    /// Returns the value of the `H` flag.
    pub fn get_flag_h(&self) -> u8 {
        self.regs.f & 0b0010_0000
    }

    /// Returns the value of the `C` flag.
    pub fn get_flag_c(&self) -> u8 {
        self.regs.f & 0b0001_0000
    }

    /// Returns an opcode from the memory at the address of the program counter.
    fn fetch_next(&mut self) -> u8 {
        let byte = self.mem.borrow().read_byte(self.pc);
        self.pc += 1;
        byte
    }

    /// Decodes an opcode into an instruction.
    fn decode(&self, opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction::new(op00),
            0x31 => Instruction::new(op31),
            _ => {
                println!("Unknown opcode: {:#04x}", opcode);
                Instruction::new(op00)
            },
        }
    }

    /// Executes an instruction.
    fn execute(&mut self, instr: Instruction) {
        (instr.execute)(self);
    }
    
    /// Simulates one step of the CPU.
    pub fn step(&mut self) {
        let opcode = self.fetch_next();
        let instr = self.decode(opcode);
        self.execute(instr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fetch_decode_execute() {
        let mem = Memory::new();
        let mut cpu = Cpu::new(Rc::new(RefCell::new(mem)));
        let opcode = cpu.fetch_next();
        let instr = cpu.decode(opcode);
        cpu.execute(instr);
        assert_eq!(cpu.pc, 0x01);
    }
}
