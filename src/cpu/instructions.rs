use super::Cpu;

/// The `NOP` instruction.
pub fn op00<'a>(_cpu: &'a mut Cpu) {
    // NOP
}

/// The `LD SP, u16` instruction.
pub fn op31(cpu: &mut Cpu) {
    let addr = cpu.pc;
    cpu.sp = cpu.mem.borrow().read_word(addr);
}
