use std::env;
use std::fs;

struct ByteStream {
    bytes: Vec<u8>,
    index: usize
}

fn peek_byte(byte_stream: &ByteStream) -> u8 {
    debug_assert!(!byte_stream.bytes.is_empty());
    debug_assert!(byte_stream.index < byte_stream.bytes.len());

    let byte: u8 = byte_stream.bytes[byte_stream.index];
    return byte;
}

fn grab_byte(byte_stream: &mut ByteStream) -> u8 {
    debug_assert!(!byte_stream.bytes.is_empty());
    debug_assert!(byte_stream.index < byte_stream.bytes.len());

    let byte: u8 = byte_stream.bytes[byte_stream.index];
    byte_stream.index += 1;
    
    return byte;
}

fn grab_byte_as_word(byte_stream: &mut ByteStream) -> u16 {
    let byte: u8  = grab_byte(byte_stream);
    return byte as u16;
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

#[derive(Debug)]
struct Registers {
    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,
    sp: u16,
    bp: u16,
    si: u16,
    di: u16
}

fn set_al(registers: &mut Registers, value: u16) {
    debug_assert!(value & 0x00FF == value);
    registers.ax = (registers.ax & 0xFF00) + value;
}

fn set_ah(registers: &mut Registers, value: u16) {
    debug_assert!(value & 0x00FF == value);
    registers.ax = (registers.ax & 0x00FF) + (value << 8);
}

fn set_bl(registers: &mut Registers, value: u16) {
    debug_assert!(value & 0x00FF == value);
    registers.bx = (registers.bx & 0xFF00) + value;
}

fn set_bh(registers: &mut Registers, value: u16) {
    debug_assert!(value & 0x00FF == value);
    registers.bx = (registers.bx & 0x00FF) + (value << 8);
}

fn set_cl(registers: &mut Registers, value: u16) {
    debug_assert!(value & 0x00FF == value);
    registers.cx = (registers.cx & 0xFF00) + value;
}

fn set_ch(registers: &mut Registers, value: u16) {
    debug_assert!(value & 0x00FF == value);
    registers.cx = (registers.cx & 0x00FF) + (value << 8);
}

fn set_dl(registers: &mut Registers, value: u16) {
    debug_assert!(value & 0x00FF == value);
    registers.dx = (registers.dx & 0xFF00) + value;
}

fn set_dh(registers: &mut Registers, value: u16) {
    debug_assert!(value & 0x00FF == value);
    registers.dx = (registers.dx & 0x00FF) + (value << 8);
}

fn set_ax(registers: &mut Registers, value: u16) {
    registers.ax = value;
}

fn set_bx(registers: &mut Registers, value: u16) {
    registers.bx = value;
}

fn set_cx(registers: &mut Registers, value: u16) {
    registers.cx = value;
}

fn set_dx(registers: &mut Registers, value: u16) {
    registers.dx = value;
}

fn set_sp(registers: &mut Registers, value: u16) {
    registers.sp = value;
}

fn set_bp(registers: &mut Registers, value: u16) {
    registers.bp = value;
}

fn set_si(registers: &mut Registers, value: u16) {
    registers.si = value;
}

fn set_di(registers: &mut Registers, value: u16) {
    registers.di = value;
}

const DATA_SIZE_ENCODINGS: &'static [&str] = &["byte", "word"];

type GrabDataFn = fn(&mut ByteStream) -> u16;
const GRAB_DATA_FNS: &'static [GrabDataFn] = &[grab_byte_as_word, grab_word];

const REG_FIELD_ENCODINGS: &'static [&str] = &[
    "al", "cl", "dl", "bl", "ah", "ch", "dh", "bh", // w = 0
    "ax", "cx", "dx", "bx", "sp", "bp", "si", "di"  // w = 1
];

