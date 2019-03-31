/// Hack Assembler:
/// Translates hack assembly language code to machine instructions. More information
/// on hack computer can be found at: https://www.nand2tetris.org


use std::io;
use std::env;
use std::fs;


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


mod parser {

    struct ParsedLine {
        line: String,  // Line number associated with the line
        inst: String,  // Instruction type whether A, L or C
        symb: String,  // Symbol if it is an A instruction or Label
        dest: String,  // Destination for a C instruction
        comp: String,  // Computation if it's a C instruction
        jump: String   // Jump if it is an C instructionj
    }


    pub fn parse_content(f_content: String) {
        // Parses the assembly file line by line and
        // returns a vector of ParsedLine
        let line_iter = f_content.split("\n");
        let line_iter = line_iter.map(|l| remove_comments(l.trim().to_string()));
        let line_iter = line_iter.filter(|l| l.len() > 0);

        for (linenum, line) in line_iter.enumerate() {
            let ln = line.to_string();
            let i_type = get_instruction_type(ln);
            println!("{} - {} - {}", linenum, i_type, line);
        }
    }


    fn remove_comments(line: String) -> String {
        // Removes comments from a line of code
        let split: Vec<&str> = line.split("//").collect();
        let clean_str = split[0];
        clean_str.to_string()
    }


    fn get_instruction_type(line: String) -> &'static str {
        // Checks whether it is A, L or C type instruction
        let char_vect: Vec<char> = line.chars().collect();
        let first_char = char_vect[0];

        if first_char == '@' {
            "A"
        } else if first_char == '(' {
            "L"
        } else {
            "C"
        }
    }


    fn get_dest(line: String, inst_type: &str) -> &'static str {
        // Get destination for the instruction
        let mut dest = "";

        if inst_type == "C" {
            let split: Vec<&str> = line.split("=").collect();
            dest = split[0];
        }

        dest
    }
}
