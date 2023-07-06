use super::Cpu;

mod lsm;
pub use lsm::{x16::*, x8::*};

mod control;
pub use control::{br::*, misc::*};

mod alu;
pub use alu::{x16::*, x8::*};

mod rsb;
pub use rsb::x8::*;

pub mod util;

/// Represents an instruction of the GameBoy.
#[derive(Debug, Clone)]
pub struct Instruction {
    //cycles: u8,
    //length: u8,
    pub addr: u16,
    pub opcode: u8,
    pub description: String,
    pub is_prefixed: bool,
    pub execute: fn(&mut Cpu),
}

impl Instruction {
    /// Creates a new `Instruction`.
    pub fn new(
        execute: fn(&mut Cpu),
        addr: u16,
        opcode: u8,
        description: &str,
        is_prefixed: bool,
    ) -> Self {
        Self {
            addr,
            opcode,
            description: description.to_string(),
            is_prefixed,
            execute,
        }
    }

    /// Creates a new normal `Instruction`. (Shortcut)
    pub fn normal(execute: fn(&mut Cpu), addr: u16, opcode: u8, description: &str) -> Self {
        Self::new(execute, addr, opcode, description, false)
    }

    /// Creates a new prefixed `Instruction`. (Shortcut)
    pub fn prefixed(execute: fn(&mut Cpu), addr: u16, opcode: u8, description: &str) -> Self {
        Self::new(execute, addr, opcode, description, true)
    }
}