type SetRegisterFn = fn(&mut Registers, u16);
const SET_REGISTER_FNS: &'static [SetRegisterFn] = &[
    set_al, set_cl, set_dl, set_bl, set_ah, set_ch, set_dh, set_bh,
    set_ax, set_cx, set_dx, set_bx, set_sp, set_bp, set_si, set_di
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

const MOV_REG_MEM_TO_FROM_REG_BITS: u8 = 0x88;
const MOV_IMM_TO_REG_MEM_BITS: u8 = 0xC6;
const MOV_IMM_TO_REG_BITS: u8 = 0xB0;
const MOV_MEM_TO_ACC_BITS: u8 = 0xA0;
const MOV_ACC_TO_MEM_BITS: u8 = 0xA2;

fn decode_mov_mem_reg_to_from_reg_encoding(byte_stream: &mut ByteStream) {
    let byte: u8 = grab_byte(byte_stream);
    let d_bit: u8 = (byte & 0x2) >> 1;  // 1 <=> reg field gives destination
    let w_bit: u8 = byte & 0x1;         // 1 <=> wide version of instruction
    
    let byte: u8 = grab_byte(byte_stream);

    let mod_field: u8 = (byte & 0xC0) >> 6;
    let reg_field: u8 = (byte & 0x38) >> 3;
    let rm_field: u8 = byte & 0x07;

    match mod_field {
        0x00 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
            let field_index: usize = (reg_field + 8 * w_bit) as usize;
            debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

            let field: &str = REG_FIELD_ENCODINGS[field_index];

            if expression_index == 6 {
                let address: u16 = if w_bit == 1 {
                    grab_word(byte_stream)
                } else {
                    grab_byte(byte_stream) as u16
                };

                if d_bit == 1 {
                    println!("mov {}, [{}]", field, address);
                } else {
                    println!("mov [{}], {}", address, field);
                }
            } else {
                let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

                if d_bit == 1 {
                    println!("mov {}, [{}]", field, expression);
                } else {
                    println!("mov [{}], {}", expression, field);
                }
            }
        },
        0x01 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
            let field_index: usize = (reg_field + 8 * w_bit) as usize;
            debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

            let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];
            let field: &str = REG_FIELD_ENCODINGS[field_index];

            let displacement: i8 = grab_byte(byte_stream) as i8;
            let sign: char = if displacement > 0 { '+' } else { '-' };

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
        0x02 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
            let field_index: usize = (reg_field + 8 * w_bit) as usize;
            debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

            let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];
            let field: &str = REG_FIELD_ENCODINGS[field_index];

            let displacement: i16 = grab_word(byte_stream) as i16;
            let sign: char = if displacement > 0 { '+' } else { '-' };

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
}

fn decode_mov_imm_to_reg_mem_encoding(byte_stream: &mut ByteStream) {
    let byte: u8 = grab_byte(byte_stream);
    let w_bit: u8 = byte & 0x01;

    let byte: u8 = grab_byte(byte_stream);
    debug_assert!(byte & 0x38 == 0);
    let mod_field: u8 = (byte & 0xC0) >> 6;
    let rm_field: u8 = byte & 0x07;

    match mod_field {
        0x00 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());

            let size: &str = DATA_SIZE_ENCODINGS[w_bit as usize];

            if expression_index == 6 {
                let address: u16 = if w_bit == 1 {
                    grab_word(byte_stream)
                } else {
                    grab_byte(byte_stream) as u16
                };

                let data: u16 = if w_bit == 1 {
                    grab_word(byte_stream)
                } else {
                    grab_byte(byte_stream) as u16
                };

                println!("mov [{}], {} {}", address, size, data);
            } else {
                let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

                let data: u16 = if w_bit == 1 {
                    grab_word(byte_stream)
                } else {
                    grab_byte(byte_stream) as u16
                };

                println!("mov [{}], {} {}", expression, size, data);
            }
        },
        0x01 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());

            let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

            let size: &str = DATA_SIZE_ENCODINGS[w_bit as usize];

            let displacement: i8 = grab_byte(byte_stream) as i8;
            let sign: char = if displacement > 0 { '+' } else { '-' };

            let data: u16 = if w_bit == 1 {
                grab_word(byte_stream)
            } else {
                grab_byte(byte_stream) as u16
            };

            println!("mov [{} {} {}], {} {}", expression, sign, displacement.abs(), size, data);
        },
        0x02 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());

            let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

            let size: &str = DATA_SIZE_ENCODINGS[w_bit as usize];

            let displacement: i16 = grab_word(byte_stream) as i16;
            let sign: char = if displacement > 0 { '+' } else { '-' };

            let data: u16 = if w_bit == 1 {
                grab_word(byte_stream)
            } else {
                grab_byte(byte_stream) as u16
            };

            println!("mov [{} {} {}], {} {}", expression, sign, displacement.abs(), size, data);
        },
        0x03 => {
            let field_index: usize = (rm_field + 8 * w_bit) as usize;
            debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

            let destination: &str = REG_FIELD_ENCODINGS[field_index];

            let data: u16 = if w_bit == 1 {
                grab_word(byte_stream)
            } else {
                grab_byte(byte_stream) as u16
            };

            println!("mov {}, {}", destination, data);
        },
        _ => {
            debug_assert!(false);
        }
    }    
}

