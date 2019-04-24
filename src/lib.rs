// Parse hack assembly code into machine language code

extern crate regex;

#[allow(dead_code)]
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

    pub enum Instruction {
        A {lnum: usize, symbol: Option<String>, address: Option<usize>},
        L {lnum: usize, symbol: String},
        C {lnum: usize, dest: Option<String>, comp: Option<String>, jump: Option<String>}
    }

    pub fn parse(contents: &String) -> Vec<Instruction> {
        // Declare regex for matching file types
        let contents = remove_comments(contents);
        let instruction_regex = get_regex_hashmap();
        let mut parsed: Vec<Instruction> = Vec::new();

        for (lnum, line) in contents.lines().enumerate() {
            let some = instruction_regex.iter().find(|(_, regex)| regex.is_match(line));

            let parsed_line = match some {

                Some((&inst, regex)) => {
                    let caps = regex.captures(line).unwrap();

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
            };

            parsed.push(parsed_line);
        }

        parsed
    }

    fn get_regex_hashmap() -> HashMap<&'static str, Regex> {
        // Creates a hashmap for parsing different instruction types
        let mut instruction_regex = HashMap::new();
        instruction_regex.insert("A", Regex::new(r"^@(.+)$").unwrap());
        instruction_regex.insert("L", Regex::new(r"\((.+)\)$").unwrap());
        instruction_regex.insert("C", Regex::new(r"^(\w{1, 2})=?(.+)?;?(\w{3})?$").unwrap());
        instruction_regex
    }

    pub fn remove_comments(code: &String) -> String {
        // Removes comments from code comments are of the form
        // // - Code till end of line
        let comment_regex = Regex::new(r"^(?P<code>.*)(?P<comment>//.*)$").unwrap();
        let mut result = String::new();

        for line in code.lines() {
            let cleaned = comment_regex.replace_all(line, "${code}");
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

        Instruction::A {
            lnum: lnum,
            address: address,
            symbol: symbol,
        }
    }

    fn  parse_l(lnum: usize, caps: Captures) -> Instruction {
        // Parse L
        // L instruction: (LABEL_NAME)
        let label = caps.get(1).unwrap().as_str().trim();

        Instruction::L {
            lnum: lnum + 1,
            symbol: String::from(label),
        }
    }

    fn parse_c(lnum: usize, caps: Captures) -> Instruction {
        // Parse C
        // C instruction: A = M + 1; JMP
        let dest = caps.get(1).map_or(None, |d| Some(String::from(d.as_str().trim())));
        let comp = caps.get(2).map_or(None, |c| Some(String::from(c.as_str().trim())));
        let jump = caps.get(3).map_or(None, |j| Some(String::from(j.as_str().trim())));

        Instruction::C {
            lnum,
            dest,
            comp,
            jump
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_comments() {
        let code_line = String::from("@1// @1");
        let cleaned = parser::remove_comments(&code_line);
        assert_eq!(cleaned, String::from("@1"));
    }

    #[test]
    fn parse_a() {
        let code = String::from("@1 // @1");
        let parsed = parser::parse(&code);
        for parsed_line in parsed {
            if let parser::Instruction::A {lnum: l, address:a, symbol: s} = parsed_line {
                assert_eq!(l, 0 as usize);
                assert_eq!(a, Some(1));
                assert_eq!(s, None);
            }
        }
    }

    #[test]
    fn parse_l() {
        let code = String::from("(LABEL)");
        let parsed = parser::parse(&code);
        for parsed_line in parsed {
            if let parser::Instruction::L {lnum: l, symbol: s} = parsed_line {
                assert_eq!(l, 1 as usize);
                assert_eq!(s, "LABEL");
            }
        }
    }
}
