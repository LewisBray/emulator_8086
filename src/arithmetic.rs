use crate::registers::*;
use crate::memory::*;
use crate::mode::*;

const ARITHMETIC_INSTRUCTION_ENCODINGS: &'static [&str] = &[
    "add", "or", "adc", "sbb", "and", "sub", "xor", "cmp"
];

fn set_bit(mut x: u16, bit_flag: u16, value: bool) -> u16 {
    if value {
        x |= bit_flag;
    } else {
        x &= !bit_flag;
    }

    return x;
}

fn update_flags_register_16_bit(mut flags_register: u16, result: u16) -> u16 {
    let is_zero: bool = result == 0;
    flags_register = set_bit(flags_register, ZF_FLAG_BIT, is_zero);

    let is_signed: bool = result & 0x8000 != 0;
    flags_register = set_bit(flags_register, SF_FLAG_BIT, is_signed);

    return flags_register;
}

fn update_flags_register_8_bit(mut flags_register: u16, result: u8) -> u16 {
    let is_zero: bool = result == 0;
    flags_register = set_bit(flags_register, ZF_FLAG_BIT, is_zero);

    let is_signed: bool = result & 0x80 != 0;
    flags_register = set_bit(flags_register, SF_FLAG_BIT, is_signed);

    return flags_register;
}

fn add_op_16_bit(x: u16, y: u16, flags_register: &mut u16) -> u16 {
    let result: u16 = x.wrapping_add(y);
    *flags_register = update_flags_register_16_bit(*flags_register, result);
    return result;
}

fn or_op_16_bit(x: u16, y: u16, flags_register: &mut u16) -> u16 {
    let result: u16 = x | y;
    *flags_register = update_flags_register_16_bit(*flags_register, result);
    return result;
}

fn and_op_16_bit(x: u16, y: u16, flags_register: &mut u16) -> u16 {
    let result: u16 = x & y;
    *flags_register = update_flags_register_16_bit(*flags_register, result);
    return result;
}

fn sub_op_16_bit(x: u16, y: u16, flags_register: &mut u16) -> u16 {
    let result: u16 = x.wrapping_sub(y);
    *flags_register = update_flags_register_16_bit(*flags_register, result);
    return result;
}

fn xor_op_16_bit(x: u16, y: u16, flags_register: &mut u16) -> u16 {
    let result: u16 = x ^ y;
    *flags_register = update_flags_register_16_bit(*flags_register, result);
    return result;
}

fn cmp_op_16_bit(x: u16, y: u16, flags_register: &mut u16) {
    let result: u16 = x.wrapping_sub(y);
    *flags_register = update_flags_register_16_bit(*flags_register, result);
}

fn arithmetic_op_with_reg_16_bit(registers: &mut Registers, field_index: u8, instruction_index: u8, immediate: u16) {
    let field_value: u16 = get_16_bit_register(registers, field_index);

    match instruction_index {
        0 => {
            let add_result: u16 = add_op_16_bit(field_value, immediate, &mut registers.flags);
            set_16_bit_register(registers, field_index, add_result);
        },
        1 => {
            let or_result: u16 = or_op_16_bit(field_value, immediate, &mut registers.flags);
            set_16_bit_register(registers, field_index, or_result);
        },
        2 => {
            debug_assert!(false);
        },
        3 => {
            debug_assert!(false);
        },
        4 => {
            let and_result: u16 = and_op_16_bit(field_value, immediate, &mut registers.flags);
            set_16_bit_register(registers, field_index, and_result);
        },
        5 => {
            let sub_result: u16 = sub_op_16_bit(field_value, immediate, &mut registers.flags);
            set_16_bit_register(registers, field_index, sub_result);
        },
        6 => {
            let xor_result: u16 = xor_op_16_bit(field_value, immediate, &mut registers.flags);
            set_16_bit_register(registers, field_index, xor_result);
        },
        7 => {
            cmp_op_16_bit(field_value, immediate, &mut registers.flags);
        },
        _ => {
            debug_assert!(false);
        }
    }
}

