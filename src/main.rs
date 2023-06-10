mod registers;
mod memory;
mod data_transfer;
mod arithmetic;
mod control_transfer;
mod mode;

use registers::*;
use memory::*;
use data_transfer::*;
use arithmetic::*;
use control_transfer::*;

use std::env;
use std::fs;

fn unimplemented_op(_registers: &mut Registers, _memory: &mut Memory) {
    debug_assert!(false);
}

type Op = fn(&mut Registers, &mut Memory);
const OPS: &'static [Op; 256] = &[
    // 0x00
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_imm_to_acc_8_bit,
    arithmetic_imm_to_acc_16_bit,
    unimplemented_op,
    unimplemented_op,

    // 0x08
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_imm_to_acc_8_bit,
    arithmetic_imm_to_acc_16_bit,
    unimplemented_op,
    unimplemented_op,

    // 0x10
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_imm_to_acc_8_bit,
    arithmetic_imm_to_acc_16_bit,
    unimplemented_op,
    unimplemented_op,

    // 0x18
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_imm_to_acc_8_bit,
    arithmetic_imm_to_acc_16_bit,
    unimplemented_op,
    unimplemented_op,

    // 0x20
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_imm_to_acc_8_bit,
    arithmetic_imm_to_acc_16_bit,
    unimplemented_op,
    unimplemented_op,

    // 0x28
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_imm_to_acc_8_bit,
    arithmetic_imm_to_acc_16_bit,
    unimplemented_op,
    unimplemented_op,

    // 0x30
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_imm_to_acc_8_bit,
    arithmetic_imm_to_acc_16_bit,
    unimplemented_op,
    unimplemented_op,

    // 0x38
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_mem_reg_with_reg_to_either_8_bit,
    arithmetic_mem_reg_with_reg_to_either_16_bit,
    arithmetic_imm_to_acc_8_bit,
    arithmetic_imm_to_acc_16_bit,
    unimplemented_op,
    unimplemented_op,

    // 0x40
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0x48
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0x50
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0x58
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0x60
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0x68
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0x70
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,

    // 0x78
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,
    decode_conditional_jump_encoding,

    // 0x80
    arithmetic_imm_to_reg_mem_8_bit,
    arithmetic_imm_to_reg_mem_16_bit,
    arithmetic_imm_to_reg_mem_8_bit,
    arithmetic_imm_to_reg_mem_16_bit,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0x88
    mov_mem_reg_to_from_reg_8_bit,
    mov_mem_reg_to_from_reg_16_bit,
    mov_mem_reg_to_from_reg_8_bit,
    mov_mem_reg_to_from_reg_16_bit,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0x90
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0x98
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0xA0
    mov_mem_to_acc_8_bit,
    mov_mem_to_acc_16_bit,
    mov_acc_to_mem_8_bit,
    mov_acc_to_mem_16_bit,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0xA8
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0xB0
    mov_imm_to_reg_8_bit,
    mov_imm_to_reg_8_bit,
    mov_imm_to_reg_8_bit,
    mov_imm_to_reg_8_bit,
    mov_imm_to_reg_8_bit,
    mov_imm_to_reg_8_bit,
    mov_imm_to_reg_8_bit,
    mov_imm_to_reg_8_bit,

    // 0xB8
    mov_imm_to_reg_16_bit,
    mov_imm_to_reg_16_bit,
    mov_imm_to_reg_16_bit,
    mov_imm_to_reg_16_bit,
    mov_imm_to_reg_16_bit,
    mov_imm_to_reg_16_bit,
    mov_imm_to_reg_16_bit,
    mov_imm_to_reg_16_bit,

    // 0xC0
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    mov_imm_to_reg_mem_8_bit,
    mov_imm_to_reg_mem_16_bit,

    // 0xC8
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0xD0
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0xD8
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0xE0
    decode_loop_encoding,
    decode_loop_encoding,
    decode_loop_encoding,
    decode_loop_encoding,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0xE8
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0xF0
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,

    // 0xF8
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op,
    unimplemented_op
];

fn main() {
    let input_file = env::args().nth(1).expect("Please specify an input file");
    let machine_code: Vec<u8> = fs::read(input_file).expect("Missing instruction stream file");

    let mut registers = Registers::default();

    let mut memory: Memory = [0; u16::MAX as usize];
    memory[0..machine_code.len()].copy_from_slice(&machine_code);

    println!("bits 16");

    let byte_count: usize = machine_code.len();
    while (registers.ip as usize) < byte_count {
        let byte: u8 = machine_code[registers.ip as usize];
        let op: Op = OPS[byte as usize];
        op(&mut registers, &mut memory);
    }

    dbg!(registers);
}
