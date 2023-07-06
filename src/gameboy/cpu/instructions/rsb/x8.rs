use crate::gameboy::cpu::instructions::util::{bit, rotate_left, rotate_left_through_carry};
use crate::gameboy::cpu::Cpu;

/// BIT 7, H
/// Checks bit 7 (counting from zero) of the `H` register.
pub fn op_cb7c(cpu: &mut Cpu) {
    cpu.regs.f = bit(7, cpu.regs.h, cpu.regs.f);
}

// RL C
pub fn op_cb11(cpu: &mut Cpu) {
    (cpu.regs.c, cpu.regs.f) = rotate_left_through_carry(cpu.regs.c, cpu.regs.f);
}

/// RLA
pub fn op_17(cpu: &mut Cpu) {
    (cpu.regs.a, cpu.regs.f) = rotate_left(cpu.regs.a, cpu.regs.f);
}
