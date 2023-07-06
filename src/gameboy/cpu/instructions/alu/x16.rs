use crate::gameboy::cpu::instructions::util::{add16, increment16};
use crate::gameboy::cpu::Cpu;

/// INC DE
pub fn op_13(cpu: &mut Cpu) {
    cpu.regs.set_de(increment16(cpu.regs.de()));
}

/// ADD HL, DE
pub fn op_19(cpu: &mut Cpu) {
    let (new_hl, new_flags) = add16(cpu.regs.hl(), cpu.regs.de(), cpu.regs.f);
    cpu.regs.set_hl(new_hl);
    cpu.regs.f = new_flags;
}

/// INC HL
pub fn op_23(cpu: &mut Cpu) {
    cpu.regs.set_hl(increment16(cpu.regs.hl()));
}
