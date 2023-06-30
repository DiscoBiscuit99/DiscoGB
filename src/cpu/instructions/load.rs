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

/// LD L, u8
pub fn op_2e(cpu: &mut Cpu) {
    let addr = cpu.pc;
    let byte = cpu.mem.read().unwrap().read_byte(addr);

    // Consume the byte
    cpu.pc += 1;

    cpu.regs.l = byte;
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

/// LD (HL), A
pub fn op_77(cpu: &mut Cpu) {
    let addr = cpu.regs.hl();
    let a = cpu.regs.a;

    cpu.mem.write().unwrap().write_byte(addr, a);
}

/// LD (FF00+u8), A
pub fn op_e0(cpu: &mut Cpu) {
    let addr = 0xff00 + cpu.mem.read().unwrap().read_byte(cpu.pc) as u16;
    let a = cpu.regs.a;

    // Consume the byte
    cpu.pc += 1;

    cpu.mem.write().unwrap().write_byte(addr, a);
}

/// LD (FF00+C), A
pub fn op_e2(cpu: &mut Cpu) {
    let addr = 0xff00 + cpu.regs.c as u16;
    let a = cpu.regs.a;

    cpu.mem.write().unwrap().write_byte(addr, a);
}
