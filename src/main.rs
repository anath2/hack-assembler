/// Hack Assembler:
/// Translates hack assembly language code to machine instructions. More information
/// on hack computer can be found at: https://www.nand2tetris.org


use std::io;
use std::env;
use std::fs;

extern crate regex;
extern crate lazy_static;

mod parser;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Bad arguments - expected file path");
    }

    let asm_f = args[1].to_string();
    let contents = read_assembly(asm_f).expect("Something went wrong");
    parser::parse_content(contents);
}


fn read_assembly(asm_f: String) -> Result<String, io::Error> {
    // Reads an .asm and output a Result with file content
    // if read is sucessful, otherwise returns a string literal
    // with the error message
    let temp_f = asm_f.clone();
    let parts: Vec<&str> = temp_f.split(".").collect();

    if parts.len() != 2 || parts[1] != "asm" {
        panic!("Bad input file! {}\n", temp_f)
    }

    fs::read_to_string(asm_f)
}