fn decode_mov_imm_to_reg_encoding(byte_stream: &mut ByteStream, registers: &mut Registers) {
    let byte: u8 = grab_byte(byte_stream);
    let w_bit: u8 = (byte & 0x08) >> 3;
    let reg_field: u8 = byte & 0x07;

    let field_index: usize = (reg_field + 8 * w_bit) as usize;
    debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

    let grab_data: GrabDataFn = GRAB_DATA_FNS[w_bit as usize];
    let immediate: u16 = grab_data(byte_stream);

    let set_register: SetRegisterFn = SET_REGISTER_FNS[field_index];
    set_register(registers, immediate);

    let field: &str = REG_FIELD_ENCODINGS[field_index];

    println!("mov {}, {}", field, immediate);
}

fn decode_mov_mem_to_acc_encoding(byte_stream: &mut ByteStream) {
    let byte: u8 = grab_byte(byte_stream);
    let w_bit: u8 = byte & 0x01;

    let address: u16 = if w_bit == 1 {
        grab_word(byte_stream)
    } else {
        grab_byte(byte_stream) as u16
    };

    println!("mov ax, [{}]", address);
}

fn decode_mov_acc_to_mem_encoding(byte_stream: &mut ByteStream) {
    let byte: u8 = grab_byte(byte_stream);
    let w_bit: u8 = byte & 0x01;

    let address: u16 = if w_bit == 1 {
        grab_word(byte_stream)
    } else {
        grab_byte(byte_stream) as u16
    };

    println!("mov [{}], ax", address);
}

const ARITHMETIC_REG_MEM_WITH_REG_TO_EITHER_BITS: u8 = 0x00;
const ARITHMETIC_IMM_TO_REG_MEM_BITS: u8 = 0x80;
const ARITHMETIC_IMM_TO_ACC_BITS: u8 = 0x04;

const ARITHMETIC_INSTRUCTION_ENCODINGS: &'static [&str] = &[
    "add", "or", "adc", "sbb", "and", "sub", "xor", "cmp"
];

