/// Parses each line in the assembly code
///
/// For each line:
///     - Find the command type A_TYPE (address) C_TYPE (command) or L_TYPE (label)
///     - symbol - gets the symbol if it's an A or L instruction
///     - dest - Destination if it's a C instruction
///     - comp - Computation mneumonic if it' a C instruction
///     - jmp  - Jump mneumonic if a jump mneumonic is included
///
/// Note:
/// Whitespaces and comments (starting //) are ignored.
/// Assume that the code has no syntax errors


use regex::Regex;


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
        let i_type = get_instruction_type(&line);
        let dest = get_dest(&line, &i_type);
        println!("{} - {} - {} - {}", linenum, i_type, dest, line);
    }
}


fn remove_comments(line: String) -> String {
    // Removes comments from a line of code
    let r = Regex::new(r"^.*(//.*)$").unwrap();
    let cleaned = r.replace_all(&line, "").to_string();
    cleaned
}


fn get_instruction_type(line: &String) -> String {
    // Checks whether it is A, L or C type instruction
    let char_vect: Vec<char> = line.chars().collect();
    let first_char = char_vect[0];

    if first_char == '@' {
        "A".to_string()
    } else if first_char == '(' {
        "L".to_string()
    } else {
        "C".to_string()
    }
}


fn get_dest(line: &String, inst_type: &String) -> String {
    // Get destination for the instruction
    if inst_type == "C" {
        let split: Vec<&str> = line.split("=").collect();
        split[0].to_string()
    } else {
        "".to_string()
    }
}
