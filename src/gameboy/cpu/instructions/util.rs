/// Utility function for comparing two bytes.
pub fn compare(reg: u8, byte: u8, mut flags: u8) -> u8 {
    let result = reg.wrapping_sub(byte);

    // Subtract Flag: set, as this is a subtraction operation
    flags |= 0b0100_0000;

    // Zero Flag: set if the result is 0
    if result == 0 {
        flags |= 0b1000_0000;
    } else {
        flags &= 0b0111_1111;
    }

    // Half-Carry Flag: set if there's a borrow from bit 4
    if (reg ^ byte ^ result) & 0x10 == 0x10 {
        flags |= 0b0010_0000;
    } else {
        flags &= 0b1101_1111;
    }

    // Carry Flag: set if there's a borrow from bit 8
    if result > reg {
        flags |= 0b0001_0000;
    } else {
        flags &= 0b1110_1111;
    }

    flags
}

/// Utility function for rotating left.
pub fn rotate_left(reg: u8, mut flags: u8) -> (u8, u8) {
    let new_carry = (reg & 0b1000_0000) >> 7;
    let new_reg = (reg << 1) | new_carry;

    //// Zero Flag: set if new register value is 0
    //if new_reg == 0 {
    //    flags |= 0b1000_0000;
    //} else {
    //    flags &= 0b0111_1111;
    //}

    // Zero Flag: unset
    flags &= 0b0111_1111;

    // Subtract Flag: cleared as this is a rotate operation
    flags &= 0b1011_1111;

    // Half-Carry Flag: cleared as this is a rotate operation
    flags &= 0b1101_1111;

    // Carry Flag: set to the old 7th bit of the register
    if new_carry == 1 {
        flags |= 0b0001_0000;
    } else {
        flags &= 0b1110_1111;
    }

    (new_reg, flags)
}

/// Utility function for rotating right.
pub fn rotate_right(reg: u8, mut flags: u8) -> (u8, u8) {
    let new_carry = reg & 0b0000_0001;
    let new_reg = (reg >> 1) | (new_carry << 7);

    // Zero Flag: set if new register value is 0
    if new_reg == 0 {
        flags |= 0b1000_0000;
    } else {
        flags &= 0b0111_1111;
    }

    // Subtract Flag: cleared as this is a rotate operation
    flags &= 0b1011_1111;

    // Half-Carry Flag: cleared as this is a rotate operation
    flags &= 0b1101_1111;

    // Carry Flag: set to the old 0th bit of the register
    if new_carry == 1 {
        flags |= 0b0001_0000;
    } else {
        flags &= 0b1110_1111;
    }

    (new_reg, flags)
}

/// Utility function for rotating left through carry.
pub fn rotate_left_through_carry(reg: u8, mut flags: u8) -> (u8, u8) {
    let old_carry = (flags & 0b0001_0000) >> 4;
    let new_carry = (reg & 0b1000_0000) >> 7;
    let new_reg = (reg << 1) | old_carry;

    // Zero Flag: set if new register value is 0
    if new_reg == 0 {
        flags |= 0b1000_0000;
    } else {
        flags &= 0b0111_1111;
    }

    // Subtract Flag: cleared as this is a rotate operation
    flags &= 0b1011_1111;

    // Half-Carry Flag: cleared as this is a rotate operation
    flags &= 0b1101_1111;

    // Carry Flag: set to the old 7th bit of the register
    if new_carry == 1 {
        flags |= 0b0001_0000;
    } else {
        flags &= 0b1110_1111;
    }

    (new_reg, flags)
}

/// Utility function for rotating right through carry.
pub fn rotate_right_through_carry(reg: u8, mut flags: u8) -> (u8, u8) {
    let old_carry = (flags & 0b0001_0000) << 3;
    let new_carry = reg & 0b0000_0001;
    let new_reg = (reg >> 1) | old_carry;

    // Zero Flag: set if new register value is 0
    if new_reg == 0 {
        flags |= 0b1000_0000;
    } else {
        flags &= 0b0111_1111;
    }

    // Subtract Flag: cleared as this is a rotate operation
    flags &= 0b1011_1111;

    // Half-Carry Flag: cleared as this is a rotate operation
    flags &= 0b1101_1111;

    // Carry Flag: set to the old 0th bit of the register
    if new_carry == 1 {
        flags |= 0b0001_0000;
    } else {
        flags &= 0b1110_1111;
    }

    (new_reg, flags)
}

/// Utility function for XOR'ing two bytes.
/// Returns the resulting value and flags.
pub fn xor(reg: u8, byte: u8, mut flags: u8) -> (u8, u8) {
    let result = reg ^ byte;

    // Zero Flag: set if result is 0
    if result == 0 {
        flags |= 0b1000_0000;
    } else {
        flags &= 0b0111_1111;
    }

    // Subtract Flag: cleared as this is a XOR operation
    flags &= 0b1011_1111;

    // Half-Carry Flag: cleared as this is a XOR operation
    flags &= 0b1101_1111;

    // Carry Flag: cleared as this is a XOR operation
    flags &= 0b1110_1111;

    (result, flags)
}

/// Utility function for incrementing a register.
/// Returns the resulting value and flags.
pub fn increment8(reg: u8, mut flags: u8) -> (u8, u8) {
    let result = reg.wrapping_add(1);

    // Zero Flag: set if result is 0
    if result == 0 {
        flags |= 0b1000_0000;
    } else {
        flags &= 0b0111_1111;
    }

    // Half-Carry Flag: set if carry from bit 3
    if (reg & 0xf) + 1 > 0xf {
        flags |= 0b0010_0000;
    } else {
        flags &= 0b1101_1111;
    }

    // Subtract Flag: cleared as this is an increment operation
    flags &= 0b1011_1111;

    // Carry Flag: unchanged, not affected by increment operation

    (result, flags)
}

