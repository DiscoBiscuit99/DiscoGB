use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::memory::Memory;

mod instructions;
use instructions::*;

mod registers;
use registers::Registers;

/// Represents the CPU of the GameBoy.
#[derive(Debug, Clone)]
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
    mem: Arc<RwLock<Memory>>,
    /// The number of cycles that have elapsed.
    cycles: u32,
}

impl Cpu {
    /// Creates a new `Cpu` instance.
    pub fn new(mem: Arc<RwLock<Memory>>) -> Self {
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
        (self.regs.f & 0b1000_0000) >> 7
    }

    /// Returns the value of the `N` flag.
    pub fn get_flag_n(&self) -> u8 {
        (self.regs.f & 0b0100_0000) >> 6
    }

    /// Returns the value of the `H` flag.
    pub fn get_flag_h(&self) -> u8 {
        (self.regs.f & 0b0010_0000) >> 5
    }

    /// Returns the value of the `C` flag.
    pub fn get_flag_c(&self) -> u8 {
        (self.regs.f & 0b0001_0000) >> 4
    }

    /// Returns an opcode from the memory at the address of the program counter.
    fn fetch_next(&mut self) -> u8 {
        let byte = self.mem.try_read().unwrap().read_byte(self.pc);

        self.pc += 1;
        byte
    }

    /// Decodes an opcode into an instruction.
    fn decode(&mut self, opcode: u8, prev_pc: Option<u16>) -> Instruction {
        match opcode {
            0x00 => Instruction::normal(op_00, prev_pc.unwrap(), opcode, "NOP"),
            0x0e => Instruction::normal(op_0e, prev_pc.unwrap(), opcode, "LD C, u8"),
            0x3e => Instruction::normal(op_3e, prev_pc.unwrap(), opcode, "LD A, u8"),
            0x20 => Instruction::normal(op_20, prev_pc.unwrap(), opcode, "JR NZ, i8"),
            0x21 => Instruction::normal(op_21, prev_pc.unwrap(), opcode, "LD HL, u16"),
            0x31 => Instruction::normal(op_31, prev_pc.unwrap(), opcode, "LD SP, u16"),
            0x32 => Instruction::normal(op_32, prev_pc.unwrap(), opcode, "LD (HL-), A"),
            0x9f => Instruction::normal(op_9f, prev_pc.unwrap(), opcode, "SBC A, A"),
            0xaf => Instruction::normal(op_af, prev_pc.unwrap(), opcode, "XOR A, A"),
            0xfe => Instruction::normal(op_fe, prev_pc.unwrap(), opcode, "CP A, u8"),
            0xcb => {
                let opcode = self.fetch_next();
                match opcode {
                    0x7c => Instruction::prefixed(op_cb7c, prev_pc.unwrap(), opcode, "BIT 7, H"),
                    _ => panic!("Unknown (prefixed) opcode: {:#04x}", opcode),
                }
            }
            _ => panic!("Unknown opcode: {:#04x}", opcode),
        }
    }

    /// Executes an instruction.
    fn execute(&mut self, instr: Instruction) {
        (instr.execute)(self);
    }

    /// Simulates one step of the CPU.
    pub fn step(&mut self) {
        let prev_pc = self.pc;
        let opcode = self.fetch_next();
        let instr = self.decode(opcode, Some(prev_pc));

        let instr_info = format!(
            "PC: {:#06x} | Opcode: {:#04x} | Instruction: {}",
            instr.addr, instr.opcode, instr.description
        );

        if instr.is_prefixed {
            println!("{} (Special)", instr_info);
        } else {
            println!("{}", instr_info);
        }

        self.execute(instr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fetch_decode_execute() {
        let mem = Memory::new();
        let mut cpu = Cpu::new(Arc::new(RwLock::new(mem)));
        let prev_pc = cpu.pc;
        let opcode = cpu.fetch_next();
        let instr = cpu.decode(opcode, Some(prev_pc));
        cpu.execute(instr);
        assert_eq!(cpu.pc, 0x01);
    }
}
