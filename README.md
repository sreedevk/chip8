## Yet Another Chip8 Emulator

Chip8 Virtual Machine Implementation in Rust (Originally in [C++](https://github.com/sreedevk/chip8/tree/cpp)).
This is just an academic venture into the world of emulators & VMs.

### Build & Run
```sh
# clone it 
git clone https://github.com/sreedevk/chip8

# jump into it
cd `chip8`

# build it
cargo build --release

# run the VM
./target/release/chip8 vm <path/to/rom>

# run the Assembler
./target/release/chip8 assembler <path/to/program>
```

### Notes
This is a work in progress. The VM is nearly complete.
