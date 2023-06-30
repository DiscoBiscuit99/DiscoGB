use crate::cpu::Cpu;

/// INC C
pub fn op_0c(cpu: &mut Cpu) {
    let c = cpu.regs.c;
    cpu.regs.c = c.wrapping_add(1);

    // Set the `Z` flag if `c + 1 = 0`. (zero)
    if c.wrapping_add(1) == 0 {
        cpu.regs.f |= 0b1000_0000;
    } else {
        cpu.regs.f &= 0b0111_1111;
    }

    // Unset the `N` flag. (subtraction)
    cpu.regs.f &= 0b1011_1111;

    // Set the `H` flag if the lower 4 bits of `c + 1` are less than the lower 4 bits of `c`. (half carry)
    if c & 0x0f < (c.wrapping_add(1) & 0x0f) {
        cpu.regs.f |= 0b0010_0000;
    } else {
        cpu.regs.f &= 0b1101_1111;
    }
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
    if a & 0x0f < b & 0x0f + c {
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
    if a & 0x0f < byte & 0x0f {
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
