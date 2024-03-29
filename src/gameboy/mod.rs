use std::sync::{Arc, RwLock};

pub mod cpu;
use cpu::Cpu;

pub mod memory;
use memory::Memory;

/// A struct representing the GameBoy.
#[derive(Debug, Clone)]
pub struct GameBoy {
    pub cpu: Arc<RwLock<Cpu>>,
    pub memory: Arc<RwLock<Memory>>,
}

impl GameBoy {
    /// Creates a new `GameBoy`.
    pub fn new() -> Self {
        let memory = Arc::new(RwLock::new(Memory::new()));
        let cpu = Arc::new(RwLock::new(Cpu::new(memory.clone())));

        Self { cpu, memory }
    }

    /// Runs the GameBoy.
    pub fn run(&mut self) {
        self.cpu.write().unwrap().run();
    }
}
