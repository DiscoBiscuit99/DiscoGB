use crate::gameboy::cpu::instructions::util::{add8, compare, decrement8, increment8, sbc, sub, xor};
use crate::gameboy::cpu::Cpu;

/// INC B
pub fn op_04(cpu: &mut Cpu) {
    let (new_b, new_flags) = increment8(cpu.regs.b, cpu.regs.f);
    cpu.regs.b = new_b;
    cpu.regs.f = new_flags;
}

/// DEC B
pub fn op_05(cpu: &mut Cpu) {
    let (new_b, new_flags) = decrement8(cpu.regs.b, cpu.regs.f);
    cpu.regs.b = new_b;
    cpu.regs.f = new_flags;
}

/// INC C
pub fn op_0c(cpu: &mut Cpu) {
    let (new_c, new_flags) = increment8(cpu.regs.c, cpu.regs.f);
    cpu.regs.c = new_c;
    cpu.regs.f = new_flags;
}

/// DEC C
pub fn op_0d(cpu: &mut Cpu) {
    let (new_c, new_flags) = decrement8(cpu.regs.c, cpu.regs.f);
    cpu.regs.c = new_c;
    cpu.regs.f = new_flags;
}

/// DEC D
pub fn op_15(cpu: &mut Cpu) {
    let (new_d, new_flags) = decrement8(cpu.regs.d, cpu.regs.f);
    cpu.regs.d = new_d;
    cpu.regs.f = new_flags;
}

/// INC E
pub fn op_1c(cpu: &mut Cpu) {
    let (new_e, new_flags) = increment8(cpu.regs.e, cpu.regs.f);
    cpu.regs.e = new_e;
    cpu.regs.f = new_flags;
}

/// DEC E
pub fn op_1d(cpu: &mut Cpu) {
    let (new_e, new_flags) = decrement8(cpu.regs.e, cpu.regs.f);
    cpu.regs.e = new_e;
    cpu.regs.f = new_flags;
}

/// INC H
pub fn op_24(cpu: &mut Cpu) {
    let (new_h, new_flags) = increment8(cpu.regs.h, cpu.regs.f);
    cpu.regs.h = new_h;
    cpu.regs.f = new_flags;
}

/// INC A
pub fn op_3c(cpu: &mut Cpu) {
    let (new_a, new_flags) = increment8(cpu.regs.a, cpu.regs.f);
    cpu.regs.a = new_a;
    cpu.regs.f = new_flags;
}

/// DEC A
pub fn op_3d(cpu: &mut Cpu) {
    let (new_a, new_flags) = decrement8(cpu.regs.a, cpu.regs.f);
    cpu.regs.a = new_a;
    cpu.regs.f = new_flags;
}

/// ADD A, (HL)
pub fn op_86(cpu: &mut Cpu) {
    let addr = cpu.regs.hl();
    let byte = cpu.mem.read().unwrap().read_byte(addr);
    let (new_a, new_flags) = add8(cpu.regs.a, byte, cpu.regs.f);
    cpu.regs.a = new_a;
    cpu.regs.f = new_flags;
}

/// SUB A, B
pub fn op_90(cpu: &mut Cpu) {
    let (new_a, new_flags) = sub(cpu.regs.a, cpu.regs.b, cpu.regs.f);
    cpu.regs.a = new_a;
    cpu.regs.f = new_flags;
}

/// SBC A, A
pub fn op_9f(cpu: &mut Cpu) {
    let (new_a, new_flags) = sbc(cpu.regs.a, cpu.regs.a, cpu.regs.f);
    cpu.regs.a = new_a;
    cpu.regs.f = new_flags;
}

/// XOR A, A
pub fn op_af(cpu: &mut Cpu) {
    let (new_a, new_flags) = xor(cpu.regs.a, cpu.regs.a, cpu.regs.f);
    cpu.regs.a = new_a;
    cpu.regs.f = new_flags;
}

/// CP A, (HL)
pub fn op_be(cpu: &mut Cpu) {
    let addr = cpu.regs.hl();
    let byte = cpu.mem.read().unwrap().read_byte(addr);
    cpu.regs.f = compare(cpu.regs.a, byte, cpu.regs.f);
}

/// CP A, u8
pub fn op_fe(cpu: &mut Cpu) {
    cpu.regs.f = compare(cpu.regs.a, cpu.next_byte(), cpu.regs.f);
}
