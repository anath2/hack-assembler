/// Hack Assembler:
/// Translates hack assembly language code to machine instructions. More information
/// on hack computer can be found at: https://www.nand2tetris.org


use std::{env, process};


fn main() {
    // Application entry point

    let args = parse_arguments().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

}


fn parse_arguments() -> Result <Vec<String>, &'static str> {
    // Parse assembly code arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err("Invalid command-line arguments")
    } else {
        Ok(args)
    }
}
