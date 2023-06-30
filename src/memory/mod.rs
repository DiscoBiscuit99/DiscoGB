const ROM_SIZE: u16 = 0x8000;
const VRAM_SIZE: u16 = 0x2000;
const ERAM_SIZE: u16 = 0x2000;
const WRAM_SIZE: u16 = 0x2000;
const ECHO_SIZE: u16 = 0x1e00;
const OAM_SIZE: u16 = 0xa0;
const IO_SIZE: u16 = 0x80;
const HRAM_SIZE: u16 = 0x7f;

const ROM_ADDR: u16 = 0x0000;
const VRAM_ADDR: u16 = 0x8000;
const ERAM_ADDR: u16 = 0xa000;
const WRAM_ADDR: u16 = 0xc000;
const ECHO_ADDR: u16 = 0xe000;
const OAM_ADDR: u16 = 0xfe00;
const UNUSED_ADDR: u16 = 0xfea0;
const IO_ADDR: u16 = 0xFF00;
const HRAM_ADDR: u16 = 0xff80;
const IE_ADDR: u16 = 0xffff;

const ROM_ADDR_END: u16 = 0x7fff;
const VRAM_ADDR_END: u16 = 0x9fff;
const ERAM_ADDR_END: u16 = 0xbfff;
const WRAM_ADDR_END: u16 = 0xdfff;
const ECHO_ADDR_END: u16 = 0xfdff;
const OAM_ADDR_END: u16 = 0xFe9f;
const UNUSED_ADDR_END: u16 = 0xfeff;
const IO_ADDR_END: u16 = 0xff7f;
const HRAM_ADDR_END: u16 = 0xfffe;

const BOOT_ROM: [u8; 256] = [
    0x31, 0xfe, 0xff, 0xaf, 0x21, 0xff, 0x9f, 0x32, 0xcb, 0x7c, 0x20, 0xfb, 0x21, 0x26, 0xff, 0x0e,
    0x11, 0x3e, 0x80, 0x32, 0xe2, 0x0c, 0x3e, 0xf3, 0xe2, 0x32, 0x3e, 0x77, 0x77, 0x3e, 0xfc, 0xe0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1a, 0xcd, 0x95, 0x00, 0xcd, 0x96, 0x00, 0x13, 0x7b,
    0xfe, 0x34, 0x20, 0xf3, 0x11, 0xd8, 0x00, 0x06, 0x08, 0x1a, 0x13, 0x22, 0x23, 0x05, 0x20, 0xf9,
    0x3e, 0x19, 0xea, 0x10, 0x99, 0x21, 0x2f, 0x99, 0x0e, 0x0c, 0x3d, 0x28, 0x08, 0x32, 0x0d, 0x20,
    0xf9, 0x2e, 0x0f, 0x18, 0xf3, 0x67, 0x3e, 0x64, 0x57, 0xe0, 0x42, 0x3e, 0x91, 0xe0, 0x40, 0x04,
    0x1e, 0x02, 0x0e, 0x0c, 0xf0, 0x44, 0xfe, 0x90, 0x20, 0xfa, 0x0d, 0x20, 0xf7, 0x1d, 0x20, 0xf2,
    0x0e, 0x13, 0x24, 0x7c, 0x1e, 0x83, 0xfe, 0x62, 0x28, 0x06, 0x1e, 0xc1, 0xfe, 0x64, 0x20, 0x06,
    0x7b, 0xe2, 0x0c, 0x3e, 0x87, 0xe2, 0xf0, 0x42, 0x90, 0xe0, 0x42, 0x15, 0x20, 0xd2, 0x05, 0x20,
    0x4f, 0x16, 0x20, 0x18, 0xcb, 0x4f, 0x06, 0x04, 0xc5, 0xcb, 0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17,
    0x05, 0x20, 0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9, 0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e,
    0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc,
    0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e, 0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c,
    0x21, 0x04, 0x01, 0x11, 0xa8, 0x00, 0x1a, 0x13, 0xbe, 0x20, 0xfe, 0x23, 0x7d, 0xfe, 0x34, 0x20,
    0xf5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xfb, 0x86, 0x20, 0xfe, 0x3e, 0x01, 0xe0, 0x50,
];

/// Represents a banked memory region.
#[derive(Debug, Clone)]
pub struct BankedMemory {
    pub bank0: Vec<u8>,
    pub bankn: Vec<u8>,
    size: u16,
    offset: u16,
}

impl BankedMemory {
    /// Creates a new banked memory region with the given size.
    fn new(size: u16, offset: u16) -> Self {
        Self {
            bank0: vec![0; size as usize / 2],
            bankn: vec![0; size as usize / 2],
            size,
            offset,
        }
    }

    /// Returns the byte at the given address, taking into account the offset.
    pub fn read(&self, addr: u16) -> u8 {
        if addr - self.offset < self.size / 2 {
            self.bank0[addr as usize]
        } else {
            self.bankn[(addr - self.offset - self.size / 2) as usize]
        }
    }

    /// Writes the given value at the given address, taking into account the offset.
    fn write(&mut self, addr: u16, value: u8) {
        if addr - self.offset < self.size / 2 {
            self.bank0[addr as usize] = value;
        } else {
            self.bankn[(addr - self.offset - self.size / 2) as usize] = value;
        }
    }
}

