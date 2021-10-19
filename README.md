## Yet Another Chip8 Emulator

Chip8 Virtual Machine Implementation in Rust (Originally in [C++](https://github.com/sreedevk/chip8/tree/cpp)).
This is just an academic venture into the world of emulators & VMs.

![chip8](https://user-images.githubusercontent.com/36154121/137998301-8c7918ea-08ab-40b0-8cf1-d68918ef57d3.gif)

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
