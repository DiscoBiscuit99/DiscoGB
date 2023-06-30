use crate::cpu::Cpu;

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
