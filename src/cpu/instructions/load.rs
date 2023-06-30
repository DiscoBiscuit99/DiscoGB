use crate::cpu::Cpu;

/// LD C, u8
pub fn op_0e(cpu: &mut Cpu) {
    let addr = cpu.pc;
    let byte = cpu.mem.read().unwrap().read_byte(addr);

    // Consume the byte
    cpu.pc += 1;

    cpu.regs.c = byte;
}

/// LD A, u8
pub fn op_3e(cpu: &mut Cpu) {
    let addr = cpu.pc;
    let byte = cpu.mem.read().unwrap().read_byte(addr);

    // Consume the byte
    cpu.pc += 1;

    cpu.regs.a = byte;
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
