/// Parses each line in the assembly code
/// Whitespaces and comments (starting //) are ignored.
/// Assume that the code has no syntax errors

use regex::Regex;
use lazy_static


lazy_static! {
    static COMMENT_REGEX: Regex = Regex::new(r"^.*(//.*)$").unwrap();
    static A_INSTRUCTION: Regex = Regex::new(r"^@(.+)$").unwrap();
    static L_INSTRUCTION: Regex = Regex::new(r"\((.+)\)$").unwrap();
    static C_INSTRUCTION: Regex = Regex::new(r"^(.+)(?:=?)(.+)(?:;?)(.+)$").unwrap();
}


pub fn parse_content(f_content: String) -> Vec<ParsedLine> {
    // Parses lines of assembly code
    let lines = f_content.split("\n");
    let mut parsed: Vec<ParsedLine> = Vec::new();

    for (lno, line) in lines.enumerate() {
        let lno = lno as u32;
        let line = line.to_string();
        let p = ParsedLine::new(lno, line);
        parsed.push(p)
    }

    parsed
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
        let line = remove_comments(line);
        let line = line.trim().to_string();

        ParsedLine {
            line: lno,
            inst: "".to_string(),
            addr: "".to_string(),
            label: "".to_string(),
            dest: "".to_string(),
            comp: "".to_string(),
            jump: "".to_string()
        }
    }
}


fn remove_comments(line: String) -> String {
    // Removes comments from a line of code
    let cleaned = COMMENT_REGEX.replace_all(&line, "").to_string();
    cleaned
}