fn decode_arithmetic_mem_reg_with_reg_to_either_encoding(byte_stream: &mut ByteStream) {
    let byte: u8 = grab_byte(byte_stream);

    let instruction_index: usize = ((byte & 0x38) >> 3) as usize;
    debug_assert!(instruction_index < ARITHMETIC_INSTRUCTION_ENCODINGS.len());
    
    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index];

    let d_bit: u8 = (byte & 0x02) >> 1;
    let w_bit: u8 = byte & 0x01;

    let byte: u8 = grab_byte(byte_stream);

    let mod_field: u8 = (byte & 0xC0) >> 6;
    let reg_field: u8 = (byte & 0x38) >> 3;
    let rm_field: u8 = byte & 0x07;

    match mod_field {
        0x00 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
            let field_index: usize = (reg_field + 8 * w_bit) as usize;
            debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

            let field: &str = REG_FIELD_ENCODINGS[field_index];

            if expression_index == 6 {
                let address: u16 = if w_bit == 1 {
                    grab_word(byte_stream)
                } else {
                    grab_byte(byte_stream) as u16
                };

                if d_bit == 1 {
                    println!("{} {}, [{}]", instruction, field, address);
                } else {
                    println!("{} [{}], {}", instruction, address, field);
                }
            } else {
                let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

                if d_bit == 1 {
                    println!("{} {}, [{}]", instruction, field, expression);
                } else {
                    println!("{} [{}], {}", instruction, expression, field);
                }
            }
        },
        0x01 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());
            let field_index: usize = (reg_field + 8 * w_bit) as usize;
            debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

            let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];
            let field: &str = REG_FIELD_ENCODINGS[field_index];

            let displacement: i8 = grab_byte(byte_stream) as i8;
            let sign: char = if displacement > 0 { '+' } else { '-' };

            if d_bit == 1 {
                if displacement != 0 {
                    println!("{} {}, [{} {} {}]", instruction, field, expression, sign, displacement.abs());
                } else {
                    println!("{} {}, [{}]", instruction, field, expression);
                }
            } else {
                if displacement != 0 {
                    println!("{} [{} {} {}], {}", instruction, expression, sign, displacement.abs(), field);
                } else {
                    println!("{} [{}], {}", instruction, expression, field);
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

            let displacement: i16 = grab_word(byte_stream) as i16;
            let sign: char = if displacement > 0 { '+' } else { '-' };

            if d_bit == 1 {
                if displacement != 0 {
                    println!("{} {}, [{} {} {}]", instruction, field, expression, sign, displacement.abs());
                } else {
                    println!("{} {}, [{}]", instruction, field, expression);
                }
            } else {
                if displacement != 0 {
                    println!("{} [{} {} {}], {}", instruction, expression, sign, displacement.abs(), field);
                } else {
                    println!("{} [{}], {}", instruction, expression, field);
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

            println!("{} {}, {}", instruction, destination, source);
        },
        _ => {
            debug_assert!(false);
        }
    }
}

fn decode_arithmetic_signed_imm_to_reg_encoding(byte_stream: &mut ByteStream) {
    let byte: u8 = grab_byte(byte_stream);

    let s_bit: u8 = (byte & 0x02) >> 1;
    let w_bit: u8 = byte & 0x01;

    let byte: u8 = grab_byte(byte_stream);

    let mod_field: u8 = (byte & 0xC0) >> 6;
    let instruction_index: usize = ((byte & 0x38) >> 3) as usize;
    debug_assert!(instruction_index < ARITHMETIC_INSTRUCTION_ENCODINGS.len());
    let rm_field: u8 = byte & 0x07;

    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index];

    match mod_field {
        0x00 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());

            let size: &str = DATA_SIZE_ENCODINGS[w_bit as usize];

            if expression_index == 6 {
                let address: u16 = if w_bit == 1 {
                    grab_word(byte_stream)
                } else {
                    grab_byte(byte_stream) as u16
                };

                let data: u16 = if s_bit == 0 && w_bit == 1 {
                    grab_word(byte_stream)
                } else {
                    grab_byte(byte_stream) as u16
                };

                if s_bit == 1 {
                    println!("{} {} [{}], {}", instruction, size, address, data as i8);
                } else {
                    println!("{} {} [{}], {}", instruction, size, address, data);
                }
            } else {
                let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

                let data: u16 = if s_bit == 0 && w_bit == 1 {
                    grab_word(byte_stream)
                } else {
                    grab_byte(byte_stream) as u16
                };

                if s_bit == 1 {
                    println!("{} {} [{}], {}", instruction, size, expression, data as i8);
                } else {
                    println!("{} {} [{}], {}", instruction, size, expression, data);
                }
            }
        },
        0x01 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());

            let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

            let displacement: i8 = grab_byte(byte_stream) as i8;
            let sign: char = if displacement > 0 { '+' } else { '-' };

            let data: u16 = if s_bit == 0 && w_bit == 1 {
                grab_word(byte_stream)
            } else {
                grab_byte(byte_stream) as u16
            };

            let size: &str = DATA_SIZE_ENCODINGS[w_bit as usize];

            if s_bit == 1 {
                if displacement != 0 {
                    println!("{} {} [{} {} {}], {}", instruction, size, expression, sign, displacement.abs(), data as i8);
                } else {
                    println!("{} {} [{}], {}", instruction, size, expression, data as i8);
                }
            } else {
                if displacement != 0 {
                    println!("{} {} [{} {} {}], {}", instruction, size, expression, sign, displacement.abs(), data);
                } else {
                    println!("{} {} [{}], {}", instruction, size, expression, data);
                }
            }
        },
        0x02 => {
            let expression_index: usize = rm_field as usize;
            debug_assert!(expression_index < REG_EXPRESSION_ENCODINGS.len());

            let expression: &str = REG_EXPRESSION_ENCODINGS[expression_index];

            let displacement: i16 = grab_word(byte_stream) as i16;
            let sign: char = if displacement > 0 { '+' } else { '-' };

            let data: u16 = if s_bit == 0 && w_bit == 1 {
                grab_word(byte_stream)
            } else {
                grab_byte(byte_stream) as u16
            };

            let size: &str = DATA_SIZE_ENCODINGS[w_bit as usize];

            if s_bit == 1 {
                if displacement != 0 {
                    println!("{} {} [{} {} {}], {}", instruction, size, expression, sign, displacement.abs(), data as i8);
                } else {
                    println!("{} {} [{}], {}", instruction, size, expression, data as i8);
                }
            } else {
                if displacement != 0 {
                    println!("{} {} [{} {} {}], {}", instruction, size, expression, sign, displacement.abs(), data);
                } else {
                    println!("{} {} [{}], {}", instruction, size, expression, data);
                }
            }
        },
        0x03 => {
            let field_index: usize = (rm_field + 8 * w_bit) as usize;
            debug_assert!(field_index < REG_FIELD_ENCODINGS.len());

            let field: &str = REG_FIELD_ENCODINGS[field_index];

            let data: u16 = if s_bit == 0 && w_bit == 1 {
                grab_word(byte_stream)
            } else {
                grab_byte(byte_stream) as u16
            };

            if s_bit == 1 {
                println!("{} {}, {}", instruction, field, data as i8);
            } else {
                println!("{} {}, {}", instruction, field, data);
            }
        },
        _ => {
            debug_assert!(false);
        }
    }
}

