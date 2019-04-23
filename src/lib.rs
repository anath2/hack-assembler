// Parse hack assembly code into machine language code

extern crate regex;


pub struct Assembler<'a> {
    assembly: &'a str,
    machine: String,
}


// impl<'a> Assembler<'a> {

//     pub fn translate(assembly: &'a str) -> Assembler<'a> {
//         // Translate assembly code into machine instructions
//     }

// }


pub mod parser {
    use regex::{Regex, Captures};
    use std::process;
    use std::collections::HashMap;
    // Define line parsing structs

    pub struct A_instruction {
        lnum: usize,
        symbol: Option<String>,
        address: Option<usize>,
    }

    pub struct L_instruction {
        lnum: usize,
        symbol: String,
    }

    pub struct C_instruction {
        lnum: usize,
        dest: Option<String>,
        comp: Option<String>,
        jump: Option<String>,
    }

    pub enum Instruction {
        A(A_instruction),
        L(L_instruction),
        C(C_instruction)
    }

    pub fn parse(contents: &String) -> Vec<Instruction> {
        // Declare regex for matching file types
        let contents = remove_comments(contents);

        let mut instruction_regex = HashMap::new();
        instruction_regex.insert("A", Regex::new(r"^@(.+)$").unwrap());
        instruction_regex.insert("L", Regex::new(r"\((.+)\)$").unwrap());
        instruction_regex.insert("C", Regex::new(r"^(\w{1, 2})=?(.+)?;?(\w{3})?$").unwrap());

        let mut parsed: Vec<Instruction> = Vec::new();

        for (lnum, line) in contents.lines().enumerate() {

            for (&inst, regex) in &instruction_regex {

                if regex.is_match(&line) {
                    let caps = regex.captures(line).unwrap();

                    let parsed_line = match inst {
                        "A" => parse_a(lnum, caps),
                        "L" => parse_l(lnum, caps),
                         _ => parse_c(lnum, caps)
                    };

                    parsed.push(parsed_line);

                } else {
                    eprintln!("An error occurred parsing line - {}", &line);
                    process::exit(1);
                }
            }
        }

        parsed
    }

    fn remove_comments(code: &String) -> String {
        // Removes comments from code comments are of the form
        // // - Code till end of line
        let comment_regex = Regex::new(r"^.*(//.*)$").unwrap();
        let mut result = String::new();

        for line in code.lines() {
            let cleaned = comment_regex.replace_all(line, "");
            result.push_str(cleaned.trim());
        }

        result
    }

    fn parse_a(lnum: usize, caps: Captures) -> Instruction {
        // Parse A instructions
        // A instruction: @1
        let address_str = caps.get(1).unwrap().as_str().trim();

        let address = match address_str.parse() {
            Ok(v) => Some(v),
            Err(_) => None
        };

        let symbol = match address {
            Some(_) => None,
            None => Some(String::from(address_str))
        };

        Instruction::A(A_instruction {
            lnum,
            address,
            symbol
        })
    }

    fn  parse_l(lnum: usize, caps: Captures) -> Instruction {
        // Parse L
        // L instruction: (LABEL_NAME)
        let label = caps.get(1).unwrap().as_str().trim();

        Instruction::L(L_instruction {
            lnum: lnum + 1,
            symbol: String::from(label),
        })
    }

    fn parse_c(lnum: usize, caps: Captures) -> Instruction {
        // Parse C
        // C instruction: A = M + 1; JMP
        let dest = caps.get(1).map_or(None, |d| Some(String::from(d.as_str().trim())));
        let comp = caps.get(2).map_or(None, |c| Some(String::from(c.as_str().trim())));
        let jump = caps.get(3).map_or(None, |j| Some(String::from(j.as_str().trim())));

        Instruction::C(C_instruction{
            lnum,
            dest,
            comp,
            jump
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_type() {
        let code = String::from("@1");
        let parsed = parser::parse(&code);
    }

}
