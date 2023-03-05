use std::env;
use std::fs;

const MOV_BITS: u8 = 0x88;
const MOV_REG_FIELD_ENCODINGS: &'static [&str] = &[
    "al", "cl", "dl", "bl", "ah", "ch", "dh", "bh", // w = 0
    "ax", "cx", "dx", "bx", "sp", "bp", "si", "di"  // w = 1
];

fn main() {
    let input_file = env::args().nth(1).expect("Please specify an input file");
    let bytes: Vec<u8> = fs::read(input_file).expect("Missing instruction stream file");

    println!("bits 16");

    let mut byte_index: usize = 0;
    let byte_count: usize = bytes.len();
    while byte_index < byte_count {
        let byte: u8 = bytes[byte_index];
        if byte & MOV_BITS != 0 {
            let d_bit: u8 = (byte & 0x2) >> 1;
            let w_bit: u8 = byte & 0x1;
            
            byte_index += 1;
            let byte: u8 = bytes[byte_index];

            let mod_field: u8 = (byte & 0xC0) >> 6;
            debug_assert!(mod_field == 0x03);   // only handling reg to reg mov atm

            let reg_field: u8 = (byte & 0x38) >> 3;
            let rm_field: u8 = byte & 0x07;

            let source_index: usize = ((1 - d_bit) * reg_field + d_bit * rm_field + 8 * w_bit) as usize;
            debug_assert!(source_index < MOV_REG_FIELD_ENCODINGS.len());
            let destination_index: usize = (d_bit * reg_field + (1 - d_bit) * rm_field + 8 * w_bit) as usize;
            debug_assert!(destination_index < MOV_REG_FIELD_ENCODINGS.len());

            let source: &str = MOV_REG_FIELD_ENCODINGS[source_index];
            let destination: &str = MOV_REG_FIELD_ENCODINGS[destination_index];

            println!("mov {}, {}", destination, source);

            byte_index += 1;
        } else {
            debug_assert!(false);   // Not handling any other instructions atm
        }
    }
}
