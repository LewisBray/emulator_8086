use std::env;
use std::fs;

const MOV_REG_MEM_TO_FROM_REG_BITS: u8 = 0x88;
const MOV_IMM_TO_REG_BITS: u8 = 0xB0;
const MOV_MEM_TO_ACC_BITS: u8 = 0xA0;
const MOV_ACC_TO_MEM_BITS: u8 = 0xA2;

const ADD_REG_MEM_WITH_REG_TO_EITHER_BITS: u8 = 0x00;
const ADD_IMM_TO_REG_MEM_BITS: u8 = 0x80;
const ADD_IMM_TO_ACC_BITS: u8 = 0x04;

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

struct ByteStream {
    bytes: Vec<u8>,
    index: usize
}

fn grab_byte(byte_stream: &mut ByteStream) -> u8 {
    debug_assert!(!byte_stream.bytes.is_empty());
    debug_assert!(byte_stream.index < byte_stream.bytes.len());

    let byte: u8 = byte_stream.bytes[byte_stream.index];
    byte_stream.index += 1;
    
    return byte;
}

fn grab_word(byte_stream: &mut ByteStream) -> u16 {
    debug_assert!(!byte_stream.bytes.is_empty());
    debug_assert!(byte_stream.index < byte_stream.bytes.len() - 1);

    let word_low: u8 = byte_stream.bytes[byte_stream.index];
    byte_stream.index += 1;

    let word_high: u8 = byte_stream.bytes[byte_stream.index];
    byte_stream.index += 1;

    let word: u16 = ((word_high as u16) << 8) + (word_low as u16);

    return word;
}

