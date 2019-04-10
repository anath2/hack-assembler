/// Code Module translates assembly mneumonics
/// into appropriate machine code

/// Instructions are of the format

/// 1 1 1 a   c1 c2 c3 c4 c5 c6 d1 d2   d3 j1 j2 j3 (C Instruction)
/// 0 a1 a2 a3   a4 a5 a6 a7  a8 a9 a10 a11  a12 a13 a14 a15 (A Instruction)

/// The 'j' bits ditermine the jump instruction
/// the 'd' bits decide the destination
/// 'a' bit ditermines whether the compute bits operate on A register
/// or M register
/// The most significant bit is for the instruction type 0 being an A instruction and
/// C instruction. "L" type instructions are ignored
/// A type instructions have the most significant bit set to zero and a
/// 15 bit address

mod parser;


struct comp {
    "0"  :   "101010",
    "1"  :   "111111",
    "-1" :   "111110",
    "D"  :   "001100",
    "A"  :   "110000",
    "!D" :   "001101",
    "!A" :   "110001",
    "-D" :   "001111",
    "-A" :   "110011",
    "D+1":   "011111",
    "A+1":   "110111",
    "D-1":   "001110",
    "A-1":   "110010",
    "D+A":   "000010",
    "D-A":   "010011",
    "A-D":   "000111",
    "D&A":   "000000",
    "D|A":   "010101"
};


struct dest {
    "M"  : "001",
    "D"  : "010",
    "MD" : "011",
    "A"  : "100",
    "AM" : "101",
    "AD" : "110",
    "AMD": "111"
};


struct jump {
    "JGT": "001",
    "JEQ": "010",
    "JGE": "011",
    "JLT": "100",
    "JNE": "101",
    "JLE": "110",
    "JMP": "111"
};


pub fn decode (parsed: Vec<ParsedLine>) {
    // Translates parsed assembly language code into uuu
    // machine language
    let mut decoded = String::new();

    for line in &parsed {
        let decoded_line = match line.inst.as_str() {
            "A" => decode_a(line) + "\n",
            "C" => decode_c(line) + "\n",
            _   => ""  // Ignore other instruction types
        };

        if decoded_line.len() > 0 {
            decoded.push_str(decoded_line);
        }
    }

    decoded
}


fn decode_a(parsed_line: ParsedLine) -> &str {
    // Translates A instructions to machine code
    let binary_str = format!("{:015b}", parsed_line.addr);  // Since address is 15 bits

    "0" + binary_str
}


fn decode_c(parsed_line: ParsedLine) -> &str {
    // Translates C instructions to machine code
    let dest_bits = decode_dest(parsed_line);
    let comp_bits = decode_comp(parsed_line);
    let jump_bits = decode_jump(parsed_line);

    "111" + dest_bits + comp_bits + jump_bits
}


fn decode_dest(parsed_line: ParsedLine) -> &str {
    let dest = parsed_line.dest.trim();

    match dest {
        "M"  => "001",
        "D"  => "010",
        "MD" => "011",
        "A"  => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD"=> "111"
    }
}


fn decode_comp(parsed_line: ParsedLine) -> &str {
    // Clean up the comp string
    let comp_chars = parsed_line.comp.chars();
    let filtered = comp_chars.filter(|x| *x != ' ');

    let mut cleaned_string = String::new();

    for c in filtered {
        cleaned_string.push(f);
    }

    match cleaned_string.as_str() {
        "0"   =>   "101010",
        "1"   =>   "111111",
        "-1"  =>   "111110",
        "D"   =>   "001100",
        "A"   =>   "110000",
        "!D"  =>   "001101",
        "!A"  =>   "110001",
        "-D"  =>   "001111",
        "-A"  =>   "110011",
        "D+1" =>   "011111",
        "A+1" =>   "110111",
        "D-1" =>   "001110",
        "A-1" =>   "110010",
        "D+A" =>   "000010",
        "D-A" =>   "010011",
        "A-D" =>   "000111",
        "D&A" =>   "000000",
        "D|A" =>   "010101"
    }

}


fn decode_jump(parsed_line: ParsedLine) -> &str {
    let jump = parsed_line.jump.trim();

    match jump {
        "JGT": "001",
        "JEQ": "010",
        "JGE": "011",
        "JLT": "100",
        "JNE": "101",
        "JLE": "110",
        "JMP": "111"
    }
}
