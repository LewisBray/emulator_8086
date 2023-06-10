pub type Memory = [u8; u16::MAX as usize];

pub fn store_byte(memory: &mut Memory, address: u16, byte: u8) {
    debug_assert!((address as usize) < memory.len());

    memory[address as usize] = byte;
}

pub fn store_word(memory: &mut Memory, address: u16, word: u16) {
    debug_assert!((address as usize + 1) < memory.len());

    let word_low: u8 = (word & 0x00FF) as u8;
    memory[address as usize] = word_low;

    let word_high: u8 = ((word & 0xFF00) >> 8) as u8;
    memory[address as usize + 1] = word_high;
}

pub fn load_byte(memory: &Memory, address: u16) -> u8 {
    debug_assert!((address as usize) < memory.len());

    let byte: u8 = memory[address as usize];
    
    return byte;
}

pub fn load_word(memory: &Memory, address: u16) -> u16 {
    debug_assert!((address as usize + 1) < memory.len());

    let word_low: u8 = memory[address as usize];
    let word_high: u8 = memory[address as usize + 1];

    let word: u16 = ((word_high as u16) << 8) + (word_low as u16);

    return word;
}

pub fn grab_instruction_byte(memory: &Memory, ip: &mut u16) -> u8 {
    let byte: u8 = load_byte(memory, *ip);
    *ip += 1;
    
    return byte;
}

pub fn grab_instruction_word(memory: &Memory, ip: &mut u16) -> u16 {
    let word: u16 = load_word(memory, *ip);
    *ip += 2;

    return word;
}
