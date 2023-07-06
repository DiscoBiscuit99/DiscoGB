use crate::gameboy::cpu::Cpu;

/// LD B, u8
pub fn op_06(cpu: &mut Cpu) {
    cpu.regs.b = cpu.next_byte();
}

/// LD D, u8
pub fn op_16(cpu: &mut Cpu) {
    cpu.regs.d = cpu.next_byte();
}

/// LD A, (DE)
pub fn op_1a(cpu: &mut Cpu) {
    let addr = cpu.regs.de();
    cpu.regs.a = cpu.mem.read().unwrap().read_byte(addr);
}

/// LD E, u8
pub fn op_1e(cpu: &mut Cpu) {
    cpu.regs.e = cpu.next_byte();
}

/// LD (HL+), A
pub fn op_22(cpu: &mut Cpu) {
    let addr = cpu.regs.hl();
    let a = cpu.regs.a;
    cpu.mem.write().unwrap().write_byte(addr, a);
    cpu.regs.set_hl(addr.wrapping_add(1));
}

/// LD (HL-), A
pub fn op_32(cpu: &mut Cpu) {
    let addr = cpu.regs.hl();
    let a = cpu.regs.a;
    cpu.mem.write().unwrap().write_byte(addr, a);
    cpu.regs.set_hl(addr.wrapping_sub(1));
}

/// LD C, A
pub fn op_4f(cpu: &mut Cpu) {
    cpu.regs.c = cpu.regs.a;
}

/// LD D, A
pub fn op_57(cpu: &mut Cpu) {
    cpu.regs.d = cpu.regs.a;
}

/// LD H, A
pub fn op_67(cpu: &mut Cpu) {
    cpu.regs.h = cpu.regs.a;
}

/// LD (HL), A
pub fn op_77(cpu: &mut Cpu) {
    let addr = cpu.regs.hl();
    let a = cpu.regs.a;
    cpu.mem.write().unwrap().write_byte(addr, a);
}

/// LD A, B
pub fn op_78(cpu: &mut Cpu) {
    cpu.regs.a = cpu.regs.b;
}

/// LD A, E
pub fn op_7b(cpu: &mut Cpu) {
    cpu.regs.a = cpu.regs.e;
}

/// LD A, H
pub fn op_7c(cpu: &mut Cpu) {
    cpu.regs.a = cpu.regs.h;
}

/// LD A, L
pub fn op_7d(cpu: &mut Cpu) {
    cpu.regs.a = cpu.regs.l;
}

/// LD (FF00+u8), A
pub fn op_e0(cpu: &mut Cpu) {
    let addr = 0xff00 + cpu.next_byte() as u16;
    let a = cpu.regs.a;
    cpu.mem.write().unwrap().write_byte(addr, a);
}

/// LD (FF00+C), A
pub fn op_e2(cpu: &mut Cpu) {
    let addr = 0xff00 + cpu.regs.c as u16;
    let a = cpu.regs.a;
    cpu.mem.write().unwrap().write_byte(addr, a);
}

/// LD (u16), A
pub fn op_ea(cpu: &mut Cpu) {
    let addr = cpu.next_word();
    let a = cpu.regs.a;
    cpu.mem.write().unwrap().write_byte(addr, a);
}

/// LD C, u8
pub fn op_0e(cpu: &mut Cpu) {
    cpu.regs.c = cpu.next_byte();
}

/// LD L, u8
pub fn op_2e(cpu: &mut Cpu) {
    cpu.regs.l = cpu.next_byte();
}

/// LD A, u8
pub fn op_3e(cpu: &mut Cpu) {
    cpu.regs.a = cpu.next_byte();
}

/// LD A, (FF00+u8)
pub fn op_f0(cpu: &mut Cpu) {
    let addr = 0xff00 + cpu.next_byte() as u16;
    cpu.regs.a = cpu.mem.read().unwrap().read_byte(addr);
}
