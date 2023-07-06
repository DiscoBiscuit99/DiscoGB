use std::sync::{Arc, RwLock};

use super::memory::Memory;

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

    /// Sets the value of the `Z` flag.
    pub fn set_flag_z(&mut self, value: bool) {
        if value {
            self.regs.f |= 0b1000_0000;
        } else {
            self.regs.f &= 0b0111_1111;
        }
    }

    /// Sets the value of the `N` flag.
    pub fn set_flag_n(&mut self, value: bool) {
        if value {
            self.regs.f |= 0b0100_0000;
        } else {
            self.regs.f &= 0b1011_1111;
        }
    }

    /// Sets the value of the `H` flag.
    pub fn set_flag_h(&mut self, value: bool) {
        if value {
            self.regs.f |= 0b0010_0000;
        } else {
            self.regs.f &= 0b1101_1111;
        }
    }

    /// Sets the value of the `C` flag.
    pub fn set_flag_c(&mut self, value: bool) {
        if value {
            self.regs.f |= 0b0001_0000;
        } else {
            self.regs.f &= 0b1110_1111;
        }
    }

    /// Returns a byte from the memory at the address of the program counter.
    pub fn next_byte(&mut self) -> u8 {
        let byte = self.mem.try_read().unwrap().read_byte(self.pc);
        self.pc += 1;
        byte
    }

    /// Returns a word from the memory at the address of the program counter.
    pub fn next_word(&mut self) -> u16 {
        let word = self.mem.try_read().unwrap().read_word(self.pc);
        self.pc += 2;
        word
    }

    /// Decodes an opcode into an instruction.
    fn decode(&mut self, opcode: u8, prev_pc: Option<u16>) -> Instruction {
        match opcode {
            0x00 => Instruction::normal(op_00, prev_pc.unwrap(), opcode, "NOP"),
            // Arithmetic Instructions
            0x04 => Instruction::normal(op_04, prev_pc.unwrap(), opcode, "INC B"),
            0x05 => Instruction::normal(op_05, prev_pc.unwrap(), opcode, "DEC B"),
            0x0c => Instruction::normal(op_0c, prev_pc.unwrap(), opcode, "INC C"),
            0x0d => Instruction::normal(op_0d, prev_pc.unwrap(), opcode, "DEC C"),
            0x13 => Instruction::normal(op_13, prev_pc.unwrap(), opcode, "INC DE"),
            0x15 => Instruction::normal(op_15, prev_pc.unwrap(), opcode, "DEC D"),
            0x19 => Instruction::normal(op_19, prev_pc.unwrap(), opcode, "ADD HL, DE"),
            0x1c => Instruction::normal(op_1c, prev_pc.unwrap(), opcode, "INC E"),
            0x1d => Instruction::normal(op_1d, prev_pc.unwrap(), opcode, "DEC E"),
            0x23 => Instruction::normal(op_23, prev_pc.unwrap(), opcode, "INC HL"),
            0x24 => Instruction::normal(op_24, prev_pc.unwrap(), opcode, "INC H"),
            0x3c => Instruction::normal(op_3c, prev_pc.unwrap(), opcode, "INC A"),
            0x3d => Instruction::normal(op_3d, prev_pc.unwrap(), opcode, "DEC A"),
            0x86 => Instruction::normal(op_86, prev_pc.unwrap(), opcode, "ADD A, (HL)"),
            0x90 => Instruction::normal(op_90, prev_pc.unwrap(), opcode, "SUB A, B"),
            0x9f => Instruction::normal(op_9f, prev_pc.unwrap(), opcode, "SBC A, A"),
            0xaf => Instruction::normal(op_af, prev_pc.unwrap(), opcode, "XOR A, A"),
            0xbe => Instruction::normal(op_be, prev_pc.unwrap(), opcode, "CP A, (HL)"),
            0xfe => Instruction::normal(op_fe, prev_pc.unwrap(), opcode, "CP A, u8"),

            // Load/Store/Move Instructions
            0x0e => Instruction::normal(op_0e, prev_pc.unwrap(), opcode, "LD C, u8"),
            0x06 => Instruction::normal(op_06, prev_pc.unwrap(), opcode, "LD B, u8"),
            0x11 => Instruction::normal(op_11, prev_pc.unwrap(), opcode, "LD DE, u16"),
            0x16 => Instruction::normal(op_16, prev_pc.unwrap(), opcode, "LD D, u8"),
            0x1a => Instruction::normal(op_1a, prev_pc.unwrap(), opcode, "LD A, (DE)"),
            0x1e => Instruction::normal(op_1e, prev_pc.unwrap(), opcode, "LD E, u8"),
            0x21 => Instruction::normal(op_21, prev_pc.unwrap(), opcode, "LD HL, u16"),
            0x22 => Instruction::normal(op_22, prev_pc.unwrap(), opcode, "LD (HL+), A"),
            0x2e => Instruction::normal(op_2e, prev_pc.unwrap(), opcode, "LD L, u8"),
            0x31 => Instruction::normal(op_31, prev_pc.unwrap(), opcode, "LD SP, u16"),
            0x32 => Instruction::normal(op_32, prev_pc.unwrap(), opcode, "LD (HL-), A"),
            0x3e => Instruction::normal(op_3e, prev_pc.unwrap(), opcode, "LD A, u8"),
            0x4f => Instruction::normal(op_4f, prev_pc.unwrap(), opcode, "LD C, A"),
            0x57 => Instruction::normal(op_57, prev_pc.unwrap(), opcode, "LD D, A"),
            0x67 => Instruction::normal(op_67, prev_pc.unwrap(), opcode, "LD H, A"),
            0x77 => Instruction::normal(op_77, prev_pc.unwrap(), opcode, "LD (HL), A"),
            0x78 => Instruction::normal(op_78, prev_pc.unwrap(), opcode, "LD A, B"),
            0x7b => Instruction::normal(op_7b, prev_pc.unwrap(), opcode, "LD A, E"),
            0x7c => Instruction::normal(op_7c, prev_pc.unwrap(), opcode, "LD A, H"),
            0x7d => Instruction::normal(op_7d, prev_pc.unwrap(), opcode, "LD A, L"),
            0xc1 => Instruction::normal(op_c1, prev_pc.unwrap(), opcode, "POP BC"),
            0xc5 => Instruction::normal(op_c5, prev_pc.unwrap(), opcode, "PUSH BC"),
            0xe0 => Instruction::normal(op_e0, prev_pc.unwrap(), opcode, "LD (FF00 + u8), A"),
            0xe2 => Instruction::normal(op_e2, prev_pc.unwrap(), opcode, "LD (FF00 + C), A"),
            0xea => Instruction::normal(op_ea, prev_pc.unwrap(), opcode, "LD (u16), A"),
            0xf0 => Instruction::normal(op_f0, prev_pc.unwrap(), opcode, "LD A, (FF00 + u8)"),

            // Control Instructions
            0x18 => Instruction::normal(op_18, prev_pc.unwrap(), opcode, "JR i8"),
            0x20 => Instruction::normal(op_20, prev_pc.unwrap(), opcode, "JR NZ, i8"),
            0x28 => Instruction::normal(op_28, prev_pc.unwrap(), opcode, "JR Z, i8"),
            0xc9 => Instruction::normal(op_c9, prev_pc.unwrap(), opcode, "RET"),
            0xcd => Instruction::normal(op_cd, prev_pc.unwrap(), opcode, "CALL u16"),

            // Rotate/Shift/Bitwise Instructions
            0x17 => Instruction::normal(op_17, prev_pc.unwrap(), opcode, "RLA"),
            0xcb => {
                let opcode = self.next_byte();
                match opcode {
                    0x11 => Instruction::prefixed(op_cb11, prev_pc.unwrap(), opcode, "RL C"),
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
        let opcode = self.next_byte();
        let instr = self.decode(opcode, Some(prev_pc));

        let instr_info = format!(
            "PC: {:#06x} | Opcode: {:#04x} | Instruction: {}",
            instr.addr, instr.opcode, instr.description
        );

        if instr.is_prefixed {
            println!("{} (Prefixed)", instr_info);
        } else {
            println!("{}", instr_info);
        }

        self.execute(instr);
    }

    /// Pushes a value onto the stack.
    pub fn push_stack(&mut self, value: u16) {
        self.sp -= 2;
        self.mem.write().unwrap().write_word(self.sp, value);
    }

    /// Pops a value off the stack.
    pub fn pop_stack(&mut self) -> u16 {
        let value = self.mem.read().unwrap().read_word(self.sp);
        self.sp += 2;
        value
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
        let opcode = cpu.next_byte();
        let instr = cpu.decode(opcode, Some(prev_pc));
        cpu.execute(instr);
        assert_eq!(cpu.pc, 0x01);
    }
}
