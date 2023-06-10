use crate::registers::*;
use crate::memory::*;
use crate::mode::*;

pub fn decode_mov_mem_reg_to_from_reg_encoding(registers: &mut Registers, memory: &mut Memory) {
    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);
    let d_bit: u8 = (byte & 0x2) >> 1;  // 1 <=> reg field gives destination
    let w_bit: u8 = byte & 0x1;         // 1 <=> wide version of instruction
    
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
                        let value: u16 = load_word(memory, address);
                        set_16_bit_register(registers, reg_field, value);
                    } else {
                        let value: u16 = get_16_bit_register(registers, reg_field);
                        store_word(memory, address, value);
                    }

                    let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                    if d_bit == 1 {
                        println!("mov {}, [{}]", field, address);
                    } else {
                        println!("mov [{}], {}", address, field);
                    }
                } else {
                    let address: u16 = calculate_reg_expression(registers, rm_field);
                    if d_bit == 1 {
                        let value: u16 = load_word(memory, address);
                        set_16_bit_register(registers, reg_field, value);
                    } else {
                        let value: u16 = get_16_bit_register(registers, reg_field);
                        store_word(memory, address, value);
                    }

                    let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    if d_bit == 1 {
                        println!("mov {}, [{}]", field, expression);
                    } else {
                        println!("mov [{}], {}", expression, field);
                    }
                }
            },
            MODE_MEM_8_BIT_DISP => {
                let displacement: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                if d_bit == 1 {
                    let value: u16 = load_word(memory, address);
                    set_16_bit_register(registers, reg_field, value);
                } else {
                    let value: u16 = get_16_bit_register(registers, reg_field);
                    store_word(memory, address, value);
                }

                let sign: char = if displacement > 0 { '+' } else { '-' };
                let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                if d_bit == 1 {
                    if displacement != 0 {
                        println!("mov {}, [{} {} {}]", field, expression, sign, displacement.abs());
                    } else {
                        println!("mov {}, [{}]", field, expression);
                    }
                } else {
                    if displacement != 0 {
                        println!("mov [{} {} {}], {}", expression, sign, displacement.abs(), field);
                    } else {
                        println!("mov [{}], {}", expression, field);
                    }
                }
            },
            MODE_MEM_16_BIT_DISP => {
                let displacement: i16 = grab_instruction_word(memory, &mut registers.ip) as i16;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                if d_bit == 1 {
                    let value: u16 = load_word(memory, address);
                    set_16_bit_register(registers, reg_field, value);
                } else {
                    let value: u16 = get_16_bit_register(registers, reg_field);
                    store_word(memory, address,value);
                }

                let sign: char = if displacement > 0 { '+' } else { '-' };
                let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
                let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                if d_bit == 1 {
                    if displacement != 0 {
                        println!("mov {}, [{} {} {}]", field, expression, sign, displacement.abs());
                    } else {
                        println!("mov {}, [{}]", field, expression);
                    }
                } else {
                    if displacement != 0 {
                        println!("mov [{} {} {}], {}", expression, sign, displacement.abs(), field);
                    } else {
                        println!("mov [{}], {}", expression, field);
                    }
                }
            },
            MODE_REG => {
                let (source_field, destination_field): (u8, u8) = if d_bit == 1 {
                    (rm_field, reg_field)
                } else {
                    (reg_field, rm_field)
                };

                let value: u16 = get_16_bit_register(registers, source_field);
                set_16_bit_register(registers, destination_field, value);

                let source: &str = REG_FIELD_ENCODINGS_16_BIT[source_field as usize];
                let destination: &str = REG_FIELD_ENCODINGS_16_BIT[destination_field as usize];
                println!("mov {}, {}", destination, source);
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
                        let value: u8 = load_byte(memory, address);
                        set_8_bit_register(registers, reg_field, value);
                    } else {
                        let value: u8 = get_8_bit_register(registers, reg_field);
                        store_byte(memory, address, value);
                    }

                    let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                    if d_bit == 1 {
                        println!("mov {}, [{}]", field, address);
                    } else {
                        println!("mov [{}], {}", address, field);
                    }
                } else {
                    let address: u16 = calculate_reg_expression(registers, rm_field);
                    if d_bit == 1 {
                        let value: u8 = load_byte(memory, address);
                        set_8_bit_register(registers, reg_field, value);
                    } else {
                        let value: u8 = get_8_bit_register(registers, reg_field);
                        store_byte(memory, address, value);
                    }

                    let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    if d_bit == 1 {
                        println!("mov {}, [{}]", field, expression);
                    } else {
                        println!("mov [{}], {}", expression, field);
                    }
                }
            },
            MODE_MEM_8_BIT_DISP => {
                let displacement: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                if d_bit == 1 {
                    let value: u8 = load_byte(memory, address);
                    set_8_bit_register(registers, reg_field, value);
                } else {
                    let value: u8 = get_8_bit_register(registers, reg_field);
                    store_byte(memory, address, value);
                }

                let sign: char = if displacement > 0 { '+' } else { '-' };
                let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                if d_bit == 1 {
                    if displacement != 0 {
                        println!("mov {}, [{} {} {}]", field, expression, sign, displacement.abs());
                    } else {
                        println!("mov {}, [{}]", field, expression);
                    }
                } else {
                    if displacement != 0 {
                        println!("mov [{} {} {}], {}", expression, sign, displacement.abs(), field);
                    } else {
                        println!("mov [{}], {}", expression, field);
                    }
                }
            },
            MODE_MEM_16_BIT_DISP => {
                let displacement: i16 = grab_instruction_word(memory, &mut registers.ip) as i16;
                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                if d_bit == 1 {
                    let value: u8 = load_byte(memory, address);
                    set_8_bit_register(registers, reg_field, value);
                } else {
                    let value: u8 = get_8_bit_register(registers, reg_field);
                    store_byte(memory, address,value);
                }

                let sign: char = if displacement > 0 { '+' } else { '-' };
                let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
                let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                if d_bit == 1 {
                    if displacement != 0 {
                        println!("mov {}, [{} {} {}]", field, expression, sign, displacement.abs());
                    } else {
                        println!("mov {}, [{}]", field, expression);
                    }
                } else {
                    if displacement != 0 {
                        println!("mov [{} {} {}], {}", expression, sign, displacement.abs(), field);
                    } else {
                        println!("mov [{}], {}", expression, field);
                    }
                }
            },
            MODE_REG => {
                let (source_field, destination_field): (u8, u8) = if d_bit == 1 {
                    (rm_field, reg_field)
                } else {
                    (reg_field, rm_field)
                };

                let value: u8 = get_8_bit_register(registers, source_field);
                set_8_bit_register(registers, destination_field, value);

                let source: &str = REG_FIELD_ENCODINGS_8_BIT[source_field as usize];
                let destination: &str = REG_FIELD_ENCODINGS_8_BIT[destination_field as usize];
                println!("mov {}, {}", destination, source);
            },
            _ => {
                debug_assert!(false);
            }
        }
    }
}

