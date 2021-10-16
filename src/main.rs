#![allow(dead_code)]

mod chip;
use chip::VM;

fn main() {
    let mut machine: VM = VM::new();
    machine.boot(String::from("roms/pong.ch8"));
}
