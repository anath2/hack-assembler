/// Hack assembly language parser:
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


pub fn parse(file_content: String) {
    // Parse file resulting in an array
}


fn parse_line(line: String) {
    // Parse a line of assembly
}


fn get_type(line: str) {

}