fn main() {
    let input_file = env::args().nth(1).expect("Please specify an input file");
    let bytes: Vec<u8> = fs::read(input_file).expect("Missing instruction stream file");

    let mut byte_stream = ByteStream{bytes: bytes, index: 0};

    println!("bits 16");

    let byte_count: usize = byte_stream.bytes.len();
    while byte_stream.index < byte_count {
        let byte: u8 = grab_byte(&mut byte_stream);
        if byte & 0xFC == MOV_REG_MEM_TO_FROM_REG_BITS {
            let d_bit: u8 = (byte & 0x2) >> 1;  // 1 <=> reg field gives destination
            let w_bit: u8 = byte & 0x1;         // 1 <=> wide version of instruction
            
            let byte: u8 = grab_byte(&mut byte_stream);

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
                },
                0x01 => {
                    let expression_index: usize = rm_field as usize;
                    debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
                    let field_index: usize = (reg_field + 8 * w_bit) as usize;
                    debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

                    let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];
                    let field: &str = REG_FIELD_ENCODINGS[field_index];

                    let displacement: u8 = grab_byte(&mut byte_stream);

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
                },
                0x02 => {
                    let expression_index: usize = rm_field as usize;
                    debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
                    let field_index: usize = (reg_field + 8 * w_bit) as usize;
                    debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

                    let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];
                    let field: &str = REG_FIELD_ENCODINGS[field_index];

                    let displacement: u16 = grab_word(&mut byte_stream);

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
                },
                0x03 => {
                    let source_index: usize = ((1 - d_bit) * reg_field + d_bit * rm_field + 8 * w_bit) as usize;
                    debug_assert!(source_index < REG_FIELD_ENCODINGS.len());
                    let destination_index: usize = (d_bit * reg_field + (1 - d_bit) * rm_field + 8 * w_bit) as usize;
                    debug_assert!(destination_index < REG_FIELD_ENCODINGS.len());

                    let source: &str = REG_FIELD_ENCODINGS[source_index];
                    let destination: &str = REG_FIELD_ENCODINGS[destination_index];

                    println!("mov {}, {}", destination, source);
                },
                _ => {
                    debug_assert!(false);
                }
            }
        } else if byte & 0xF0 == MOV_IMM_TO_REG_BITS {
            let w_bit: u8 = (byte & 0x08) >> 3;
            let reg_field: u8 = byte & 0x07;

            let immediate: u16 = if w_bit == 1 {
                grab_word(&mut byte_stream)
            } else {
                grab_byte(&mut byte_stream) as u16
            };

            let field_index: usize = (reg_field + 8 * w_bit) as usize;
            debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

            let field: &str = REG_FIELD_ENCODINGS[field_index];

            println!("mov {}, {}", field, immediate);
        } else if byte & 0xFE == MOV_MEM_TO_ACC_BITS {
            let w_bit: u8 = byte & 0x01;

            let address: u16 = if w_bit == 1 {
                grab_word(&mut byte_stream)
            } else {
                grab_byte(&mut byte_stream) as u16
            };

            println!("mov ax, [{}]", address);
        } else if byte & 0xFE == MOV_ACC_TO_MEM_BITS {
            let w_bit: u8 = byte & 0x01;

            let address: u16 = if w_bit == 1 {
                grab_word(&mut byte_stream)
            } else {
                grab_byte(&mut byte_stream) as u16
            };

            println!("mov [{}], ax", address);
        } else if byte & 0xFC == ADD_REG_MEM_WITH_REG_TO_EITHER_BITS {
            let d_bit: u8 = (byte & 0x02) >> 1;
            let w_bit: u8 = byte & 0x01;

            let byte: u8 = grab_byte(&mut byte_stream);

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
                        println!("add {}, [{}]", field, expression);
                    } else {
                        println!("add [{}], {}", expression, field);
                    }
                },
                0x01 => {
                    let expression_index: usize = rm_field as usize;
                    debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
                    let field_index: usize = (reg_field + 8 * w_bit) as usize;
                    debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

                    let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];
                    let field: &str = REG_FIELD_ENCODINGS[field_index];

                    let displacement: u8 = grab_byte(&mut byte_stream);

                    if d_bit == 1 {
                        if displacement != 0 {
                            println!("add {}, [{} + {}]", field, expression, displacement);
                        } else {
                            println!("add {}, [{}]", field, expression);
                        }
                    } else {
                        if displacement != 0 {
                            println!("add [{} + {}], {}", expression, displacement, field);
                        } else {
                            println!("add [{}], {}", expression, field);
                        }
                    }
                },
                0x02 => {
                    let expression_index: usize = rm_field as usize;
                    debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
                    let field_index: usize = (reg_field + 8 * w_bit) as usize;
                    debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

                    let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];
                    let field: &str = REG_FIELD_ENCODINGS[field_index];

                    let displacement: u16 = grab_word(&mut byte_stream);

                    if d_bit == 1 {
                        if displacement != 0 {
                            println!("add {}, [{} + {}]", field, expression, displacement);
                        } else {
                            println!("add {}, [{}]", field, expression);
                        }
                    } else {
                        if displacement != 0 {
                            println!("add [{} + {}], {}", expression, displacement, field);
                        } else {
                            println!("add [{}], {}", expression, field);
                        }
                    }
                },
                0x03 => {
                    let source_index: usize = ((1 - d_bit) * reg_field + d_bit * rm_field + 8 * w_bit) as usize;
                    debug_assert!(source_index < REG_FIELD_ENCODINGS.len());
                    let destination_index: usize = (d_bit * reg_field + (1 - d_bit) * rm_field + 8 * w_bit) as usize;
                    debug_assert!(destination_index < REG_FIELD_ENCODINGS.len());

                    let source: &str = REG_FIELD_ENCODINGS[source_index];
                    let destination: &str = REG_FIELD_ENCODINGS[destination_index];

                    println!("add {}, {}", destination, source);
                },
                _ => {
                    debug_assert!(false);
                }
            }
        } else if byte & 0xFC == ADD_IMM_TO_REG_MEM_BITS {
            let s_bit: u8 = (byte & 0x02) >> 1;
            let w_bit: u8 = byte & 0x01;

            let byte: u8 = grab_byte(&mut byte_stream);
            debug_assert!(byte & 0x38 == 0);

            let mod_field: u8 = (byte & 0xC0) >> 6;
            let rm_field: u8 = byte & 0x07;
            match mod_field {
                0x00 => {
                    let expression_index: usize = rm_field as usize;
                    debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());

                    let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

                    let data: u16 = if s_bit == 0 && w_bit == 1 {
                        grab_word(&mut byte_stream)
                    } else {
                        grab_byte(&mut byte_stream) as u16
                    };

                    let size: &str = if w_bit == 1 { "word" } else { "byte" };

                    if s_bit == 1 {
                        println!("add {} [{}], {}", size, expression, data as i8);
                    } else {
                        println!("add {} [{}], {}", size, expression, data);
                    }
                },
                0x01 => {
                    let expression_index: usize = rm_field as usize;
                    debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());

                    let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

                    let displacement: u8 = grab_byte(&mut byte_stream);

                    let data: u16 = if s_bit == 0 && w_bit == 1 {
                        grab_word(&mut byte_stream)
                    } else {
                        grab_byte(&mut byte_stream) as u16
                    };

                    let size: &str = if w_bit == 1 { "word" } else { "byte" };

                    if s_bit == 1 {
                        if displacement != 0 {
                            println!("add {} [{} + {}], {}", size, expression, displacement, data as i8);
                        } else {
                            println!("add {} [{}], {}", size, expression, data as i8);
                        }
                    } else {
                        if displacement != 0 {
                            println!("add {} [{} + {}], {}", size, expression, displacement, data);
                        } else {
                            println!("add {} [{}], {}", size, expression, data);
                        }
                    }
                },
                0x02 => {
                    let expression_index: usize = rm_field as usize;
                    debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());

                    let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

                    let displacement: u16 = grab_word(&mut byte_stream);

                    let data: u16 = if s_bit == 0 && w_bit == 1 {
                        grab_word(&mut byte_stream)
                    } else {
                        grab_byte(&mut byte_stream) as u16
                    };

                    let size: &str = if w_bit == 1 { "word" } else { "byte" };

                    if s_bit == 1 {
                        if displacement != 0 {
                            println!("add {} [{} + {}], {}", size, expression, displacement, data as i8);
                        } else {
                            println!("add {} [{}], {}", size, expression, data as i8);
                        }
                    } else {
                        if displacement != 0 {
                            println!("add {} [{} + {}], {}", size, expression, displacement, data);
                        } else {
                            println!("add {} [{}], {}", size, expression, data);
                        }
                    }
                },
                0x03 => {
                    let field_index: usize = (rm_field + 8 * w_bit) as usize;
                    debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

                    let field: &str = REG_FIELD_ENCODINGS[field_index];

                    let data: u16 = if s_bit == 0 && w_bit == 1 {
                        grab_word(&mut byte_stream)
                    } else {
                        grab_byte(&mut byte_stream) as u16
                    };

                    if s_bit == 1 {
                        println!("add {}, {}", field, data as i8);
                    } else {
                        println!("add {}, {}", field, data);
                    }
                },
                _ => {
                    debug_assert!(false);
                }
            }
        } else if byte & 0xFE == ADD_IMM_TO_ACC_BITS {
            let w_bit: u8 = byte & 0x01;

            let immediate: u16 = if w_bit == 1 {
                grab_word(&mut byte_stream)
            } else {
                grab_byte(&mut byte_stream) as u16
            };

            let field: &str = if w_bit == 1 { "ax" } else { "al" };

            println!("add {}, {}", field, immediate);
        } else {
            debug_assert!(false);   // Not handling any other instructions atm
        }
    }
}
