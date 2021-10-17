#![allow(dead_code)]
#![allow(unused_imports)]

mod chip;
mod assembler;
use chip::VM;

fn main() {
    let mut machine: VM = VM::new();
    machine.boot(String::from("roms/pong.ch8"));
}
