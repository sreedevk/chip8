#![allow(dead_code)]
#![allow(unused_imports)]

mod chip;
mod assembler;
use chip::VM;
use assembler::*;
use std::env;
use std::io::Error;

fn main() -> Result<(), Error> {
    (app())();
    Ok(())
}

fn app() -> Box<dyn Fn()> {
    let args: Vec<String> = env::args().collect();
    let default_path = Box::new(|| { 
        println!("ERR: INVALID OPTS");
        print_app_info();
    });

    if args.len() < 2 { return default_path }

    match args[1].as_str() {
        "vm" => Box::new(move || {
            let mut machine: VM = VM::new();
            machine.boot(String::from(&args[2])); 
        }),
        "assembler" => Box::new(move || {
            Assembler::assemble(String::from(&args[2]));
        }),
        _ => default_path
    }    
}

fn print_app_info() {
    println!(
        "{}",
        r#"

         ██████╗██╗  ██╗██╗██████╗  █████╗ 
        ██╔════╝██║  ██║██║██╔══██╗██╔══██╗
        ██║     ███████║██║██████╔╝╚█████╔╝
        ██║     ██╔══██║██║██╔═══╝ ██╔══██╗
        ╚██████╗██║  ██║██║██║     ╚█████╔╝
         ╚═════╝╚═╝  ╚═╝╚═╝╚═╝      ╚════╝ 
                                           
        USAGE: chip8 [mode] <opts>
        modes: [vm | tassembler]
        vm: start the chip8 interpreter.
            opts: CHIP8_ROM
        assembler: start the chip8 assembler.
            opts: CHIP8_ASSEMBLY
        "#
    );
}
