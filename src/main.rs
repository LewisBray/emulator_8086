use std::env;
use std::fs;

const MOV_REG_MEM_TO_FROM_REG_BITS: u8 = 0x88;
const MOV_IMM_TO_REG_BITS: u8 = 0xB0;
const MOV_MEM_TO_ACC_BITS: u8 = 0xA0;
const MOV_ACC_TO_MEM_BITS: u8 = 0xA2;

const REG_FIELD_ENCODINGS: &'static [&str] = &[
    "al", "cl", "dl", "bl", "ah", "ch", "dh", "bh", // w = 0
    "ax", "cx", "dx", "bx", "sp", "bp", "si", "di"  // w = 1
];

const REG_EXPRESSION_ENCODINGS: &'static [&str] = &[
    "bx + si",
    "bx + di",
    "bp + si",
    "bp + di",
    "si",
    "di",
    "bp",
    "bx"
];

fn main() {
    let input_file = env::args().nth(1).expect("Please specify an input file");
    let bytes: Vec<u8> = fs::read(input_file).expect("Missing instruction stream file");

    println!("bits 16");

    let mut byte_index: usize = 0;
    let byte_count: usize = bytes.len();
    while byte_index < byte_count {
        let byte: u8 = bytes[byte_index];
        if byte & 0xFC == MOV_REG_MEM_TO_FROM_REG_BITS {
            let d_bit: u8 = (byte & 0x2) >> 1;  // 1 <=> reg field gives destination
            let w_bit: u8 = byte & 0x1;         // 1 <=> wide version of instruction
            
            byte_index += 1;
            let byte: u8 = bytes[byte_index];

            let mod_field: u8 = (byte & 0xC0) >> 6;
            let reg_field: u8 = (byte & 0x38) >> 3;
            let rm_field: u8 = byte & 0x07;

            match mod_field {
                0x00 => {
                    let expression_index: usize = rm_field as usize;
                    debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
                    let field_index: usize = (reg_field + 8 * w_bit) as usize;
                    debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

                    let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];
                    let field: &str = REG_FIELD_ENCODINGS[field_index];

                    if d_bit == 1 {
                        println!("mov {}, [{}]", field, expression);
                    } else {
                        println!("mov [{}], {}", expression, field);
                    }

                    byte_index += 1;
                },
                0x01 => {
                    let expression_index: usize = rm_field as usize;
                    debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
                    let field_index: usize = (reg_field + 8 * w_bit) as usize;
                    debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

                    let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];
                    let field: &str = REG_FIELD_ENCODINGS[field_index];

                    byte_index += 1;
                    let displacement: u8 = bytes[byte_index];

                    if d_bit == 1 {
                        if displacement != 0 {
                            println!("mov {}, [{} + {}]", field, expression, displacement);
                        } else {
                            println!("mov {}, [{}]", field, expression);
                        }
                    } else {
                        if displacement != 0 {
                            println!("mov [{} + {}], {}", expression, displacement, field);
                        } else {
                            println!("mov [{}], {}", expression, field);
                        }
                    }

                    byte_index += 1;
                },
                0x02 => {
                    let expression_index: usize = rm_field as usize;
                    debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
                    let field_index: usize = (reg_field + 8 * w_bit) as usize;
                    debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

                    let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];
                    let field: &str = REG_FIELD_ENCODINGS[field_index];

                    byte_index += 1;
                    let displacement_low: u8 = bytes[byte_index];

                    byte_index += 1;
                    let displacement_high: u8 = bytes[byte_index];

                    let displacement: u16 = ((displacement_high as u16) << 8) + displacement_low as u16;

                    if d_bit == 1 {
                        if displacement != 0 {
                            println!("mov {}, [{} + {}]", field, expression, displacement);
                        } else {
                            println!("mov {}, [{}]", field, expression);
                        }
                    } else {
                        if displacement != 0 {
                            println!("mov [{} + {}], {}", expression, displacement, field);
                        } else {
                            println!("mov [{}], {}", expression, field);
                        }
                    }

                    byte_index += 1;
                },
                0x03 => {
                    let source_index: usize = ((1 - d_bit) * reg_field + d_bit * rm_field + 8 * w_bit) as usize;
                    debug_assert!(source_index < REG_FIELD_ENCODINGS.len());
                    let destination_index: usize = (d_bit * reg_field + (1 - d_bit) * rm_field + 8 * w_bit) as usize;
                    debug_assert!(destination_index < REG_FIELD_ENCODINGS.len());

                    let source: &str = REG_FIELD_ENCODINGS[source_index];
                    let destination: &str = REG_FIELD_ENCODINGS[destination_index];

                    println!("mov {}, {}", destination, source);
                    
                    byte_index += 1;
                }
                _ => {
                    debug_assert!(false);
                }
            }
        } else if byte & 0xF0 == MOV_IMM_TO_REG_BITS {
            let w_bit: u8 = (byte & 0x08) >> 3;
            let reg_field: u8 = byte & 0x07;

            let immediate: u16 = if w_bit == 1 {
                byte_index += 1;
                let immediate_low: u8 = bytes[byte_index];

                byte_index += 1;
                let immediate_high: u8 = bytes[byte_index];

                ((immediate_high as u16) << 8) + immediate_low as u16
            } else {
                byte_index += 1;
                bytes[byte_index] as u16
            };

            let field_index: usize = (reg_field + 8 * w_bit) as usize;
            debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

            let field: &str = REG_FIELD_ENCODINGS[field_index];

            println!("mov {}, {}", field, immediate);

            byte_index += 1;
        } else if byte & 0xFE == MOV_MEM_TO_ACC_BITS {
            let w_bit: u8 = byte & 0x01;

            let address: u16 = if w_bit == 1 {
                byte_index += 1;
                let address_low: u8 = bytes[byte_index];

                byte_index += 1;
                let address_high: u8 = bytes[byte_index];

                ((address_high as u16) << 8) + address_low as u16
            } else {
                byte_index += 1;
                bytes[byte_index] as u16
            };

            println!("mov ax, [{}]", address);

            byte_index += 1;
        } else if byte & 0xFE == MOV_ACC_TO_MEM_BITS {
            let w_bit: u8 = byte & 0x01;

            let address: u16 = if w_bit == 1 {
                byte_index += 1;
                let address_low: u8 = bytes[byte_index];

                byte_index += 1;
                let address_high: u8 = bytes[byte_index];

                ((address_high as u16) << 8) + address_low as u16
            } else {
                byte_index += 1;
                bytes[byte_index] as u16
            };

            println!("mov [{}], ax", address);

            byte_index += 1;
        } else {
            debug_assert!(false);   // Not handling any other instructions atm
        }
    }
}
