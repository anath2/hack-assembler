/// Parses each line in the assembly code
/// Whitespaces and comments (starting //) are ignored.
/// Assume that the code has no syntax errors

use regex::Regex;


lazy_static! {
     static ref COMMENT_REGEX: Regex = Regex::new(r"^.*(//.*)$").unwrap();
    static ref A_INSTRUCTION: Regex = Regex::new(r"^@(.+)$").unwrap();
    static ref L_INSTRUCTION: Regex = Regex::new(r"\((.+)\)$").unwrap();
    static ref C_INSTRUCTION: Regex = Regex::new(r"^(\w{1, 2})=?(.+)?;?(\w{3})?$").unwrap();
}


enum Instruction {
    A {line_number: usize, symbol: String, address: usize},
    C {line_number: usize, dest: String, comp: String, jump: String},
    L {line_number: usize, symbol: String}
}


pub fn parse_content(f_content: String) -> Vec<ParsedLine> {
    // Parses lines of assembly code
    let lines = f_content.split("\n");
    let mut parsed: Vec<ParsedLine> = Vec::new();

    for (lno, line) in lines.enumerate() {
        let line = remove_comments(line);
        let line = line.trim();

        if line.len() > 0 {
            let lno = lno as u16;
            let line = line.to_string();
            let p = ParsedLine::new(lno, line);
            parsed.push(p);
        }

    }

    parsed
}


fn remove_comments(line: &str) -> String {
    // Removes comments from a line of code
    let cleaned = COMMENT_REGEX.replace_all(line, "").to_string();
    cleaned
}


pub struct ParsedLine {
    line:  u16,     // Line number associated with the line
    inst:  String,  // Instruction type
    addr:  i32,     // Address in case of an A instruction
    dest:  String,  // Destination for a C instruction
    comp:  String,  // Computation if it's a C instruction
    jump:  String,  // Jump if it is an C instructionj
    symb:  String,  // Symbol if it is a L or A instruction
}


impl ParsedLine {

    pub fn new(lno: u16, line: String) -> ParsedLine {
        // Parses a line and returns a parsed dict
        let inst_res = get_instruction_type(&line);

        if inst_res.is_ok() {
            let inst = inst_res.unwrap();

            match inst {
                "A" => parse_a_instruction(lno, &line),
                "L"=> parse_l_instruction(lno, &line),
                _   => parse_c_instruction(lno, &line),
            }

        } else {
            panic!("Syntax error on line number {}", lno);
        }
    }
}


fn get_instruction_type(line: &String) -> Result<&str, &str> {
    match line {
        line if A_INSTRUCTION.is_match(line) => Ok("A"),
        line if L_INSTRUCTION.is_match(line) => Ok("L"),
        line if C_INSTRUCTION.is_match(line) => Ok("C"),
        _ => Err("Unknown Instruction type")
    }
}


fn parse_a_instruction(lno: u16, line: &String) -> ParsedLine {
    // Parses Instruction of A type
    let caps = A_INSTRUCTION.captures(line).unwrap();
    let addr = caps.get(1).map_or("", |a| a.as_str()).trim();

    // If addr is not a line add it as a symbol
    let parsed = match addr.parse() {
        Ok(v) => ParsedLine {
            line:  lno,
            inst:  "A".to_string(),
            addr:  v,
            dest:  "".to_string(),
            comp:  "".to_string(),
            jump:  "".to_string(),
            symb:  "".to_string()
        },
        Err(_) => ParsedLine {
            line:  lno,
            inst:  "A".to_string(),
            addr:  -1,  // -1 indicating unknown address
            dest:  "".to_string(),
            comp:  "".to_string(),
            jump:  "".to_string(),
            symb:  addr.to_string()
        }
    };

    parsed
}


fn parse_l_instruction(lno: u16, line: &String) -> ParsedLine {
    // Parses Instruction of L type
    let caps = L_INSTRUCTION.captures(line).unwrap();
    let label = caps.get(1).map_or("", |a| a.as_str().trim());

    ParsedLine {
        line:  lno + 1,  // Label always points to the next line number
        inst:  "L".to_string(),
        addr:  -1,
        dest:  "".to_string(),
        comp:  "".to_string(),
        jump:  "".to_string(),
        symb:  label.to_string()
    }
}


fn parse_c_instruction(lno: u16, line: &String) -> ParsedLine{
    // Parses a C instruction
    let caps = C_INSTRUCTION.captures(line).unwrap();
    let dest = caps.get(1).map_or("", |d| d.as_str().trim());
    let comp = caps.get(2).map_or("", |c| c.as_str().trim());
    let jump = caps.get(3).map_or("", |j| j.as_str().trim());

    ParsedLine {
        line:  lno,
        inst:  "C".to_string(),
        addr:  -1,
        dest:  dest.to_string(),
        comp:  comp.to_string(),
        jump:  jump.to_string(),
        symb:  "".to_string()
    }
}
