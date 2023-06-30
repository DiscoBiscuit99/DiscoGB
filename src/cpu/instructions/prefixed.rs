use crate::cpu::Cpu;

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
