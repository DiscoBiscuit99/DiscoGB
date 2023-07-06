use crate::gameboy::cpu::Cpu;

/// JR i8
pub fn op_18(cpu: &mut Cpu) {
    let offset = cpu.next_byte() as i8;
    cpu.pc = cpu.pc.wrapping_add(offset as u16);
}

/// JR NZ, i8
pub fn op_20(cpu: &mut Cpu) {
    let offset = cpu.next_byte() as i8;
    if cpu.get_flag_z() == 0 {
        cpu.pc = cpu.pc.wrapping_add(offset as u16);
    }
}

/// JR Z, i8
pub fn op_28(cpu: &mut Cpu) {
    let offset = cpu.next_byte() as i8;
    if cpu.get_flag_z() != 0 {
        cpu.pc = cpu.pc.wrapping_add(offset as u16);
    }
}

/// RET
pub fn op_c9(cpu: &mut Cpu) {
    cpu.pc = cpu.pop_stack();
}

/// CALL u16
pub fn op_cd(cpu: &mut Cpu) {
    let word = cpu.next_word();
    cpu.push_stack(cpu.pc);
    cpu.pc = word;
}
