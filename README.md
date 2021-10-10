## Yet Another Chip8 Emulator

Chip8 Virtual Machine Implementation in C++ (originally in C).
The Purpose of this implementation if just for me to understand
the internal workings of the Chip8 VM.

![2021-10-10-222833_1643x885_scrot](https://user-images.githubusercontent.com/36154121/136705836-20504304-e484-4b7a-a959-07df580003ce.png)

### Build & Run
```sh
# clone it 
git clone https://github.com/sreedevk/chip8

# jump into it
cd `chip8`

# add src & build directories
cmake -S src/ -B build/
# build it
cmake --build build/ --clean-first

# run it with a ch8 rom. Sample roms are included in the `roms` directory
./build/chip8 <path/to/rom>
```

### Notes

There are some known issues in this implementation. The draw opcode has issues.
I am working on fixing them. Meanwhile, Feel free to enjoy the VM inspector that
prints out the internal state of the VM after the execution of every instruction.
Screenshot of the VM Inspector has been attached above.
