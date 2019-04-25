/// Hack Assembler:
/// Translates hack assembly language code to machine instructions. More information
/// on hack computer can be found at: https://www.nand2tetris.org

extern crate assembler;


use std::{env, process, fs, path};
use assembler::{parser, decoder};


pub struct Assembler<'a> {
    assembly: &'a str,
    machine: String,
}


impl<'a> Assembler<'a> {

    pub fn translate(assembly: &'a str) -> Assembler<'a> {
        let parsed = parser::parse(assembly);
        let decoded = decoder::decode(parsed);

        Assembler {
            assembly,
            machine: decoded
        }
    }
}


fn parse_arguments() -> Result<Vec<String>, &'static str> {
    // parse assembly code arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err("invalid command-line arguments")
    } else {
        Ok(args)
    }
}


fn main() {
    // Processes assembly file passed as command line args and writes
    // decoded instructions to file

    let args = parse_arguments().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let filepath = &args[1];  // Passed in as command line argument

    let contents = fs::read_to_string(filepath.as_str()).unwrap_or_else(|err| {
        eprintln!("An error occurred reading file - {}", err);
        process::exit(1);
    });

    let assembled = Assembler::translate(contents.as_str());
    println!("Assembly code:\n{}", assembled.assembly);
    println!("Machine code:\n{}", assembled.machine);

    let filepath = path::Path::new(filepath);  // Writing result to file without extension
    let outfile = filepath.file_stem().unwrap();

    fs::write(&outfile, assembled.machine).unwrap_or_else(|err| {
        eprintln!("An error occurred writing file - {}", err);
        process::exit(1);

    });
}
