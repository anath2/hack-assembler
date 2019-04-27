// Parse hack assembly code into machine language code

extern crate regex;


pub mod parser {
    /// Converts hack azssembly code into tokens to be decoded into
    /// binary code by the decoder

    use regex::{Regex, Captures};
    use std::process;
    use std::collections::HashMap;

    pub enum Instruction {
        A {lnum: usize, symbol: Option<String>, address: Option<usize>},
        L {lnum: usize, symbol: String},
        C {lnum: usize, dest: Option<String>, comp: Option<String>, jump: Option<String>}
    }

    pub fn parse<'a>(contents: &'a str) -> Vec<Instruction> {
        // Declare regex for matching file types
        let mut parsed: Vec<Instruction> = Vec::new();

        for (lnum, line) in contents.lines().enumerate() {
            let line = String::from(line);
            let line = remove_spaces(&line);
            let line_option = remove_comments(&line);

            let parsed_line = match line_option {
                Some(l) => parse_line(lnum, l),
                None => {continue;}

            };

            parsed.push(parsed_line);
        }

        parsed
    }

    fn remove_spaces(code: &String) -> String {
        // Remove excess spaces from a line of code
        let mut clean_str = String::new();

        for char in code.chars() {
            if !char.is_whitespace() {
                clean_str.push(char)
            }
        }

        clean_str
    }

    fn remove_comments(code: &String) -> Option<String> {
        // Removes comments from code comments are of the form
        // // - Code till end of line
        let comment_regex = Regex::new(r"^(?P<code>.*)(?P<comment>//.*)$").unwrap();
        let cleaned = comment_regex.replace_all(code, "${code}");

        if cleaned == "" {
            return None
        }

        Some(String::from(cleaned.trim()))
    }

    fn parse_line(lnum: usize, line: String) -> Instruction {
        let instruction_regex = get_regex_hashmap();
        let inst_option = instruction_regex.iter().find(|(_, regex)| regex.is_match(line.as_str()));

        match inst_option {

                Some((&inst, regex)) => {
                    let caps = regex.captures(line.as_str()).unwrap();

                    match inst {
                        "A" => parse_a(lnum, caps),
                        "L" => parse_l(lnum, caps),
                        _ => parse_c(lnum, caps)
                    }
                },

                None => {
                    // None of the regular expression matching is probably a syntax error
                    eprintln!("An error occurred parsing line - {}", &line);
                    process::exit(1)
                },
        }
    }

    fn get_regex_hashmap() -> HashMap<&'static str, Regex> {
        // Creates a hashmap for parsing different instruction types
        let mut instruction_regex = HashMap::new();
        instruction_regex.insert("A", Regex::new(r"^@(?P<address>.+)$").unwrap());
        instruction_regex.insert("L", Regex::new(r"\((?P<label>.+)\)$").unwrap());
        instruction_regex.insert("C", Regex::new(r"^(?P<dest>\w{1, 2})?=?(?P<comp>\S{1,3});?(?P<jump>\w{3})?$").unwrap());
        instruction_regex
    }

    fn parse_a(lnum: usize, caps: Captures) -> Instruction {
        // Parse A instructions
        // A instruction: @1
        let address_str = caps.name("address").unwrap().as_str();

        let address = match address_str.parse() {
            Ok(v) => Some(v),
            Err(_) => None
        };

        let symbol = match address {
            Some(_) => None,
            None => Some(String::from(address_str))
        };

        Instruction::A {
            lnum: lnum,
            address: address,
            symbol: symbol,
        }
    }

    fn  parse_l(lnum: usize, caps: Captures) -> Instruction {
        // Parse L
        // L instruction: (LABEL_NAME)
        let label = caps.name("label").unwrap().as_str();

        Instruction::L {
            lnum: lnum + 1,
            symbol: String::from(label),
        }
    }

    fn parse_c(lnum: usize, caps: Captures) -> Instruction {
        // Parse C
        // C instruction: A = M + 1; JMP
        let dest = caps.name("dest").map_or(None, |d| Some(String::from(d.as_str())));
        let comp = caps.name("comp").map_or(None, |c| Some(String::from(c.as_str())));
        let jump = caps.name("jump").map_or(None, |j| Some(String::from(j.as_str())));

        Instruction::C {
            lnum,
            dest,
            comp,
            jump
        }
    }
}


pub mod symbol_table {
    pub struct Symbol {
        name: String,
        value: usize
    }

