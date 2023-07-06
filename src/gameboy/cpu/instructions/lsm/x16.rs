use crate::gameboy::cpu::Cpu;

/// LD DE, u16
pub fn op_11(cpu: &mut Cpu) {
    let word = cpu.next_word();
    cpu.regs.set_de(word);
}

/// LD HL, u16
pub fn op_21(cpu: &mut Cpu) {
    let word = cpu.next_word();
    cpu.regs.set_hl(word);
}

/// LD SP, u16
pub fn op_31(cpu: &mut Cpu) {
    cpu.sp = cpu.next_word();
}

/// POP BC
pub fn op_c1(cpu: &mut Cpu) {
    let word = cpu.pop_stack();
    cpu.regs.set_bc(word);
}

/// PUSH BC
pub fn op_c5(cpu: &mut Cpu) {
    cpu.push_stack(cpu.regs.bc());
}
