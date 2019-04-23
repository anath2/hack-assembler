/// Hack Assembler:
/// Translates hack assembly language code to machine instructions. More information
/// on hack computer can be found at: https://www.nand2tetris.org

extern crate assembler;

use std::{env, process, fs};
use assembler::Assembler;


fn main() {
    // Application entry point

    let args = parse_arguments().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let filepath = &args[1];  // Passed in as command line argument

    let contents = fs::read_to_string(filepath.as_str()).unwrap_or_else(|err| {
        eprintln!("An error occurred reading file - {}", err);
        process::exit(1);
    });

}


fn parse_arguments() -> result <vec<string>, &'static str> {
    // parse assembly code arguments
    let args: vec<string> = env::args().collect();

    if args.len() < 2 {
        err("invalid command-line arguments")
    } else {
        ok(args)
    }
}