fn decode_arithmetic_imm_to_acc_encoding(byte_stream: &mut ByteStream) {
    let byte: u8 = grab_byte(byte_stream);

    let w_bit: u8 = byte & 0x01;

    let instruction_index: usize = ((byte & 0x38) >> 3) as usize;
    debug_assert!(instruction_index < ARITHMETIC_INSTRUCTION_ENCODINGS.len());
    
    let instruction: &str = ARITHMETIC_INSTRUCTION_ENCODINGS[instruction_index];

    let immediate: u16 = if w_bit == 1 {
        grab_word(byte_stream)
    } else {
        grab_byte(byte_stream) as u16
    };

    let field: &str = if w_bit == 1 { "ax" } else { "al" };

    println!("{} {}, {}", instruction, field, immediate);
}

const CONDITIONAL_JUMP_INSTRUCTION_BITS: u8 = 0x70;

const CONDITIONAL_JUMP_INSTRUCTION_ENCODINGS: &'static [&str] = &[
    "jo", "jno", "jb", "jnb", "je", "jne", "jbe", "ja",
    "js", "jns", "jp", "jnp", "jl", "jnl", "jle", "jg"
];

fn decode_conditional_jump_encoding(byte_stream: &mut ByteStream) {
    let byte: u8 = grab_byte(byte_stream);

    let instruction_index: usize = (byte & 0x0F) as usize;
    debug_assert!(instruction_index < CONDITIONAL_JUMP_INSTRUCTION_ENCODINGS.len());

    let instruction: &str = CONDITIONAL_JUMP_INSTRUCTION_ENCODINGS[instruction_index];

    let offset: i8 = grab_byte(byte_stream) as i8;

    println!("{} {}", instruction, offset);
}