fn arithmetic_op_with_mem_16_bit(registers: &mut Registers, memory: &mut Memory, address: u16, instruction_index: u8, immediate: u16) {
    let word: u16 = load_word(memory, address);

    match instruction_index {
        0 => {
            let add_result: u16 = add_op_16_bit(word, immediate, &mut registers.flags);
            store_word(memory, address, add_result);
        },
        1 => {
            let or_result: u16 = or_op_16_bit(word, immediate, &mut registers.flags);
            store_word(memory, address, or_result);
        },
        2 => {
            debug_assert!(false);
        },
        3 => {
            debug_assert!(false);
        },
        4 => {
            let and_result: u16 = and_op_16_bit(word, immediate, &mut registers.flags);
            store_word(memory, address, and_result);
        },
        5 => {
            let sub_result: u16 = sub_op_16_bit(word, immediate, &mut registers.flags);
            store_word(memory, address, sub_result);
        },
        6 => {
            let xor_result: u16 = xor_op_16_bit(word, immediate, &mut registers.flags);
            store_word(memory, address, xor_result);
        },
        7 => {
            cmp_op_16_bit(word, immediate, &mut registers.flags);
        },
        _ => {
            debug_assert!(false);
        }
    }
}

fn add_op_8_bit(x: u8, y: u8, flags_register: &mut u16) -> u8 {
    let result: u8 = x.wrapping_add(y);
    *flags_register = update_flags_register_8_bit(*flags_register, result);
    return result;
}

fn or_op_8_bit(x: u8, y: u8, flags_register: &mut u16) -> u8 {
    let result: u8 = x | y;
    *flags_register = update_flags_register_8_bit(*flags_register, result);
    return result;
}

fn and_op_8_bit(x: u8, y: u8, flags_register: &mut u16) -> u8 {
    let result: u8 = x & y;
    *flags_register = update_flags_register_8_bit(*flags_register, result);
    return result;
}

fn sub_op_8_bit(x: u8, y: u8, flags_register: &mut u16) -> u8 {
    let result: u8 = x.wrapping_sub(y);
    *flags_register = update_flags_register_8_bit(*flags_register, result);
    return result;
}

fn xor_op_8_bit(x: u8, y: u8, flags_register: &mut u16) -> u8 {
    let result: u8 = x ^ y;
    *flags_register = update_flags_register_8_bit(*flags_register, result);
    return result;
}

fn cmp_op_8_bit(x: u8, y: u8, flags_register: &mut u16) {
    let result: u8 = x.wrapping_sub(y);
    *flags_register = update_flags_register_8_bit(*flags_register, result);
}

fn arithmetic_op_with_reg_8_bit(registers: &mut Registers, field_index: u8, instruction_index: u8, immediate: u8) {
    let field_value: u8 = get_8_bit_register(registers, field_index);

    match instruction_index {
        0 => {
            let add_result: u8 = add_op_8_bit(field_value, immediate, &mut registers.flags);
            set_8_bit_register(registers, field_index, add_result);
        },
        1 => {
            let or_result: u8 = or_op_8_bit(field_value, immediate, &mut registers.flags);
            set_8_bit_register(registers, field_index, or_result);
        },
        2 => {
            debug_assert!(false);
        },
        3 => {
            debug_assert!(false);
        },
        4 => {
            let and_result: u8 = and_op_8_bit(field_value, immediate, &mut registers.flags);
            set_8_bit_register(registers, field_index, and_result);
        },
        5 => {
            let sub_result: u8 = sub_op_8_bit(field_value, immediate, &mut registers.flags);
            set_8_bit_register(registers, field_index, sub_result);
        },
        6 => {
            let xor_result: u8 = xor_op_8_bit(field_value, immediate, &mut registers.flags);
            set_8_bit_register(registers, field_index, xor_result);
        },
        7 => {
            cmp_op_8_bit(field_value, immediate, &mut registers.flags);
        },
        _ => {
            debug_assert!(false);
        }
    }
}

