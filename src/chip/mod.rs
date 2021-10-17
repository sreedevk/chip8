use std::io::prelude::*;
use std::fs::File;
use std::{io, char};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use ncurses::*;

type Sprite = [u8; 5];
type Display = [[u8; 64]; 32];

const PROGRAM_START_POINTER: usize = 0x200;
const SYS_REG_ADDR: usize = 0xF;
const STACK_SIZE: usize = 16;
const SPRITE_START_ADDR: usize = 0x050;
const CHAR_SPRITE_SIZE: usize = 5;

const UNICODE_PIXELS: [char; 65] = [
    '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 'o', 'a', 'h', 'k', 'b', 'd', 'p', 'q',
    'w', 'm', 'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X', 'z', 'c', 'v', 'u', 'n', 'x',
    'r', 'j', 'f', 't', '/', '|', '(', ')', '1', '{', '}', '[', ']', '?', '-', '_', '+',
    '~', '<', '>', 'i', '!', 'l', 'I', ';', ':', ',', '^', '`'
];

const SPRITES: [Sprite; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

struct DisplayManager;

#[derive(Debug)]
pub struct VM {
    pub pc:          usize,
    pub sp:          usize,
    pub i:           u16,
    pub sound_timer: u8,
    pub delay_timer: u8,
    pub memory:      [u8; 4096],
    pub registers:   [u8; 16],
    pub stack:       [u16; STACK_SIZE],
    pub display:     Display,
    pub keyboard:    [u8; 16],
    pub running:     bool
}


impl DisplayManager {
    fn render_gfx(machine: &VM) {
        mv(0, 0);

        for (row_index, row) in machine.display.iter().enumerate() {
            for (pix_index, pixel) in row.iter().enumerate() {
                mv(row_index as i32, pix_index as i32);
                addch(if *pixel > 0 { '█' as u32 } else { ' ' as u32 });
            }
        }
        refresh();
    }

    fn init_display() {
        let locale_conf = LcCategory::all;
        setlocale(locale_conf, "en_US.UTF-8");

        /* Setup ncurses. */
        initscr();
        raw();

        /* Require input within 2 seconds. */
        // halfdelay(20);
        /* Enable mouse events. */
        mousemask(ALL_MOUSE_EVENTS as mmask_t, None);

        /* Allow for extended keyboard (like F1). */
        keypad(stdscr(), true);
        noecho();
    }
}

pub struct Opcode {
    pub raw: u16,
    pub circuit: Box<dyn Fn(&mut VM, u16)>
}

impl VM {
    pub fn new() -> VM {
        return VM {
            pc:          PROGRAM_START_POINTER,
            sp:          0x0,
            i:           0x0,
            sound_timer: 0x0,
            delay_timer: 0x0,
            memory:      [0; 4096],
            registers:   [0; 16],
            stack:       [0; 16],
            display:     [[0; 64]; 32],
            keyboard:    [0; 16],
            running:     false
        }
    }

    /* INTERFACE */

    pub fn boot(&mut self, program_path: String) {
        DisplayManager::init_display();

        self.load_program(program_path);
        self.running = true;
        while self.running {
            let exec_circuit = self.decode(self.fetch());
            match self.execute(exec_circuit) {
                Err(_) => { self.running = false },
                Ok(_) => ()
            }
            self.post_cycle_ops();
        }
    }

    fn load_program(&mut self, program_path: String) {
        let program_file  = File::open(program_path).unwrap();
        let program_size = program_file.metadata().unwrap().len();
        let mut program: Vec<u8> = Vec::with_capacity(program_size as usize);

        for byte in program_file.bytes() {
            program.push(byte.unwrap());
        }

        let mut copy_program_pointer = PROGRAM_START_POINTER;
        for (lower, upper) in program.iter().tuples() {
            self.memory[copy_program_pointer]       = *upper;
            self.memory[(copy_program_pointer + 1)] = *lower;
            copy_program_pointer += 2;
        }
    }

    /* CORE FUNCTIONS */
    fn post_cycle_ops(&mut self) {
        DisplayManager::render_gfx(self);
    }

    fn clear_display(&mut self) {
        self.display = [[0; 64]; 32];
    }

    fn incr_pc(&mut self) {
        self.pc += 2;
    }

    fn draw_sprite(&mut self, _opcode: u16) {
        /* DRAW SPRITE ONTO DISPLAY MEMORY */
    }


    /* PROCESS CYCLE */

    fn fetch(&self) -> u16 {
        return ((self.memory[self.pc + 1] as u16) << 8) | (self.memory[self.pc] as u16);
    }

    fn decode(&mut self, opcode: u16) -> Opcode {
        Opcode {
            raw: opcode,
            circuit: self.build_circuit(opcode)
        }
    }

    fn execute(&mut self, token: Opcode) -> Result<(), io::Error> {
        (token.circuit)(self, token.raw);
        self.incr_pc();
        Ok(())
    }

    /* CIRCUITRY */

    fn build_circuit(&mut self, opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        match opcode & 0xF000u16 {
            0x0000u16 => self.generate_circuit_class0(opcode),
            0x1000u16 => self.generate_circuit_class1(opcode),
            _ => Box::new(|_machine, _code|  () )
        }
    }

    fn generate_circuit_class0(&mut self, opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        match opcode & 0x0FFFu16 {
            /*00E0 - CLS*/
            0x00E0u16 => Box::new(|machine, _code| { machine.clear_display(); }) ,
            /*00EE - RET*/
            0x00EEu16 => { Box::new(|machine, _code| {
                machine.pc = machine.stack[machine.sp] as usize;
                machine.sp -= 1; })
            }
            _ => Box::new(|_machine, code| {
                println!("RUNNING ROUTINE: {:#08x}", code);
            })
        }

    }

    fn generate_circuit_class1(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        /*1nnn - JP*/
        Box::new(|machine, code|  { machine.pc = (code & 0x0FFFu16) as usize } )
    }

    fn generate_circuit_class2(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        /*2nnn - CALL*/
        Box::new(|machine, code| {
            machine.sp += 1;
            machine.stack[machine.sp] = machine.pc as u16;
            machine.pc = (code & 0x0FFFu16) as usize;
        })
    }

    fn generate_circuit_class3(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        /* 3XKK - SE VX, BYTE */
        Box::new(|machine, code|  {
            let comp_val: u8 = (code & 0x00FF) as u8;
            let register_value: u8 = machine.registers[((code & 0xF00) >> 8) as usize];
            if comp_val == register_value { machine.incr_pc(); }
        })
    }

    fn generate_circuit_class4(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        /*4XKK - SNE VX, BYTE*/
        Box::new(|machine, code|  {
            let comp_val: u8 = (code & 0x00FF) as u8;
            let register_value: u8 = machine.registers[((code & 0xF00) >> 8) as usize];
            if register_value != comp_val { machine.incr_pc(); }
        })
    }

    fn generate_circuit_class5(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        /*5xy0 - SE Vx, Vy*/
        Box::new(|machine, code| {
            let x_value: u8 = machine.registers[((code & 0x0F00) >> 8) as usize];
            let y_value: u8 = machine.registers[((code & 0x00F0) >> 4) as usize];
            if x_value == y_value { machine.incr_pc(); }
        })
    }

    fn generate_circuit_class6(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        Box::new(|machine, code|  {
            machine.registers[((code & 0x0F00) >> 8) as usize] = (code & 0x00FF) as u8;
        })
    }

    fn generate_circuit_class7(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        /*7XKK - ADD VX, BYTE*/
        Box::new(|machine, code|  {
            let add_byte: u8 = (code as u8) & 0x00FF;
            let reg_addr: usize = ((code & 0x0F00) >> 8) as usize;
            let reg_val: u8 = machine.registers[reg_addr];
            if add_byte > 0xFF - reg_val {
                machine.registers[SYS_REG_ADDR] = 1;
            }
            else {
                machine.registers[SYS_REG_ADDR] = 0;
            }
            machine.registers[reg_addr] = reg_val + add_byte;
        })
    }

    fn generate_circuit_class8(&mut self, opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        match opcode & 0x000Fu16 {
            /*8XY0 - LD VX, VY*/
            0x0000u16 => Box::new(|machine, code|  {
                machine.registers[((code & 0x0F00u16) >> 8) as usize] = 
                    machine.registers[((code & 0x00F0u16) >> 4) as usize];
            }),
            0x0001u16 => Box::new(|machine, code| {
                /*8XY1 - OR VX, VY*/
                let xaddr: usize = ((code & 0x0F00u16) >> 8) as usize;
                let yaddr: usize = ((code & 0x00F0u16) >> 4) as usize;
                let xsetval: u8 = machine.registers[xaddr] | machine.registers[yaddr];
                machine.registers[xaddr] = xsetval;
            }),
            0x0002u16 => Box::new(|machine, code| {
                let xaddr = ((code & 0x0F00u16) >> 8) as usize;
                let yaddr = ((code & 0x00F0u16) >> 4) as usize;
                let xsetval = machine.registers[xaddr] & machine.registers[yaddr];
                machine.registers[xaddr] = xsetval;
            }),
            0x0003u16 => Box::new(|machine, code| {
                let xaddr = ((code & 0x0F00u16) >> 8) as usize;
                let yaddr = ((code & 0x00F0u16) >> 4) as usize;
                let xsetval = machine.registers[xaddr] ^ machine.registers[yaddr];
                machine.registers[xaddr] = xsetval;
            }),
            0x0004u16 => Box::new(|machine, code| {
                /*8XY4 - ADD VX, VY*/
                let xaddr = ((code & 0x0F00u16) >> 8) as usize;
                let yaddr = ((code & 0x00F0u16) >> 4) as usize;
                let yval  = machine.registers[yaddr];
                let xval  = machine.registers[xaddr];
                machine.registers[SYS_REG_ADDR] = if yval > 0xFFu8 - xval { 1 } else { 0 };
                machine.registers[xaddr] = yval + xval;
            }),
            0x0005u16 => Box::new(|machine, code| {
                /*8XY5 - SUB VX, VY*/
                let xaddr = ((code & 0x0F00u16) >> 8) as usize;
                let yaddr = ((code & 0x00F0u16) >> 4) as usize;
                let xval  = machine.registers[xaddr];
                let yval  = machine.registers[yaddr];
                machine.registers[SYS_REG_ADDR] = if xval > yval { 1 } else { 0 };
                machine.registers[xaddr] = xval - yval;
            }),
            0x0006u16 => Box::new(|machine, code| {
                /*8XY6 - SHR VX {, VY}*/
                let xaddr = ((code & 0x0F00u16) >> 8) as usize;
                let xval  = machine.registers[xaddr];
                machine.registers[SYS_REG_ADDR] = if xval & 0x01 > 0 { 1 } else { 0 };
                machine.registers[xaddr] = xval >> 1;
            }),
            0x0007u16 => Box::new(|machine, code| {
                /*8XY7 - SUBN VX, VY*/
                let xaddr = ((code & 0x0F00u16) >> 8) as usize;
                let yaddr = ((code & 0x00F0u16) >> 4) as usize;
                let xval  = machine.registers[xaddr];
                let yval = machine.registers[yaddr];
                machine.registers[SYS_REG_ADDR] =  if yval > xval { 1 } else { 0 };
                machine.registers[xaddr] = yval - xval;
            }),
            0x000Eu16 => Box::new(|machine, code| {
                /*8XYE - SHL VX {, VY}*/
                let xaddr = ((code & 0x0F00u16) >> 8) as usize;
                let xval  = machine.registers[xaddr];
                machine.registers[SYS_REG_ADDR] = if xval & 0x01 > 0 { 1 } else { 0 };
                machine.registers[xaddr] = xval >> 1;
            }),
            _ => Box::new(|_machine, code|  {
                println!("UNSUPPORTED OPCODE: {:#?}", code);
            } )
        }
    }

    fn generate_circuit_class9(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        /*9XY0 - SNE VX, VY */
        Box::new(|machine, code| {
            let xaddr = ((code & 0x0F00u16) >> 8) as usize;
            let yaddr = ((code & 0x00F0u16) >> 4) as usize;
            if machine.registers[xaddr] != machine.registers[yaddr] {
                machine.incr_pc();
            }
        })
    }

    fn generate_circuit_classa(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        Box::new(|machine, code| {
            machine.i = code & 0x0FFFu16;
        })
    }

    fn generate_circuit_classb(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        Box::new(|machine, code| {
            let jp_addr = ((code & 0x0FFFu16) + (machine.registers[0] as u16)) as usize;
            machine.pc = jp_addr;
        })
    }

    fn generate_circuit_classc(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        /*CXKK - RND VX, BYTE*/
        Box::new(|machine, code| {
            let mut rng = thread_rng();
            let random_num: u16 = rng.gen();
            let regaddr = ((code & 0x0F00u16) >> 8) as usize;
            let ibytes  = code & 0x00FFu16;
            machine.registers[regaddr] = ((random_num % 0xFFu16) & ibytes) as u8;
        })
    }

    fn generate_circuit_classd(&mut self, _opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        /*
         * DXYN - DRW VX, VY, NIBBLE 
         * Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
         * The interpreter reads n bytes from memory, starting at the address stored in I.
         * These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen.
         * If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
         * If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. 
         * See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
         * */
        Box::new(|machine, code| {
            machine.draw_sprite(code);
        })
    }

    fn generate_circuit_classe(&mut self, opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        match opcode & 0x00FFu16 {
            /*EX9E - SKP VX*/
            0x009Eu16 => Box::new(|machine, code| {
                let keyaddr = machine.registers[((code & 0x0F00u16) >> 8) as usize] as usize;
                let _key     = machine.registers[keyaddr];
                // if(this->sys->keyboard->checkKeyState(key)==1u) this->sys->incr_pc();
            }),
            /*ExA1 - SKNP Vx*/
            0x00A0u16 => Box::new(|machine, code| {
                let keyaddr: usize = machine.registers[((code & 0x0F00u16) >> 8) as usize] as usize;
                let _key     = machine.registers[keyaddr];
                // if(this->sys->keyboard->checkKeyState(key)==0u) this->sys->incr_pc();
            }),
            _ => Box::new(|_machine, code|  {
                println!("UNSUPPORTED OPCODE: {:#?}", code);
            })
        }
    }

    fn generate_circuit_classf(&mut self, opcode: u16) -> Box<dyn Fn(&mut VM, u16)> {
        match opcode & 0x00FFu16 {
            /*Fx07 - LD Vx, DT*/
            0x0007u16 => Box::new(|machine, code| {
                machine.registers[((code & 0x0F00u16) >> 8) as usize] = machine.delay_timer;
            }),
            /*Fx0A - LD Vx, K*/
            0x000Au16 => Box::new(|_machine, _code| {
                println!("INSTRUCTION NOT IMPLEMENTED");
                // let key = this->sys->keyboard->expectKeyDown();
                // machine.registers[((opcode & 0x0F00u) >> 8) as usize] = key;
            }),
            /*Fx15 - LD DT, Vx*/
            0x0015u16 => Box::new(|machine, code| {
                machine.delay_timer = machine.registers[((code & 0x0F00u16) >> 8) as usize];
            }),
            /*Fx18 - LD ST, Vx*/
            0x0018u16 => Box::new(|machine, code| {
                machine.sound_timer = machine.registers[((code & 0x0F00u16) >> 8) as usize];
            }),
            /*Fx1E - ADD I, Vx*/
            0x001Eu16 => Box::new(|machine, code| {
                machine.i += machine.registers[((code & 0x0F00u16) >> 8) as usize] as u16;
            }),
            /*Fx29 - LD F, Vx; Set I = location of sprite for digit Vx.*/
            0x0029u16 => Box::new(|machine, code| {
                let char_addr = machine.registers[((code & 0x0F00u16) >> 8) as usize] as usize;
                machine.i = (SPRITE_START_ADDR + (char_addr * CHAR_SPRITE_SIZE)) as u16;
            }),
            /*Fx33 - LD B, Vx*/
            0x0033u16 => Box::new(|machine, code| {
                let vx = machine.registers[((code & 0x0F00u16) >> 8) as usize];
                let index: usize = machine.i as usize;
                machine.memory[index]   = vx / 100;
                machine.memory[index + 1] = (vx % 100) / 10;
                machine.memory[index + 1] = vx % 10;
            }),
            /*Fx55 - LD [I], Vx*/
            0x0055u16 => Box::new(|machine, code| {
                let vx_addr = ((code & 0x0F00u16) >> 8) as usize;
                let index: usize = machine.i as usize;
                for index_offset in 0..vx_addr {
                    machine.memory[index + index_offset] = machine.registers[index_offset];
                }
            }),
            /*Fx65 - LD Vx, [I]*/
            0x0065u16 => Box::new(|machine, code| {
                let vx_addr = ((code & 0x0F00u16) >> 8) as usize;
                for index_offset in 0..vx_addr {
                    machine.registers[index_offset] = machine.memory[(machine.i as usize) + index_offset];
                }
            }),
            _ => Box::new(|_machine, code|  {
                println!("UNSUPPORTED OPCODE: {:#?}", code);
            })
        }
    }
}
