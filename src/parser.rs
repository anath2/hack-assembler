/// Parses each line in the assembly code
/// Whitespaces and comments (starting //) are ignored.
/// Assume that the code has no syntax errors

use regex::Regex;


lazy_static! {
    static ref COMMENT_REGEX: Regex = Regex::new(r"^.*(//.*)$").unwrap();
    static ref A_INSTRUCTION: Regex = Regex::new(r"^@(.+)$").unwrap();
    static ref L_INSTRUCTION: Regex = Regex::new(r"\((.+)\)$").unwrap();
    static ref C_INSTRUCTION: Regex = Regex::new(r"^(.+)(?:=?)(.+)(?:;?)(.+)$").unwrap();
}


pub fn parse_content(f_content: String) -> Vec<ParsedLine> {
    // Parses lines of assembly code
    let lines = f_content.split("\n");
    let mut parsed: Vec<ParsedLine> = Vec::new();

    for (lno, line) in lines.enumerate() {
        let line = remove_comments(line);
        let line = line.trim();

        if line.len() > 0 {
            let lno = lno as u32;
            let line = line.to_string();
            let p = ParsedLine::new(lno, line);
            parsed.push(p)
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
    line:  u32,     // Line number associated with the line
    inst:  String,  // Instruction type whether A, L or C
    addr:  String,  // Address in case of an A instruction
    dest:  String,  // Destination for a C instruction
    comp:  String,  // Computation if it's a C instruction
    jump:  String,  // Jump if it is an C instructionj
    label: String,  // Symbol if it is a Label
}


impl ParsedLine {
    pub fn new(lno: u32, line: String) -> ParsedLine {
        // Parses a line and returns a parsed dict
        let inst = get_instruction_type(&line);

        match inst.as_str() {
            "A" => parse_a_instruction(lno, line),
            "L" => parse_l_instruction(lno, line),
            "C" => parse_c_instruction(lno, line)
        }

    }
}



fn get_instruction_type(line: &String) -> String {
    match line {
        line if A_INSTRUCTION.is_match(line) => "A".to_string(),
        line if L_INSTRUCTION.is_match(line) => "L".to_string(),
        _ => "C".to_string()
    }
}