fn arithmetic_op_with_mem_8_bit(registers: &mut Registers, memory: &mut Memory, address: u16, instruction_index: u8, immediate: u8) {
    let byte: u8 = load_byte(memory, address);

    match instruction_index {
        0 => {
            let add_result: u8 = add_op_8_bit(byte, immediate, &mut registers.flags);
            store_byte(memory, address, add_result);
        },
        1 => {
            let or_result: u8 = or_op_8_bit(byte, immediate, &mut registers.flags);
            store_byte(memory, address, or_result);
        },
        2 => {
            debug_assert!(false);
        },
        3 => {
            debug_assert!(false);
        },
        4 => {
            let and_result: u8 = and_op_8_bit(byte, immediate, &mut registers.flags);
            store_byte(memory, address, and_result);
        },
        5 => {
            let sub_result: u8 = sub_op_8_bit(byte, immediate, &mut registers.flags);
            store_byte(memory, address, sub_result);
        },
        6 => {
            let xor_result: u8 = xor_op_8_bit(byte, immediate, &mut registers.flags);
            store_byte(memory, address, xor_result);
        },
        7 => {
            cmp_op_8_bit(byte, immediate, &mut registers.flags);
        },
        _ => {
            debug_assert!(false);
        }
    }
}

pub fn decode_arithmetic_mem_reg_with_reg_to_either_encoding(registers: &mut Registers, memory: &mut Memory) {
    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);

    let instruction_index: u8 = (byte & 0x38) >> 3;
    let d_bit: u8 = (byte & 0x02) >> 1;
    let w_bit: u8 = byte & 0x01;

    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);

    let mod_field: u8 = (byte & 0xC0) >> 6;
    let reg_field: u8 = (byte & 0x38) >> 3;
    let rm_field: u8 = byte & 0x07;

    if w_bit == 1 {
        match mod_field {
            MODE_MEM_NO_DISP => {
                if rm_field == 6 {
                    let address: u16 = grab_instruction_word(memory, &mut registers.ip);
                    if d_bit == 1 {
                        let immediate: u16 = load_word(memory, address);
                        arithmetic_op_with_reg_16_bit(registers, reg_field, instruction_index, immediate);

                        let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} {}, [{}]", instruction, field, address);
                    } else {
                        let immediate: u16 = get_16_bit_register(registers, reg_field);
                        arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate);

                        let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} [{}], {}", instruction, address, field);
                    }
                } else {
                    let address: u16 = calculate_reg_expression(registers, rm_field);
                    if d_bit == 1 {
                        let immediate: u16 = load_word(memory, address);
                        arithmetic_op_with_reg_16_bit(registers, reg_field, instruction_index, immediate);

                        let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                        let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} {}, [{}]", instruction, field, expression);
                    } else {
                        let immediate: u16 = get_16_bit_register(registers, reg_field);
                        arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate);
    
                        let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                        let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} [{}], {}", instruction, expression, field);
                    }
                }
            },
            MODE_MEM_8_BIT_DISP => {
                let displacement: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                if d_bit == 1 {
                    let immediate: u16 = load_word(memory, address);
                    arithmetic_op_with_reg_16_bit(registers, reg_field, instruction_index, immediate);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} {}, [{} {} {}]", instruction, field, expression, sign, displacement.abs());
                    } else {
                        println!("{} {}, [{}]", instruction, field, expression);
                    }
                } else {
                    let immediate: u16 = get_16_bit_register(registers, reg_field);
                    arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} [{} {} {}], {}", instruction, expression, sign, displacement.abs(), field);
                    } else {
                        println!("{} [{}], {}", instruction, expression, field);
                    }
                }
            },
            MODE_MEM_16_BIT_DISP => {
                let displacement: i16 = grab_instruction_word(memory, &mut registers.ip) as i16;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                if d_bit == 1 {
                    let immediate: u16 = load_word(memory, address);
                    arithmetic_op_with_reg_16_bit(registers, reg_field, instruction_index, immediate);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} {}, [{} {} {}]", instruction, field, expression, sign, displacement.abs());
                    } else {
                        println!("{} {}, [{}]", instruction, field, expression);
                    }
                } else {
                    let immediate: u16 = get_16_bit_register(registers, reg_field);
                    arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} [{} {} {}], {}", instruction, expression, sign, displacement.abs(), field);
                    } else {
                        println!("{} [{}], {}", instruction, expression, field);
                    }
                }
            },
            MODE_REG => {
                let (source_index, destination_index): (u8, u8) = if d_bit == 1 {
                    (rm_field, reg_field)
                } else {
                    (reg_field, rm_field)
                };

                let immediate: u16 = get_16_bit_register(registers, source_index);
                arithmetic_op_with_reg_16_bit(registers, destination_index, instruction_index, immediate);

                let source: &str = REG_FIELD_ENCODINGS_16_BIT[source_index as usize];
                let destination: &str = REG_FIELD_ENCODINGS_16_BIT[destination_index as usize];
                let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                println!("{} {}, {}", instruction, destination, source);
            },
            _ => {
                debug_assert!(false);
            }
        }
    } else {
        match mod_field {
            MODE_MEM_NO_DISP => {
                if rm_field == 6 {
                    let address: u16 = grab_instruction_word(memory, &mut registers.ip);
                    if d_bit == 1 {
                        let immediate: u8 = load_byte(memory, address);
                        arithmetic_op_with_reg_8_bit(registers, reg_field, instruction_index, immediate);
    
                        let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} {}, [{}]", instruction, field, address);
                    } else {
                        let immediate: u8 = get_8_bit_register(registers, reg_field);
                        arithmetic_op_with_mem_8_bit(registers, memory, address, instruction_index, immediate);

                        let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} [{}], {}", instruction, address, field);
                    }
                } else {
                    let address: u16 = calculate_reg_expression(registers, rm_field);
                    if d_bit == 1 {
                        let immediate: u8 = load_byte(memory, address);
                        arithmetic_op_with_reg_8_bit(registers, reg_field, instruction_index, immediate);

                        let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                        let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} {}, [{}]", instruction, field, expression);
                    } else {
                        let immediate: u8 = get_8_bit_register(registers, reg_field);
                        arithmetic_op_with_mem_8_bit(registers, memory, address, instruction_index, immediate);

                        let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                        let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} [{}], {}", instruction, expression, field);
                    }
                }
            },
            MODE_MEM_8_BIT_DISP => {
                let displacement: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                if d_bit == 1 {
                    let immediate: u8 = load_byte(memory, address);
                    arithmetic_op_with_reg_8_bit(registers, reg_field, instruction_index, immediate);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} {}, [{} {} {}]", instruction, field, expression, sign, displacement.abs());
                    } else {
                        println!("{} {}, [{}]", instruction, field, expression);
                    }
                } else {
                    let immediate: u8 = get_8_bit_register(registers, reg_field);
                    arithmetic_op_with_mem_8_bit(registers, memory, address, instruction_index, immediate);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} [{} {} {}], {}", instruction, expression, sign, displacement.abs(), field);
                    } else {
                        println!("{} [{}], {}", instruction, expression, field);
                    }
                }
            },
            MODE_MEM_16_BIT_DISP => {
                let displacement: i16 = grab_instruction_word(memory, &mut registers.ip) as i16;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                if d_bit == 1 {
                    let immediate: u8 = load_byte(memory, address);
                    arithmetic_op_with_reg_8_bit(registers, reg_field, instruction_index, immediate);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} {}, [{} {} {}]", instruction, field, expression, sign, displacement.abs());
                    } else {
                        println!("{} {}, [{}]", instruction, field, expression);
                    }
                } else {
                    let immediate: u8 = get_8_bit_register(registers, reg_field);
                    arithmetic_op_with_mem_8_bit(registers, memory, address, instruction_index, immediate);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} [{} {} {}], {}", instruction, expression, sign, displacement.abs(), field);
                    } else {
                        println!("{} [{}], {}", instruction, expression, field);
                    }
                }
            },
            MODE_REG => {
                let (source_index, destination_index): (u8, u8) = if d_bit == 1 {
                    (rm_field, reg_field)
                } else {
                    (reg_field, rm_field)
                };

                let immediate: u8 = get_8_bit_register(registers, source_index);
                arithmetic_op_with_reg_8_bit(registers, destination_index, instruction_index, immediate);

                let source: &str = REG_FIELD_ENCODINGS_8_BIT[source_index as usize];
                let destination: &str = REG_FIELD_ENCODINGS_8_BIT[destination_index as usize];
                let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                println!("{} {}, {}", instruction, destination, source);
            },
            _ => {
                debug_assert!(false);
            }
        }
    }
}