const LOOP_INSTRUCTION_BITS: u8 = 0xE0;

const LOOP_INSTRUCTION_ENCODINGS: &'static [&str] = &[
    "loopnz", "loopz", "loop", "jcxz"
];

fn decode_loop_encoding(byte_stream: &mut ByteStream) {
    let byte: u8 = grab_byte(byte_stream);

    let instruction_index: usize = (byte & 0x03) as usize;
    debug_assert!(instruction_index < LOOP_INSTRUCTION_ENCODINGS.len());

    let instruction: &str = LOOP_INSTRUCTION_ENCODINGS[instruction_index];

    let offset: i8 = grab_byte(byte_stream) as i8;

    println!("{} {}", instruction, offset);
}

fn main() {
    let input_file = env::args().nth(1).expect("Please specify an input file");
    let bytes: Vec<u8> = fs::read(input_file).expect("Missing instruction stream file");

    let mut byte_stream = ByteStream{bytes: bytes, index: 0};

    let mut registers = Registers{
        ax: 0, bx: 0, cx: 0, dx: 0, sp: 0, bp: 0, si: 0, di: 0
    };

    println!("bits 16");

    let byte_count: usize = byte_stream.bytes.len();
    while byte_stream.index < byte_count {
        let byte: u8 = peek_byte(&byte_stream);
        if byte & 0xFC == MOV_REG_MEM_TO_FROM_REG_BITS {
            decode_mov_mem_reg_to_from_reg_encoding(&mut byte_stream);
        } else if byte & 0xFE == MOV_IMM_TO_REG_MEM_BITS {
            decode_mov_imm_to_reg_mem_encoding(&mut byte_stream);
        } else if byte & 0xF0 == MOV_IMM_TO_REG_BITS {
            decode_mov_imm_to_reg_encoding(&mut byte_stream, &mut registers);
        } else if byte & 0xFE == MOV_MEM_TO_ACC_BITS {
            decode_mov_mem_to_acc_encoding(&mut byte_stream);
        } else if byte & 0xFE == MOV_ACC_TO_MEM_BITS {
            decode_mov_acc_to_mem_encoding(&mut byte_stream);
        } else if byte & 0xC4 == ARITHMETIC_REG_MEM_WITH_REG_TO_EITHER_BITS {
            decode_arithmetic_mem_reg_with_reg_to_either_encoding(&mut byte_stream);
        } else if byte & 0xFC == ARITHMETIC_IMM_TO_REG_MEM_BITS {
            decode_arithmetic_signed_imm_to_reg_encoding(&mut byte_stream);
        } else if byte & 0xC4 == ARITHMETIC_IMM_TO_ACC_BITS {
            decode_arithmetic_imm_to_acc_encoding(&mut byte_stream);
        } else if byte & 0xF0 == CONDITIONAL_JUMP_INSTRUCTION_BITS {
            decode_conditional_jump_encoding(&mut byte_stream);
        } else if byte & 0xF0 == LOOP_INSTRUCTION_BITS {
            decode_loop_encoding(&mut byte_stream);
        } else {
            debug_assert!(false);   // Not handling any other instructions atm
        }
    }

    dbg!(registers);
}
