use super::Cpu;

/// Represents an instruction of the GameBoy.
pub struct Instruction {
    //opcode: u8,
    //cycles: u8,
    //length: u8,
    pub execute: fn(&mut Cpu),
}

impl Instruction {
    /// Creates a new `Instruction`.
    pub fn new(execute: fn(&mut Cpu)) -> Self {
        Self {
            execute,
        }
    }
}

/// The `NOP` instruction.
pub fn op00<'a>(_cpu: &'a mut Cpu) {
    // NOP
}

/// The `LD SP, u16` instruction.
pub fn op31(cpu: &mut Cpu) {
    let addr = cpu.pc;
    cpu.sp = cpu.mem.borrow().read_word(addr);
}