pub fn decode_mov_imm_to_reg_mem_encoding(registers: &mut Registers, memory: &mut Memory) {
    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);
    let w_bit: u8 = byte & 0x01;

    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);
    debug_assert!(byte & 0x38 == 0);
    let mod_field: u8 = (byte & 0xC0) >> 6;
    let rm_field: u8 = byte & 0x07;

    if w_bit == 1 {
        match mod_field {
            MODE_MEM_NO_DISP => {
                if rm_field == 6 {
                    let address: u16 = grab_instruction_word(memory, &mut registers.ip);
                    let data: u16 = grab_instruction_word(memory, &mut registers.ip);
                    store_word(memory, address, data);

                    println!("mov [{}], word {}", address, data);
                } else {
                    let address: u16 = calculate_reg_expression(registers, rm_field);
                    let data: u16 = grab_instruction_word(memory, &mut registers.ip);
                    store_word(memory, address, data);

                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    println!("mov [{}], word {}", expression, data);
                }
            },
            MODE_MEM_8_BIT_DISP => {
                let displacement: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                let data: u16 = grab_instruction_word(memory, &mut registers.ip);

                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);                
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                store_word(memory, address, data);

                let sign: char = if displacement > 0 { '+' } else { '-' };
                let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                println!("mov [{} {} {}], word {}", expression, sign, displacement.abs(), data);
            },
            MODE_MEM_16_BIT_DISP => {
                let displacement: i16 = grab_instruction_word(memory, &mut registers.ip) as i16;
                let data: u16 = grab_instruction_word(memory, &mut registers.ip);

                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);                
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                store_word(memory, address, data);                

                let sign: char = if displacement > 0 { '+' } else { '-' };
                let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                println!("mov [{} {} {}], word {}", expression, sign, displacement.abs(), data);
            },
            MODE_REG => {
                let data: u16 = grab_instruction_word(memory, &mut registers.ip);
                set_16_bit_register(registers, rm_field, data);

                let destination: &str = REG_FIELD_ENCODINGS_16_BIT[rm_field as usize];
                println!("mov {}, {}", destination, data);
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
                    let data: u8 = grab_instruction_byte(memory, &mut registers.ip);
                    store_byte(memory, address, data);

                    println!("mov [{}], byte {}", address, data);
                } else {
                    let address: u16 = calculate_reg_expression(registers, rm_field);
                    let data: u8 = grab_instruction_byte(memory, &mut registers.ip);
                    store_byte(memory, address, data);

                    let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                    println!("mov [{}], byte {}", expression, data);
                }
            },
            MODE_MEM_8_BIT_DISP => {
                let displacement: i8 = grab_instruction_byte(memory, &mut registers.ip) as i8;
                let data: u8 = grab_instruction_byte(memory, &mut registers.ip);

                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                store_byte(memory, address, data);

                let sign: char = if displacement > 0 { '+' } else { '-' };
                let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                println!("mov [{} {} {}], byte {}", expression, sign, displacement.abs(), data);
            },
            MODE_MEM_16_BIT_DISP => {
                let displacement: i16 = grab_instruction_word(memory, &mut registers.ip) as i16;
                let data: u8 = grab_instruction_byte(memory, &mut registers.ip);

                let reg_expression: u16 = calculate_reg_expression(registers, rm_field);
                let address: u16 = reg_expression.wrapping_add(displacement as u16);
                store_byte(memory, address, data);

                let sign: char = if displacement > 0 { '+' } else { '-' };
                let expression: &str = REG_EXPRESSION_ENCODINGS[rm_field as usize];
                println!("mov [{} {} {}], byte {}", expression, sign, displacement.abs(), data);
            },
            MODE_REG => {
                let data: u8 = grab_instruction_byte(memory, &mut registers.ip);
                set_8_bit_register(registers, rm_field, data);

                let destination: &str = REG_FIELD_ENCODINGS_8_BIT[rm_field as usize];
                println!("mov {}, {}", destination, data);
            },
            _ => {
                debug_assert!(false);
            }
        }
    }
}

