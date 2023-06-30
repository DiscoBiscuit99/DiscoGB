use super::Cpu;

/// Represents an instruction of the GameBoy.
#[derive(Debug, Clone)]
pub struct Instruction {
    //cycles: u8,
    //length: u8,
    pub addr: u16,
    pub opcode: u8,
    pub description: String,
    pub is_prefixed: bool,
    pub execute: fn(&mut Cpu),
}

impl Instruction {
    /// Creates a new `Instruction`.
    pub fn new(
        execute: fn(&mut Cpu),
        addr: u16,
        opcode: u8,
        description: &str,
        is_prefixed: bool,
    ) -> Self {
        Self {
            addr,
            opcode,
            description: description.to_string(),
            is_prefixed,
            execute,
        }
    }

    /// Creates a new normal `Instruction`. (Shortcut)
    pub fn normal(execute: fn(&mut Cpu), addr: u16, opcode: u8, description: &str) -> Self {
        Self::new(execute, addr, opcode, description, false)
    }

    /// Creates a new prefixed `Instruction`. (Shortcut)
    pub fn prefixed(execute: fn(&mut Cpu), addr: u16, opcode: u8, description: &str) -> Self {
        Self::new(execute, addr, opcode, description, true)
    }
}

/// NOP
pub fn op_00(_cpu: &mut Cpu) {
    // NOP
}

/// JR NZ, i8
pub fn op_20(cpu: &mut Cpu) {
    let addr = cpu.pc;
    let byte = cpu.mem.read().unwrap().read_byte(addr);

    // Consume the byte
    cpu.pc += 1;

    if cpu.get_flag_z() == 0 {
        cpu.pc = addr.wrapping_add(byte as i8 as u16);
    }
}

/// LD HL, u16
pub fn op_21(cpu: &mut Cpu) {
    let addr = cpu.pc;
    let word = cpu.mem.read().unwrap().read_word(addr);

    // Consume the byte
    cpu.pc += 2;

    cpu.regs.set_hl(word);
}

/// LD SP, u16
pub fn op_31(cpu: &mut Cpu) {
    let addr = cpu.pc;
    cpu.sp = cpu.mem.read().unwrap().read_word(addr);

    // Consume the byte
    cpu.pc += 2;
}

/// LD (HL-), A
pub fn op_32(cpu: &mut Cpu) {
    let addr = cpu.regs.hl();
    let a = cpu.regs.a;

    cpu.mem.write().unwrap().write_byte(addr, a);

    cpu.regs.set_hl(addr.wrapping_sub(1));
}

/// SBC A, A
pub fn op_9f(cpu: &mut Cpu) {
    let a = cpu.regs.a;
    let b = cpu.regs.a;
    let c = cpu.get_flag_c();

    cpu.regs.a = a.wrapping_sub(b).wrapping_sub(c);

    // Set the `Z` flag if `a - b - c = 0`. (zero)
    if a.wrapping_sub(b).wrapping_sub(c) == 0 {
        cpu.regs.f |= 0b1000_0000;
    } else {
        cpu.regs.f &= 0b0111_1111;
    }

    // Set the `N` flag. (subtraction)
    cpu.regs.f |= 0b0100_0000;

    // Set the `H` flag if `a` is less than the lower 4 bits of `b + c`. (half carry)
    if a & 0x0F < b & 0x0F + c {
        cpu.regs.f |= 0b0010_0000;
    } else {
        cpu.regs.f &= 0b1101_1111;
    }

    // Set the `C` flag if `a` is less than `b + c`. (carry)
    if a < b + c {
        cpu.regs.f |= 0b0001_0000;
    } else {
        cpu.regs.f &= 0b1110_1111;
    }
}

/// XOR A, A
pub fn op_af(cpu: &mut Cpu) {
    cpu.regs.a ^= cpu.regs.a;

    // Set the `Z` flag. (zero)
    cpu.regs.f |= 0b1000_0000;

    // Unset the `N` flag. (negative)
    cpu.regs.f &= 0b1011_1111;

    // Unset the `H` flag. (half carry)
    cpu.regs.f &= 0b1101_1111;

    // Unset the `C` flag. (carry)
    cpu.regs.f &= 0b1110_1111;
}

/// CP A, u8
pub fn op_fe(cpu: &mut Cpu) {
    let addr = cpu.pc;
    let byte = cpu.mem.read().unwrap().read_byte(addr);

    // Consume the byte
    cpu.pc += 1;

    let a = cpu.regs.a;

    // Set the `Z` flag if `a - byte = 0`. (zero)
    if a == byte {
        cpu.regs.f |= 0b1000_0000;
    } else {
        cpu.regs.f &= 0b0111_1111;
    }

    // Set the `N` flag. (subtraction)
    cpu.regs.f |= 0b0100_0000;

    // Set the `H` flag if `a` is less than the lower 4 bits of `byte`. (half carry)
    if a & 0x0F < byte & 0x0F {
        cpu.regs.f |= 0b0010_0000;
    } else {
        cpu.regs.f &= 0b1101_1111;
    }

    // Set the `C` flag if `a` is less than `byte`. (carry)
    if a < byte {
        cpu.regs.f |= 0b0001_0000;
    } else {
        cpu.regs.f &= 0b1110_1111;
    }
}

/// BIT 7, H
/// Checks bit 7 (counting from zero) of the `H` register.
pub fn op_cb7c(cpu: &mut Cpu) {
    // Set the `Z` flag if the bit is 0. (zero)
    if cpu.regs.h & 0b1000_0000 == 0 {
        cpu.regs.f |= 0b1000_0000;
    } else {
        cpu.regs.f &= 0b0111_1111;
    }

    // Unset the `N` flag. (negative)
    cpu.regs.f &= 0b1011_1111;

    // Set the `H` flag. (half carry)
    cpu.regs.f |= 0b0010_0000;
}
