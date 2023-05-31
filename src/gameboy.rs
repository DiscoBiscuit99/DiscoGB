use std::rc::Rc;
use std::cell::RefCell;

use crate::memory::Memory;
use crate::cpu::Cpu;

/// A struct representing the GameBoy.
pub struct GameBoy {
    cpu: Cpu,
    memory: Rc<RefCell<Memory>>,
}

impl GameBoy {
    /// Creates a new `GameBoy`.
    pub fn new() -> Self {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let cpu = Cpu::new(memory.clone());

        Self {
            cpu,
            memory,
        }
    }

    /// Runs the GameBoy.
    pub fn run(&mut self) {
        self.cpu.run();
    }
}