pub fn decode_mov_imm_to_reg_encoding(registers: &mut Registers, memory: &mut Memory) {
    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);
    let w_bit: u8 = (byte & 0x08) >> 3;
    let reg_field: u8 = byte & 0x07;

    if w_bit == 1 {
        let immediate: u16 = grab_instruction_word(memory, &mut registers.ip);
        set_16_bit_register(registers, reg_field, immediate);

        let field: &str = REG_FIELD_ENCODINGS_16_BIT[reg_field as usize];
        println!("mov {}, {}", field, immediate);
    } else {
        let immediate: u8 = grab_instruction_byte(memory, &mut registers.ip);
        set_8_bit_register(registers, reg_field, immediate);

        let field: &str = REG_FIELD_ENCODINGS_8_BIT[reg_field as usize];
        println!("mov {}, {}", field, immediate);
    }
}

pub fn decode_mov_mem_to_acc_encoding(registers: &mut Registers, memory: &mut Memory) {
    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);
    let w_bit: u8 = byte & 0x01;

    if w_bit == 1 {
        let address: u16 = grab_instruction_word(memory, &mut registers.ip);
        let data: u16 = load_word(memory, address);
        registers.ax = data;

        println!("mov ax, [{}]", address);
    } else {
        let address: u16 = grab_instruction_word(memory, &mut registers.ip);
        let data: u8 = load_byte(memory, address);
        registers.ax = set_low_byte(registers.ax, data);

        println!("mov al, [{}]", address);
    }
}

