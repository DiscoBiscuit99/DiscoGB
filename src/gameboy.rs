use std::rc::Rc;
use std::cell::RefCell;

use crate::memory::Memory;
use crate::cpu::Cpu;

/// A struct representing the GameBoy.
pub struct GameBoy {
    pub cpu: Rc<RefCell<Cpu>>,
    pub memory: Rc<RefCell<Memory>>,
}

impl GameBoy {
    /// Creates a new `GameBoy`.
    pub fn new() -> Self {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let cpu = Rc::new(RefCell::new(Cpu::new(memory.clone())));

        Self {
            cpu,
            memory,
        }
    }

    /// Runs the GameBoy.
    pub fn run(&mut self) {
        self.cpu.borrow_mut().run();
    }
}