/// Utility function for incrementing a double register.
/// Returns the resulting value and flags.
pub fn increment16(reg: u16) -> u16 {
    reg.wrapping_add(1)
}

/// Utility function for decrementing a register.
/// Returns the resulting value and flags.
pub fn decrement8(reg: u8, mut flags: u8) -> (u8, u8) {
    let result = reg.wrapping_sub(1);

    // Zero Flag: set if result is 0
    if result == 0 {
        flags |= 0b1000_0000;
    } else {
        flags &= 0b0111_1111;
    }

    // Half-Carry Flag: set if there's a borrow from bit 4
    if (reg & 0xf) == 0 {
        flags |= 0b0010_0000;
    } else {
        flags &= 0b1101_1111;
    }

    // Subtract Flag: set as this is a decrement operation
    flags |= 0b0100_0000;

    // Carry Flag: unchanged, not affected by decrement operation

    (result, flags)
}

/// Utility function for decrementing a double register.
/// Returns the resulting value and flags.
pub fn decrement16(reg: u16) -> u16 {
    reg.wrapping_sub(1)
}

/// Utility function for adding a register and a byte.
pub fn add8(reg: u8, byte: u8) -> (u8, u8) {
    let result = reg.wrapping_add(byte);

    // Subtract Flag: cleared as this is an addition operation
    let mut flags = 0b0000_0000;

    // Zero Flag: set if the result is 0
    if result == 0 {
        flags |= 0b1000_0000;
    } else {
        flags &= 0b0111_1111;
    }

    // Half-Carry Flag: set if there's a carry from bit 3
    if (reg & 0xf) + (byte & 0xf) > 0xf {
        flags |= 0b0010_0000;
    } else {
        flags &= 0b1101_1111;
    }

    // Carry Flag: set if there's a carry from bit 7
    if (reg as u16) + (byte as u16) > 0xff {
        flags |= 0b0001_0000;
    } else {
        flags &= 0b1110_1111;
    }

    (result, flags)
}

/// Utility function for adding a register and a word.
pub fn add16(reg: u16, word: u16, mut flags: u8) -> (u16, u8) {
    let result = reg.wrapping_add(word);

    // Subtract Flag: cleared as this is an addition operation
    flags &= 0b1011_1111;

    // Half-Carry Flag: set if there's a carry from bit 11
    if (reg & 0xfff) + (word & 0xfff) > 0xfff {
        flags |= 0b0010_0000;
    } else {
        flags &= 0b1101_1111;
    }

    // Carry Flag: set if there's a carry from bit 15
    if (reg as u32) + (word as u32) > 0xffff {
        flags |= 0b0001_0000;
    } else {
        flags &= 0b1110_1111;
    }

    (result, flags)
}

/// Utility function for subtracting a byte from a register.
pub fn sub(reg: u8, byte: u8, mut flags: u8) -> (u8, u8) {
    let result = reg.wrapping_sub(byte);

    // Subtract Flag: set, as this is a subtraction operation
    flags |= 0b0100_0000;

    // Zero Flag: set if the result is 0
    if result == 0 {
        flags |= 0b1000_0000;
    } else {
        flags &= 0b0111_1111;
    }

    // Half-Carry Flag: set if there's a borrow from bit 4
    if (reg ^ byte ^ result) & 0x10 == 0x10 {
        flags |= 0b0010_0000;
    } else {
        flags &= 0b1101_1111;
    }

    // Carry Flag: set if there's a borrow
    if (reg as u16) < (byte as u16) {
        flags |= 0b0001_0000;
    } else {
        flags &= 0b1110_1111;
    }

    (result, flags)
}

/// Utility function for subtracting a byte plus the carry flag from a register.
pub fn sbc(reg: u8, byte: u8, mut flags: u8) -> (u8, u8) {
    let carry = (flags & 0b0001_0000) >> 4;
    let result = reg.wrapping_sub(byte).wrapping_sub(carry);

    // Subtract Flag: set, as this is a subtraction operation
    flags |= 0b0100_0000;

    // Zero Flag: set if the result is 0
    if result == 0 {
        flags |= 0b1000_0000;
    } else {
        flags &= 0b0111_1111;
    }

    // Half-Carry Flag: set if there's a borrow from bit 4
    if (reg ^ byte ^ result) & 0x10 == 0x10 {
        flags |= 0b0010_0000;
    } else {
        flags &= 0b1101_1111;
    }

    // Carry Flag: set if there's a borrow from bit 8
    if result > reg {
        flags |= 0b0001_0000;
    } else {
        flags &= 0b1110_1111;
    }

    (result, flags)
}

/// Utility function for testing a specified bit of a register.
pub fn bit(bit: u8, reg: u8, mut flags: u8) -> u8 {
    // Test specified bit
    if (reg & (1 << bit)) == 0 {
        // If bit is 0, set the zero flag
        flags |= 0b1000_0000;
    } else {
        // If bit is 1, reset the zero flag
        flags &= 0b0111_1111;
    }

    // Reset the subtract flag
    flags &= 0b1011_1111;

    // Set the half-carry flag
    flags |= 0b0010_0000;

    flags
}
