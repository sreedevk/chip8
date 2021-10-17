pub struct Assembler;

impl Assembler {
    pub fn assemble(program: String) {
        println!("STARTING CHIP8 ASSEMBLER: {}", program);
    }

    pub fn lex(line: String) {
        println!("{:#?}", line.split_whitespace());
    }
}
