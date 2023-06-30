use crate::cpu::Cpu;

/// JR NZ, i8
pub fn op_20(cpu: &mut Cpu) {
    let addr = cpu.pc;
    let byte = cpu.mem.read().unwrap().read_byte(addr);

    // Consume the byte
    cpu.pc += 1;

    if cpu.get_flag_z() == 0 {
        cpu.pc = addr.wrapping_add(byte as i8 as u16);
    }
}