    fn create(code: &String) -> Vec<Symbol> {
        // Creates a new symbol table from assembly
        // language code
    }

}


#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn parse() {
        let code = String::from(
            "@1 // @1\n(LABEL)\nA = M + 1; JMP"
        );

        let parsed = parser::parse(&code);

        for parsed_line in parsed {
            if let parser::Instruction::A {lnum: l, address:a, symbol: s} = &parsed_line {
                assert_eq!(*l, 0 as usize);
                assert_eq!(*a, Some(1));
                assert_eq!(*s, None);
            }
            if let parser::Instruction::L {lnum: l, symbol: s} = &parsed_line {
                assert_eq!(*l, 2 as usize);
                assert_eq!(*s, "LABEL");
            }
            if let parser::Instruction::C {lnum: l, dest: d, comp: c, jump: j} = &parsed_line {
                assert_eq!(*l, 2 as usize);
                assert_eq!(*d, Some(String::from("A")));
                assert_eq!(*c, Some(String::from("M+1")));
                assert_eq!(*j, Some(String::from("JMP")));
            }
        }
    }

}


pub mod decoder {
    /// Decoder traslates parsed assembly code into machine language
    use std::process;
    use parser::Instruction;

    pub fn decode(assembly: Vec<Instruction>) -> String {
        // Decode parsed assembly language code into binary
        let mut decoded = String::new();

        for parsed_line in assembly {

            match parsed_line {
                Instruction::A {address: a, ..} => {
                    let decoded_line = format!("{}\n", decode_a(a).as_str());
                    decoded.push_str(&decoded_line);
                },
                Instruction::C {dest: d, comp: c, jump: j, ..} => {
                    let decoded_line = format!("{}\n", decode_c(d, c, j).as_str());
                    decoded.push_str(&decoded_line);
                },
                _ => {continue;},
            };
        }

        decoded
    }

    fn decode_a(a: Option<usize>) -> String {
        // Translates A instructions to machine code

        let addr = a.unwrap_or_else(|| {
            eprintln!("Unable to decode, address is None");
            process::exit(1);
        });

        format!("0{:015b}", addr)
    }

    fn decode_c(dest: Option<String>, comp: Option<String>, jump: Option<String>) -> String {
        // Translate c instruction into binary
        let decoded_d = match dest{
            Some(s) => {
                let d = s.as_str();
                match d {
                    "M" => "001",
                    "D" => "010",
                    "MD" => "011",
                    "A"  => "100",
                    "AM" => "101",
                    "AD" => "110",
                    "AMD"=> "111",
                    _ => {
                        eprintln!("Cannot decode - unknown destination - {}", d);
                        process::exit(1);
                    },
                }
            },
            None => ""
        };

        let decoded_c = match comp {
            Some(c) => {
                let c = c.as_str();
                match c {
                    "0" => "0101010",
                    "1" => "0111111",
                    "D" => "0001100",
                    "A" => "0110000",
                    "M" => "1110000",
                    "-1" => "0111110",
                    "!D" => "0001101",
                    "!A" => "0110001",
                    "!M" => "1110001",
                    "-D" => "0001111",
                    "-A" => "0110011",
                    "-M" => "1110011",
                    "D+1" => "0011111",
                    "A+1" => "0110111",
                    "M+1" => "1110111",
                    "D-1" => "0001110",
                    "A-1" => "0110010",
                    "D+A" => "0000010",
                    "D-A" => "0010011",
                    "A-D" => "0000111",
                    "D&A" => "0000000",
                    "D|A" => "0010101",
                    "M-1" => "1110010",
                    "D+M" => "1000010",
                    "D-M" => "1010011",
                    "M-D" => "1000111",
                    "D&M" => "1000000",
                    "D|M" => "1010101",
                    _ => {
                        eprintln!("Cannot decode - unknown computation - {}", c);
                        process::exit(1);
                    },
                }
            },
            None => ""
        };

        let decoded_j = match jump {
            Some(j) => {
                let j = j.as_str();
                match j {
                    "JGT" => "001",
                    "JEQ" => "010",
                    "JGE" => "011",
                    "JLT" => "100",
                    "JNE" => "101",
                    "JLE" => "110",
                    "JMP" => "111",
                    _ => {
                        eprintln!("Cannot decode - unknown jump - {}", j);
                        process::exit(1);
                    },
                }
            },
            None => ""
        };

        let decoded_c = format!("111{}{}{}", decoded_c, decoded_d, decoded_j);
        String::from(decoded_c)
    }

}
