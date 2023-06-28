/// The 8-bit registers of the GameBoy.
pub struct Registers {
    pub a: u8, pub f: u8, // Can also be used as the 16-bit register `AF`.
    pub b: u8, pub c: u8, // Can also be used as the 16-bit register `BC`.
    pub d: u8, pub e: u8, // Can also be used as the 16-bit register `DE`.
    pub h: u8, pub l: u8, // Can also be used as the 16-bit register `HL`.
}

impl Registers {
    /// Returns the value of the `AF` register.
    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }

    /// Returns the value of the `BC` register.
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    /// Returns the value of the `DE` register.
    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    /// Returns the value of the `HL` register.
    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    /// Sets the value of the `AF` register.
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00ff) as u8;
    }

    /// Sets the value of the `BC` register.
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00ff) as u8;
    }

    /// Sets the value of the `DE` register.
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00ff) as u8;
    }

    /// Sets the value of the `HL` register.
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00ff) as u8;
    }
}
