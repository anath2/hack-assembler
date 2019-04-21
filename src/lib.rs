// Parse hack assembly code into machine language code

pub struct Assembler<'a> {
    assembly: &'a str,
    machine: String,
}


impl<'a> Assembler<'a> {

    pub fn translate(assembly: &'a str) -> Assembler<'a> {

        parsed = parse(assembly: &'a str);
        let mut decoded = "";

        for line in parsed {
            let d = decode_inst(line);
            decoded.push_as_str(d);
        }

        return Assembler {
            assembly,
            machine: d
        }
    }

}
