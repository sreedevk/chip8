use std::{env, fs};
use std::collections::HashMap;
use regex::Regex;

type Token<'a> = HashMap<&'a str, &'a str>;
pub struct Assembler;

impl Assembler {
    pub fn assemble(program_file: String) {
        let code: String = Assembler::read_from_disk(program_file);
        let codelines: Vec<&str> = code.split("\n").collect();

        let _tokenized_codelines: Vec<Token> = codelines
            .iter()
            .map(|codeline| Assembler::reformat(*codeline) )
            .filter(|fmt_codeline| fmt_codeline.is_some() )
            .map(|codeline| Assembler::tokenize(String::from(codeline.unwrap())) )
            .collect();
    }

    fn read_from_disk(program: String) -> String {
        fs::read_to_string(program.as_str())
            .expect("Couldn't Load Application")
    }

    fn reformat(line: &str) -> Option<&str> {
        let formatted_line: Vec<&str> = line.split(";").collect();
        if formatted_line[0].len() > 1 {
            return Some(formatted_line[0]);
        }
        else {
            return None;
        }
    }

    fn tokenize(formatted_line: String) -> Token<'static> {
        let section_pattern  = Regex::new(r"^section .text").unwrap();
        let global_pattern   = Regex::new(r"^\s+global .*$").unwrap();
        let func_def_pattern = Regex::new(r"^\w+:").unwrap();

        if section_pattern.is_match(formatted_line.as_str()) { println!("{:#?}", &formatted_line); }
        if global_pattern.is_match(formatted_line.as_str()) { println!("{:#?}", &formatted_line); }
        if func_def_pattern.is_match(formatted_line.as_str()) { println!("{:#?}", &formatted_line); }

        let mut token: Token = HashMap::new();
        token.insert("function", "");
        token.insert("args", "");
        return token;
    }
}
