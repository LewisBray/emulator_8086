#[derive(Debug, Default)]
pub struct Registers {
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,
    pub sp: u16,
    pub bp: u16,
    pub si: u16,
    pub di: u16,
    pub ip: u16,
    pub flags: u16
}

pub const ZF_FLAG_BIT: u16 = 0x0040;
pub const SF_FLAG_BIT: u16 = 0x0080;

pub fn set_low_byte(word: u16, byte: u8) -> u16 {
    return (word & 0xFF00) + byte as u16;
}

pub fn set_high_byte(word: u16, byte: u8) -> u16 {
    return (word & 0x00FF) + ((byte as u16) << 8);    
}

pub fn get_low_byte(word: u16) -> u8 {
    return (word & 0x00FF) as u8;
}

pub fn get_high_byte(word: u16) -> u8 {
    return ((word & 0xFF00) >> 8) as u8;
}

pub const REG_FIELD_ENCODINGS_8_BIT: &'static [&str] = &[
    "al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"
];

pub const REG_FIELD_ENCODINGS_16_BIT: &'static [&str] = &[
    "ax", "cx", "dx", "bx", "sp", "bp", "si", "di"
];

pub fn set_8_bit_register(registers: &mut Registers, field_index: u8, value: u8) {
    match field_index {
        0 => { registers.ax = set_low_byte(registers.ax, value); },
        1 => { registers.cx = set_low_byte(registers.cx, value); },
        2 => { registers.dx = set_low_byte(registers.dx, value); },
        3 => { registers.bx = set_low_byte(registers.bx, value); },
        4 => { registers.ax = set_high_byte(registers.ax, value); },
        5 => { registers.cx = set_high_byte(registers.cx, value); },
        6 => { registers.dx = set_high_byte(registers.dx, value); },
        7 => { registers.bx = set_high_byte(registers.bx, value); },
        _  => {
            debug_assert!(false);
        }
    }    
}

pub fn set_16_bit_register(registers: &mut Registers, field_index: u8, value: u16) {
    match field_index {
        0 => { registers.ax = value; },
        1 => { registers.cx = value; },
        2 => { registers.dx = value; },
        3 => { registers.bx = value; },
        4 => { registers.sp = value; },
        5 => { registers.bp = value; },
        6 => { registers.si = value; },
        7 => { registers.di = value; },
        _  => {
            debug_assert!(false);
        }
    }
}

pub fn get_8_bit_register(registers: &Registers, field_index: u8) -> u8 {
    match field_index {
        0 => { return get_low_byte(registers.ax); },
        1 => { return get_low_byte(registers.cx); },
        2 => { return get_low_byte(registers.dx); },
        3 => { return get_low_byte(registers.bx); },
        4 => { return get_high_byte(registers.ax); },
        5 => { return get_high_byte(registers.cx); },
        6 => { return get_high_byte(registers.dx); },
        7 => { return get_high_byte(registers.bx); },
        _  => {
            debug_assert!(false);
            return 0;
        }
    }    
}

pub fn get_16_bit_register(registers: &Registers, field_index: u8) -> u16 {
    match field_index {
        0 => { return registers.ax; },
        1 => { return registers.cx; },
        2 => { return registers.dx; },
        3 => { return registers.bx; },
        4 => { return registers.sp; },
        5 => { return registers.bp; },
        6 => { return registers.si; },
        7 => { return registers.di; },
        _  => {
            debug_assert!(false);
            return 0;
        }
    }
}

pub const REG_EXPRESSION_ENCODINGS: &'static [&str] = &[
    "bx + si",
    "bx + di",
    "bp + si",
    "bp + di",
    "si",
    "di",
    "bp",
    "bx"
];

pub fn calculate_reg_expression(registers: &Registers, expression_index: u8) -> u16 {
    match expression_index {
        0 => { return registers.bx.wrapping_add(registers.si); },
        1 => { return registers.bx.wrapping_add(registers.di); },
        2 => { return registers.bp.wrapping_add(registers.si); },
        3 => { return registers.bp.wrapping_add(registers.di); },
        4 => { return registers.si; },
        5 => { return registers.di; },
        6 => { return registers.bp; },
        7 => { return registers.bx; },
        _ => {
            debug_assert!(false);
            return 0;
        }
    }
}
