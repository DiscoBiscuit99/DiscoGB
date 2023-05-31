use std::rc::Rc;
use std::cell::RefCell;

use crate::memory::Memory;

mod instructions;
use instructions::*;

/// Represents an instruction of the GameBoy.
struct Instruction {
    //opcode: u8,
    //cycles: u8,
    //length: u8,
    execute: fn(&mut Cpu),
}

impl Instruction {
    /// Creates a new `Instruction`.
    fn new(execute: fn(&mut Cpu)) -> Self {
        Self {
            execute,
        }
    }
}

// The 8-bit registers of the GameBoy.
struct Registers {
    a: u8, f: u8, // Can also be used as the 16-bit register `AF`.
    b: u8, c: u8, // Can also be used as the 16-bit register `BC`.
    d: u8, e: u8, // Can also be used as the 16-bit register `DE`.
    h: u8, l: u8, // Can also be used as the 16-bit register `HL`.
}

impl Registers {
    /// Returns the value of the `AF` register.
    fn af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }

    /// Returns the value of the `BC` register.
    fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    /// Returns the value of the `DE` register.
    fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    /// Returns the value of the `HL` register.
    fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    /// Sets the value of the `AF` register.
    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00ff) as u8;
    }

    /// Sets the value of the `BC` register.
    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00ff) as u8;
    }

    /// Sets the value of the `DE` register.
    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00ff) as u8;
    }

    /// Sets the value of the `HL` register.
    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00ff) as u8;
    }
}

/// Represents the CPU of the GameBoy.
pub struct Cpu {
    /// The program counter.
    pc: u16,
    /// The stack pointer.
    sp: u16,
    /// The 8-bit registers.
    regs: Registers,
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
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
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
            _ => panic!("Unknown opcode: {:#04x}", opcode),
        }
    }

    /// Executes an instruction.
    fn execute(&mut self, instr: Instruction) {
        (instr.execute)(self);
    }
    
    /// Simulates one step of the CPU.
    fn step(&mut self) {
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
