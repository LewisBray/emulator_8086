use crate::registers::*;
use crate::memory::*;

const CONDITIONAL_JUMP_INSTRUCTION_ENCODINGS: &'static [&str] = &[
    "jo", "jno", "jb", "jnb", "je", "jne", "jbe", "ja",
    "js", "jns", "jp", "jnp", "jl", "jnl", "jle", "jg"
];

fn unimplemented_conditional_jump_instruction_test(_flags_register: u16) -> bool {
    debug_assert!(false);
    return false;
}

fn je_test(flags_register: u16) -> bool {
    return flags_register & ZF_FLAG_BIT != 0;
}

fn jne_test(flags_register: u16) -> bool {
    return flags_register & ZF_FLAG_BIT == 0;
}

type ConditionalJumpInstructionTestFn = fn(u16) -> bool;
const CONDITIONAL_JUMP_INSTRUCTION_TEST_FNS: &'static [ConditionalJumpInstructionTestFn] = &[
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    je_test,
    jne_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test,
    unimplemented_conditional_jump_instruction_test
];

pub fn decode_conditional_jump_encoding(registers: &mut Registers, memory: &mut Memory) {
    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);

    let instruction_index: usize = (byte & 0x0F) as usize;
    debug_assert!(instruction_index < CONDITIONAL_JUMP_INSTRUCTION_ENCODINGS.len());

    let instruction: &str = CONDITIONAL_JUMP_INSTRUCTION_ENCODINGS[instruction_index];

    let offset: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;

    let jump_test: ConditionalJumpInstructionTestFn = CONDITIONAL_JUMP_INSTRUCTION_TEST_FNS[instruction_index];
    if jump_test(registers.flags) {
        registers.ip = registers.ip.wrapping_add(offset as u16);
    }

    println!("{} {}", instruction, offset);
}

const LOOP_INSTRUCTION_ENCODINGS: &'static [&str] = &[
    "loopnz", "loopz", "loop", "jcxz"
];

fn loopnz_test(registers: &mut Registers) -> bool {
    return registers.flags & ZF_FLAG_BIT != 0;
}

fn loopz_test(registers: &mut Registers) -> bool {
    return registers.flags & ZF_FLAG_BIT == 0;
}

fn loop_test(_registers: &mut Registers) -> bool {
    return true;
}

fn jcxz_test(registers: &mut Registers) -> bool {
    return registers.cx == 0;
}

type LoopInstructionTestFn = fn(&mut Registers) -> bool;
const LOOP_INSTRUCTION_TEST_FNS: &'static [LoopInstructionTestFn] = &[
    loopnz_test, loopz_test, loop_test, jcxz_test
];

pub fn decode_loop_encoding(registers: &mut Registers, memory: &mut Memory) {
    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);

    let instruction_index: usize = (byte & 0x03) as usize;
    debug_assert!(instruction_index < LOOP_INSTRUCTION_ENCODINGS.len());

    let instruction: &str = LOOP_INSTRUCTION_ENCODINGS[instruction_index];

    let offset: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;

    let loop_test: LoopInstructionTestFn = LOOP_INSTRUCTION_TEST_FNS[instruction_index];
    if loop_test(registers) {
        registers.ip = registers.ip.wrapping_add(offset as u16);
    }

    println!("{} {}", instruction, offset);
}
