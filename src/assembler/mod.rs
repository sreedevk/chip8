use std::{env, fs};
pub struct Assembler;

impl Assembler {
    pub fn assemble(program: String) {
        let code = Assembler::read(program);
        println!("code: {}", code);
    }

    pub fn lex(line: String) {
        println!("{:#?}", line.split_whitespace());
    }

    pub fn read(program: String) -> String {
        fs::read_to_string(program.as_str())
            .expect("Couldn't Load Application")
    }
}