/// Represents the memory of the GameBoy.
#[derive(Debug, Clone)]
pub struct Memory {
    pub rom: BankedMemory,
    pub vram: [u8; VRAM_SIZE as usize],
    pub eram: [u8; ERAM_SIZE as usize],
    pub wram: BankedMemory,
    pub echo: [u8; ECHO_SIZE as usize],
    pub oam: [u8; OAM_SIZE as usize],
    pub io: [u8; IO_SIZE as usize],
    pub hram: [u8; HRAM_SIZE as usize],
    pub ie: u8,
}

impl Memory {
    /// Creates a new `Memory` instance.
    pub fn new() -> Self {
        let mut mem = Self {
            rom: BankedMemory::new(ROM_SIZE, ROM_ADDR),
            vram: [1; VRAM_SIZE as usize],
            eram: [0; ERAM_SIZE as usize],
            wram: BankedMemory::new(WRAM_SIZE, WRAM_ADDR),
            echo: [0; ECHO_SIZE as usize],
            oam: [0; OAM_SIZE as usize],
            io: [0; IO_SIZE as usize],
            hram: [0; HRAM_SIZE as usize],
            ie: 0,
        };
        mem.init();
        mem
    }

    /// Initialize the memory.
    pub fn init(&mut self) {
        for (addr, byte) in BOOT_ROM.iter().enumerate() {
            self.rom.write(addr as u16, *byte);
        }
    }

    /// Reads a byte from the given address.
    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            ROM_ADDR..=ROM_ADDR_END => self.rom.read(addr),
            VRAM_ADDR..=VRAM_ADDR_END => self.vram[translate_addr(addr, VRAM_ADDR)],
            ERAM_ADDR..=ERAM_ADDR_END => self.eram[translate_addr(addr, ERAM_ADDR)],
            WRAM_ADDR..=WRAM_ADDR_END => self.wram.read(addr),
            ECHO_ADDR..=ECHO_ADDR_END => self.echo[translate_addr(addr, ECHO_ADDR)],
            OAM_ADDR..=OAM_ADDR_END => self.oam[translate_addr(addr, OAM_ADDR)],
            UNUSED_ADDR..=UNUSED_ADDR_END => panic!(
                "Attempted to read from unused memory at address {:#06x}",
                addr
            ),
            IO_ADDR..=IO_ADDR_END => self.io[translate_addr(addr, IO_ADDR)],
            HRAM_ADDR..=HRAM_ADDR_END => self.hram[translate_addr(addr, HRAM_ADDR)],
            IE_ADDR => self.ie,
        }
    }

    /// Reads a word from the given address.
    pub fn read_word(&self, addr: u16) -> u16 {
        let low = self.read_byte(addr);
        let high = self.read_byte(addr + 1);
        ((high as u16) << 8) | low as u16
    }

    /// Writes a byte to the given address.
    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            ROM_ADDR..=ROM_ADDR_END => panic!("Attempted to write to ROM at address {:#06x}", addr),
            VRAM_ADDR..=VRAM_ADDR_END => self.vram[translate_addr(addr, VRAM_ADDR)] = value,
            ERAM_ADDR..=ERAM_ADDR_END => self.eram[translate_addr(addr, ERAM_ADDR)] = value,
            WRAM_ADDR..=WRAM_ADDR_END => self.wram.write(addr, value),
            ECHO_ADDR..=ECHO_ADDR_END => self.echo[translate_addr(addr, ECHO_ADDR)] = value,
            OAM_ADDR..=OAM_ADDR_END => self.oam[translate_addr(addr, OAM_ADDR)] = value,
            UNUSED_ADDR..=UNUSED_ADDR_END => {
                panic!(
                    "Attempted to write to unused memory at address {:#06x}",
                    addr
                );
            }
            IO_ADDR..=IO_ADDR_END => self.io[translate_addr(addr, IO_ADDR)] = value,
            HRAM_ADDR..=HRAM_ADDR_END => self.hram[translate_addr(addr, HRAM_ADDR)] = value,
            IE_ADDR => self.ie = value,
        }
    }
}

/// Subtracts the offset from the given address and returns the result as a usize.
fn translate_addr(addr: u16, offset: u16) -> usize {
    (addr - offset) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_byte() {
        let mut mem = Memory::new();
        mem.write_byte(VRAM_ADDR, 0x12);
        assert_eq!(mem.read_byte(VRAM_ADDR), 0x12);
    }

    #[test]
    #[should_panic(expected = "Attempted to write to ROM at address 0x0000")]
    fn test_write_byte_rom() {
        let mut mem = Memory::new();
        mem.write_byte(ROM_ADDR, 0x12);
    }

    #[test]
    #[should_panic(expected = "Attempted to read from unused memory at address 0xfea0")]
    fn test_read_byte_unused() {
        let mem = Memory::new();
        mem.read_byte(UNUSED_ADDR);
    }

    #[test]
    #[should_panic(expected = "Attempted to write to unused memory at address 0xfea0")]
    fn test_write_byte_unused() {
        let mut mem = Memory::new();
        mem.write_byte(UNUSED_ADDR, 0x12);
    }
}
