// Parse hack assembly code into machine language code

extern crate regex;


pub struct Assembler<'a> {
    assembly: &'a str,
    machine: String,
}


impl<'a> Assembler<'a> {

    pub fn translate(assembly: &'a str) -> Assembler<'a> {
        // Translate assembly code into machine instructions
    }

}


pub mod Parser {
    use regex::Regex;
    use std::process;
    // Define line parsing structs

    struct A_instruction {
        lnum: usize,
        symbol: Option<String>,
        address: Option<usize>,
    }

    struct L_instruction {
        lnum: usize,
        symbol: String,
    }

    struct C_instruction {
        lnum: usize,
        dest: Option<String>,
        comp: String,
        jump: Option<String>,
    }

    enum Instruction {
        A(A_instruction),
        L(L_instruction),
        C(C_instruction)
    }

    pub fn parse(contents: &String) -> Vec<Instruction> {
        // Declare regex for matching file types
        let comment_regex = Regex::new(r"^.*(//.*)$").unwrap();


        let mut parsed: Vec<Instruction>;

        for (lnum, line) in contents.lines().enumerate() {
            let inst = get_instruction_type(line).unwrap_or_else(|err| {
                eprintln!("Error occcurred parsing - {}", err);
                process::exit(1);
            });

            let parsed_line = match inst {
                "A" => parse_a(lnum, &line),
                "L" => parse_l(lnum, &line),
                "C" => parse_c(lnum, &line),
            };

            parsed.push(parsed_line);
        }

        parsed
    }

    pub fn get_instruction_type<'a>(line: &'a str) -> Result<&'a str, &'static str> {
        // Matches lines with regular expressions for instructions
        // If no match raises an error
        let a_regex = Regex::new(r"^@(.+)$").unwrap();
        let l_regex = Regex::new(r"\((.+)\)$").unwrap();
        let c_regex = Regex::new(r"^(\w{1, 2})=?(.+)?;?(\w{3})?$").unwrap();

        if a_regex.is_match(line) {
            Ok("A")
        } else if l_regex.is_match(line) {
            Ok("L")
        } else if c_regex.is_match(line) {
            Ok("C")
        } else {
            let err_msg = format!("Syntax error in line - {}", line);
            Err(err_msg.as_str())
        }
    }

    fn parse_a(lnum: usize, line: &str) -> Instruction {
        // Parse A instructions
        // A instruction: @1
        let split: Vec<&str> = line.split("@").collect();
        let address_string = String::from(split[1]);

        let address = match address_string.parse() {
            Ok(v) => Some(v),
            Err(_) => None
        };

        let symbol = match address {
            Some(_) => None,
            None => Some(address_string)
        };

        Instruction::A(A_instruction {
            lnum,
            address,
            symbol
        })
    }

    fn  parse_l(lnum: usize, line: &str) -> Instruction {
        // Parse L
    }

    fn parse_c(lnum: usize, line: &str) -> Instruction {
        // Parse C
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_type() {
        let a_inst = "@1";

        assert_eq!(
            "A",
            Parser::get_instruction_type(a_inst).unwrap()
        );
    }

}