pub fn decode_mov_acc_to_mem_encoding(registers: &mut Registers, memory: &mut Memory) {
    let byte: u8 = grab_instruction_byte(memory, &mut registers.ip);
    let w_bit: u8 = byte & 0x01;

    if w_bit == 1 {
        let address: u16 = grab_instruction_word(memory, &mut registers.ip);
        store_word(memory, address, registers.ax);

        println!("mov [{}], ax", address);
    } else {
        let address: u16 = grab_instruction_word(memory, &mut registers.ip);
        let data: u8 = get_low_byte(registers.ax);
        store_byte(memory, address, data);

        println!("mov [{}], ax", address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::prelude::*;

    fn produce_machine_code(assembly: &[u8]) -> Vec<u8> {
        let mut assembly_file = fs::File::create("./temp.asm").expect("Failed to create temp.asm file");
        assembly_file.write(assembly).expect("Failed to write to temp.asm file");
        std::process::Command::new("./bin/nasm.exe").arg("./temp.asm").output().expect("Failed to run assembler");

        let machine_code: Vec<u8> = fs::read("./temp").expect("Missing instruction stream file");

        fs::remove_file("./temp").expect("Failed to delete temp file");
        fs::remove_file("./temp.asm").expect("Failed to delete temp.asm file");

        return machine_code;
    }

    #[test]
    fn test_mov_imm_to_reg() {
        let mut machine_code: Vec<u8> = produce_machine_code(
            b"bits 16\n\
            mov al, 255"
        );

        let mut registers = Registers::default();
        registers.ax = 0xCCCC;
        let mut memory: Memory = [0; u16::MAX as usize];

        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(get_low_byte(registers.ax), 0xFF);
        assert_eq!(get_high_byte(registers.ax), 0xCC);

        machine_code = produce_machine_code(
            b"bits 16\n\
            mov ah, 255"
        );

        registers = Registers::default();
        registers.ax = 0xCCCC;
        memory.fill(0);

        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(get_low_byte(registers.ax), 0xCC);
        assert_eq!(get_high_byte(registers.ax), 0xFF);

        machine_code = produce_machine_code(
            b"bits 16\n\
            mov bl, 255"
        );

        registers = Registers::default();
        registers.bx = 0xCCCC;
        memory.fill(0);

        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(get_low_byte(registers.bx), 0xFF);
        assert_eq!(get_high_byte(registers.bx), 0xCC);

        machine_code = produce_machine_code(
            b"bits 16\n\
            mov bh, 255"
        );

        registers = Registers::default();
        registers.bx = 0xCCCC;
        memory.fill(0);

        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(get_low_byte(registers.bx), 0xCC);
        assert_eq!(get_high_byte(registers.bx), 0xFF);

        machine_code = produce_machine_code(
            b"bits 16\n\
            mov cl, 255"
        );

        registers = Registers::default();
        registers.cx = 0xCCCC;
        memory.fill(0);

        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(get_low_byte(registers.cx), 0xFF);
        assert_eq!(get_high_byte(registers.cx), 0xCC);

        machine_code = produce_machine_code(
            b"bits 16\n\
            mov ch, 255"
        );

        registers = Registers::default();
        registers.cx = 0xCCCC;
        memory.fill(0);

        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(get_low_byte(registers.cx), 0xCC);
        assert_eq!(get_high_byte(registers.cx), 0xFF);

        machine_code = produce_machine_code(
            b"bits 16\n\
            mov dl, 255"
        );

        registers = Registers::default();
        registers.dx = 0xCCCC;
        memory.fill(0);

        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(get_low_byte(registers.dx), 0xFF);
        assert_eq!(get_high_byte(registers.dx), 0xCC);

        machine_code = produce_machine_code(
            b"bits 16\n\
            mov dh, 255"
        );

        registers = Registers::default();
        registers.dx = 0xCCCC;
        memory.fill(0);

        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(get_low_byte(registers.dx), 0xCC);
        assert_eq!(get_high_byte(registers.dx), 0xFF);
    }
    
    #[test]
    fn test_mov_imm_to_reg_wide() {
        let mut machine_code: Vec<u8> = produce_machine_code(
            b"bits 16\n\
            mov ax, 65535"
        );

        let mut registers = Registers::default();
        let mut memory: Memory = [0; u16::MAX as usize];

        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(registers.ax, 0xFFFF);

        machine_code = produce_machine_code(
            b"bits 16\n\
            mov bx, 65535"
        );
        
        registers = Registers::default();
        memory.fill(0);

        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(registers.bx, 0xFFFF);

        machine_code = produce_machine_code(
            b"bits 16\n\
            mov cx, 65535"
        );

        registers = Registers::default();
        memory.fill(0);
        
        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(registers.cx, 0xFFFF);

        machine_code = produce_machine_code(
            b"bits 16\n\
            mov dx, 65535"
        );

        registers = Registers::default();
        memory.fill(0);
        
        memory[0..machine_code.len()].copy_from_slice(&machine_code);

        decode_mov_imm_to_reg_encoding(&mut registers, &mut memory);
        assert_eq!(registers.dx, 0xFFFF);
    }
}
