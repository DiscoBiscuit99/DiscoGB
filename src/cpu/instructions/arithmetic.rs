use crate::cpu::Cpu;

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
