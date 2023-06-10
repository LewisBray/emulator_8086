use crate::registers::*;
use crate::memory::*;

pub fn je(registers: &mut Registers, memory: &mut Memory) {
    registers.ip += 1;  // Don't need to read first byte as lookup table says what instruction we are
    let offset: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;

    if registers.flags & ZF_FLAG_BIT != 0 {
        registers.ip = registers.ip.wrapping_add(offset as u16);
    }

    println!("je {}", offset);
}

pub fn jne(registers: &mut Registers, memory: &mut Memory) {
    registers.ip += 1;  // Don't need to read first byte as lookup table says what instruction we are
    let offset: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;

    if registers.flags & ZF_FLAG_BIT == 0 {
        registers.ip = registers.ip.wrapping_add(offset as u16);
    }

    println!("jne {}", offset);
}

pub fn loopnz(registers: &mut Registers, memory: &mut Memory) {
    registers.ip += 1;  // Don't need to read first byte as lookup table says what instruction we are
    let offset: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;

    if registers.flags & ZF_FLAG_BIT == 0 {
        registers.ip = registers.ip.wrapping_add(offset as u16);
    }

    println!("loopnz {}", offset);
}

pub fn loopz(registers: &mut Registers, memory: &mut Memory) {
    registers.ip += 1;  // Don't need to read first byte as lookup table says what instruction we are
    let offset: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;

    if registers.flags & ZF_FLAG_BIT != 0 {
        registers.ip = registers.ip.wrapping_add(offset as u16);
    }

    println!("loopz {}", offset);
}

// loop is a keyword so can't name the isntruction that
pub fn loop_cx(registers: &mut Registers, memory: &mut Memory) {
    registers.ip += 1;  // Don't need to read first byte as lookup table says what instruction we are
    let offset: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;

    registers.cx -= 1;
    if registers.cx != 0 {
        registers.ip = registers.ip.wrapping_add(offset as u16);
    }

    println!("loop {}", offset);
}

pub fn jcxz(registers: &mut Registers, memory: &mut Memory) {
    registers.ip += 1;  // Don't need to read first byte as lookup table says what instruction we are
    let offset: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;

    if registers.cx == 0 {
        registers.ip = registers.ip.wrapping_add(offset as u16);
    }

    println!("jcxz {}", offset);
}
