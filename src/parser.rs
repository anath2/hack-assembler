/// Parser:
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


fn trim_line(line: String) {
    // Removes whitespaces and comments from the instruction
}


fn get_instruction_type(line: String) -> char{
    // Checks for A instruction, C instruction or L instruction
}


fn get_destination_c(line: String) -> String {
    // Gets destination if it is a C instruction
}


fn get_computation_c(line: String) -> String {
    // Gets computation mneumonic if it's a C instruction
}


fn get_jump_c(line: String) {
    // Gets jump  if it's a c instruction
}