pub fn decode_arithmetic_signed_imm_to_reg_mem_encoding(registers: &mut Registers, memory: &mut Memory) {
    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);
    
    let s_bit: u8 = (byte & 0x02) >> 1;
    let w_bit: u8 = byte & 0x01;

    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);
    
    let instruction_index: u8 = (byte & 0x38) >> 3;
    let mod_field: u8 = (byte & 0xC0) >> 6;
    let rm_field: u8 = byte & 0x07;

    if w_bit == 1 {
        match mod_field {
            MODE_MEM_NO_DISP => {
                if rm_field == 6 {
                    let address: u16 = grab_instruction_word(memory, &mut registers.ip);
                    if s_bit == 1 {
                        let immediate: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                        arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate as u16);
                        
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} word [{}], {}", instruction, address, immediate);
                    } else {
                        let immediate: u16 = grab_instruction_word(memory, &mut registers.ip);
                        arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate);

                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} word [{}], {}", instruction, address, immediate);
                    }
                } else {
                    let address: u16 = calculate_reg_expression(registers, rm_field);
                    if s_bit == 1 {
                        let immediate: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                        arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate as u16);
                        
                        let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} word [{}], {}", instruction, expression, immediate as i8);
                    } else {
                        let immediate: u16 = grab_instruction_word(memory, &mut registers.ip);
                        arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate);

                        let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                        println!("{} word [{}], {}", instruction, expression, immediate);
                    }
                }
            },
            MODE_MEM_8_BIT_DISP => {
                let displacement: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                if s_bit == 1 {
                    let immediate: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                    arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate as u16);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} word [{} {} {}], {}", instruction, expression, sign, displacement.abs(), immediate);
                    } else {
                        println!("{} word [{}], {}", instruction, expression, immediate);
                    }
                } else {
                    let immediate: u16 = grab_instruction_word(memory, &mut registers.ip);
                    arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} word [{} {} {}], {}", instruction, expression, sign, displacement.abs(), immediate);
                    } else {
                        println!("{} word [{}], {}", instruction, expression, immediate);
                    }
                }
            },
            MODE_MEM_16_BIT_DISP => {
                let displacement: i16 = grab_instruction_word(memory, &mut registers.ip) as i16;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                if s_bit == 1 {
                    let immediate: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                    arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate as u16);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} word [{} {} {}], {}", instruction, expression, sign, displacement.abs(), immediate);
                    } else {
                        println!("{} word [{}], {}", instruction, expression, immediate);
                    }
                } else {
                    let immediate: u16 = grab_instruction_word(memory, &mut registers.ip);
                    arithmetic_op_with_mem_16_bit(registers, memory, address, instruction_index, immediate);

                    let sign: char = if displacement > 0 { '+' } else { '-' };
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    if displacement != 0 {
                        println!("{} word [{} {} {}], {}", instruction, expression, sign, displacement.abs(), immediate);
                    } else {
                        println!("{} word [{}], {}", instruction, expression, immediate);
                    }
                }
            },
            MODE_REG => {
                if s_bit == 1 {
                    let immediate: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                    arithmetic_op_with_reg_16_bit(registers, rm_field, instruction_index, immediate as u16);

                    let field: &str = REG_FIELD_ENCODINGS_16_BIT[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    println!("{} {}, {}", instruction, field, immediate);
                } else {
                    let immediate: u16 = grab_instruction_word(memory, &mut registers.ip);
                    arithmetic_op_with_reg_16_bit(registers, rm_field, instruction_index, immediate);

                    let field: &str = REG_FIELD_ENCODINGS_16_BIT[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    println!("{} {}, {}", instruction, field, immediate);
                }
            },
            _ => {
                debug_assert!(false);
            }
        }
    } else {
        debug_assert!(s_bit == 0);  // Doesn't seem like this is ever set when w_bit == 0 from experimentation
        match mod_field {
            MODE_MEM_NO_DISP => {
                if rm_field == 6 {
                    let address: u16 = grab_instruction_word(memory, &mut registers.ip);
                    let immediate: u8 = grab_instruction_byte(memory, &mut registers.ip);
                    arithmetic_op_with_mem_8_bit(registers, memory, address, instruction_index, immediate);

                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    println!("{} byte [{}], {}", instruction, address, immediate);
                } else {
                    let address: u16 = calculate_reg_expression(registers, rm_field);
                    let immediate: u8 = grab_instruction_byte(memory, &mut registers.ip);
                    arithmetic_op_with_mem_8_bit(registers, memory, address, instruction_index, immediate);

                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                    println!("{} byte [{}], {}", instruction, expression, immediate);
                }
            },
            MODE_MEM_8_BIT_DISP => {
                let displacement: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                let immediate: u8 = grab_instruction_byte(memory, &mut registers.ip);
                arithmetic_op_with_mem_8_bit(registers, memory, address, instruction_index, immediate);

                let sign: char = if displacement > 0 { '+' } else { '-' };
                let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                if displacement != 0 {
                    println!("{} byte [{} {} {}], {}", instruction, expression, sign, displacement.abs(), immediate);
                } else {
                    println!("{} byte [{}], {}", instruction, expression, immediate);
                }
            },
            MODE_MEM_16_BIT_DISP => {
                let displacement: i16 = grab_instruction_word(memory, &mut registers.ip) as i16;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                let immediate: u8 = grab_instruction_byte(memory, &mut registers.ip);
                arithmetic_op_with_mem_8_bit(registers, memory, address, instruction_index, immediate);

                let sign: char = if displacement > 0 { '+' } else { '-' };
                let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                if displacement != 0 {
                    println!("{} byte [{} {} {}], {}", instruction, expression, sign, displacement.abs(), immediate);
                } else {
                    println!("{} byte [{}], {}", instruction, expression, immediate);
                }
            },
            MODE_REG => {
                let immediate: u8 = grab_instruction_byte(memory, &mut registers.ip);
                arithmetic_op_with_reg_8_bit(registers, rm_field, instruction_index, immediate);

                let field: &str = REG_FIELD_ENCODINGS_8_BIT[rm_field as usize];
                let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
                println!("{} {}, {}", instruction, field, immediate);
            },
            _ => {
                debug_assert!(false);
            }
        }
    }
}

pub fn decode_arithmetic_imm_to_acc_encoding(registers: &mut Registers, memory: &mut Memory) {
    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);

    let w_bit: u8 = byte & 0x01;
    let instruction_index: u8 = (byte & 0x38) >> 3;

    if w_bit == 1 {
        let immediate: u16 = grab_instruction_word(memory, &mut registers.ip);
        arithmetic_op_with_reg_16_bit(registers, 0, instruction_index, immediate);

        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
        println!("{} ax, {}", instruction, immediate);
    } else {
        let immediate: u8 = grab_instruction_byte(memory, &mut registers.ip);
        arithmetic_op_with_reg_8_bit(registers, 0, instruction_index, immediate);

        let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index as usize];
        println!("{} al, {}", instruction, immediate);
    }
}
