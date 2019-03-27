/// Hack Assembler:
/// Translates hack assembly language code to machine instructions. More information
/// on hack computer can be found at: https://www.nand2tetris.org

use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic("Bad command line arguments")
    }

    let asm_f = &args[1].clone();
    contents = AssemblyFile::read(asm_f)

    println!("The file contents: \n{}", contents);
}


impl AssemblyFile {
    // - A hack assembly file is
    // - The file always has the extension .asm

    fn read(asm_f: &String) -> Result<String, Error> {
        let (&basename, &extension) = asm_f.split(".");

        if extension != ".asm" {
            Err("Unknown extension {} bad input file", extension);
        }

        let mut content = String::new();

        match asm_f.read_to_string(&mut content) {
            Ok(_) => Ok(content),
            Err(e) => Err(e)
        }
    }
}
