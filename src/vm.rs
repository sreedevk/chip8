use std::fmt::Display;

use crate::fonts::{FONT_SPRITES, FONT_SPRITES_SIZE};
use crate::rom::Ch8Rom;
use crate::display;
use anyhow::Result;
use rand::Rng;

pub type Byte = u8;
pub type Word = usize;
pub type Opcode = (u8, u8, u8, u8);

const STACK_DEPTH: usize = 16;
const MEMORY_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;
const KEY_COUNT: usize = 16;
const PROGRAM_START: usize = 0x200;

#[derive(Debug, Clone)]
pub struct VM {
    pub memory: Vec<Byte>,
    pub registers: [Byte; REGISTER_COUNT],
    pub index: Word,
    pub pc: Word,
    pub stack: [Word; STACK_DEPTH],
    pub stack_pointer: usize,
    pub keyboard: [bool; KEY_COUNT],
    pub delay_timer: Byte,
    pub sound_timer: Byte,
    pub rom: Ch8Rom,
    pub vram: [[Byte; 64]; 32],
}

impl Display for VM {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("memory: {:x?}", self.memory);

        Ok(())
    }
}

impl VM {
    pub fn new(rom: Ch8Rom) -> Result<Self> {
        Ok(Self {
            memory: vec![0; MEMORY_SIZE],
            registers: [0; REGISTER_COUNT],
            index: 0,
            pc: PROGRAM_START,
            stack: [0; STACK_DEPTH],
            stack_pointer: 0,
            keyboard: [false; KEY_COUNT],
            delay_timer: 0,
            sound_timer: 0,
            rom,
            vram: [[0; 64]; 32],
        })
    }

    pub fn initialize(&mut self) -> Result<()> {
        /* Load Rom */
        self.memory[PROGRAM_START..(PROGRAM_START + self.rom.size)]
            .copy_from_slice(&self.rom.memory);

        /* Load Sprites */
        self.memory[..FONT_SPRITES_SIZE].copy_from_slice(&FONT_SPRITES);

        Ok(())
    }

    pub fn fetch(&mut self) -> Result<(Opcode, Word)> {
        let bytecode = (self.memory[self.pc] as Word) << 8 | (self.memory[self.pc + 1] as Word);
        self.pc += 2;

        let nibbles = (
            ((bytecode & 0xF000) >> 12) as u8,
            ((bytecode & 0x0F00) >> 8) as u8,
            ((bytecode & 0x00F0) >> 4) as u8,
            ((bytecode & 0x000F) >> 0) as u8,
        );

        Ok((nibbles, bytecode))
    }

    pub fn tick(&mut self) -> Result<()> {
        let (opcode, bytecode) = self.fetch()?;
        self.execute(opcode, bytecode)?;
        self.update_timers();

        Ok(())
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn execute(&mut self, opcode: Opcode, bytecode: Word) -> Result<()> {
        match opcode {
            (0, 0, 0, 0) => Ok(()),
            (0, 0, 0xE, 0) => self.clear_screen(),
            (0, 0, 0xE, 0xE) => self.ret(),
            (1, _, _, _) => self.jmp(bytecode & 0x0FFF),
            (2, _, _, _) => self.bl(bytecode & 0xFFF),
            (3, x, _, _) => self.skip_if_vx_eq_nn(x as usize, (bytecode & 0xFF) as u8),
            (4, x, _, _) => self.skip_if_vx_ne_nn(x as usize, (bytecode & 0xFF) as u8),
            (5, x, y, 0) => self.skip_if_vx_eq_vy(x as usize, y as usize),
            (6, x, _, _) => self.set_vx_nn(x as usize, (bytecode & 0xFF) as u8),
            (7, x, _, _) => self.add_nn_to_vx(x as usize, (bytecode & 0xFF) as u8),
            (8, x, y, 0) => self.set_vx_vy(x as usize, y as usize),
            (8, x, y, 1) => self.vx_or_eq_vy(x as usize, y as usize),
            (8, x, y, 2) => self.vx_and_eq_vy(x as usize, y as usize),
            (8, x, y, 3) => self.vx_xor_eq_vy(x as usize, y as usize),
            (8, x, y, 4) => self.add_vy_to_vx(x as usize, y as usize),
            (8, x, y, 5) => self.sub_vy_from_vx(x as usize, y as usize),
            (8, x, _, 6) => self.rshift_vx(x as usize),
            (8, x, y, 7) => self.sub_vx_from_vy(x as usize, y as usize),
            (8, x, _, 0xE) => self.lshift_vx(x as usize),
            (9, x, y, 0) => self.skip_if_vx_ne_vy(x as usize, y as usize),
            (0xa, _, _, _) => self.set_vx_vi(bytecode & 0xFFF),
            (0xb, _, _, _) => self.jmp_v0_off_nnn(bytecode & 0xFFF),
            (0xc, x, _, _) => self.vx_rand_and_nn(x as usize, (bytecode & 0xFF) as u8),
            (0xd, x, y, n) => self.draw(x as usize, y as usize, n),
            (0xe, x, 9, 0xe) => self.skip_if_key(x as usize),
            (0xe, x, 0xa, 1) => self.skip_if_not_key(x as usize),
            (0xf, x, 0, 7) => self.set_vx_dt(x as usize),
            (0xf, x, 0, 0xa) => self.wait_for_key(x as usize),
            (0xf, x, 1, 5) => self.set_dt(x as usize),
            (0xf, x, 1, 8) => self.set_st(x as usize),
            (0xf, x, 1, 0xe) => self.add_vx_to_vi(x as usize),
            (0xf, x, 2, 9) => self.set_font_address_vi(x as usize),
            (0xf, x, 3, 3) => self.bcd_vx(x as usize),
            (0xf, x, 5, 5) => self.set_v0_sub_vx_info_i(x as usize),
            (0xf, x, 6, 5) => self.load_i_into_v0_vx(x as usize),
            (_, _, _, _) => unimplemented!(
                "Unimplemented Opcode: ({:#08x}, {:#08x}, {:#08x}, {:#08x})",
                opcode.0,
                opcode.1,
                opcode.2,
                opcode.3
            ),
        }
    }

    /* core functions */
    fn clear_screen(&mut self) -> Result<()> {
        self.vram = [[0; 64]; 32];

        Ok(())
    }

    fn ret(&mut self) -> Result<()> {
        self.stack_pointer -= 1;
        self.pc = self.stack[self.stack_pointer];

        Ok(())
    }

    fn jmp(&mut self, address: Word) -> Result<()> {
        self.pc = address;

        Ok(())
    }

    fn bl(&mut self, address: Word) -> Result<()> {
        self.stack[self.stack_pointer] = self.pc;
        self.stack_pointer += 1;
        self.pc = address;

        Ok(())
    }

    fn skip_if_vx_eq_nn(&mut self, x: usize, nn: Byte) -> Result<()> {
        if self.registers[x] == nn {
            self.pc += 2;
        }

        Ok(())
    }

    fn skip_if_vx_ne_nn(&mut self, x: usize, nn: Byte) -> Result<()> {
        if self.registers[x] != nn {
            self.pc += 2;
        }

        Ok(())
    }

    fn skip_if_vx_eq_vy(&mut self, x: usize, y: usize) -> Result<()> {
        if self.registers[x] == self.registers[y] {
            self.pc += 2;
        }

        Ok(())
    }

    fn set_vx_nn(&mut self, x: usize, nn: Byte) -> Result<()> {
        self.registers[x] = nn;

        Ok(())
    }

    fn add_nn_to_vx(&mut self, x: usize, nn: Byte) -> Result<()> {
        self.registers[x] = self.registers[x].wrapping_add(nn);

        Ok(())
    }

    fn set_vx_vy(&mut self, x: usize, y: usize) -> Result<()> {
        self.registers[x] = self.registers[y];

        Ok(())
    }

    fn vx_or_eq_vy(&mut self, x: usize, y: usize) -> Result<()> {
        self.registers[x] |= self.registers[y];

        Ok(())
    }

    fn vx_and_eq_vy(&mut self, x: usize, y: usize) -> Result<()> {
        self.registers[x] &= self.registers[y];

        Ok(())
    }

    fn vx_xor_eq_vy(&mut self, x: usize, y: usize) -> Result<()> {
        self.registers[x] ^= self.registers[y];

        Ok(())
    }

    fn add_vy_to_vx(&mut self, x: usize, y: usize) -> Result<()> {
        let (result, overflow) = self.registers[x].overflowing_add(self.registers[y]);
        self.registers[x] = result;
        self.registers[0xF] = overflow as Byte;

        Ok(())
    }

    fn sub_vy_from_vx(&mut self, x: usize, y: usize) -> Result<()> {
        let (result, overflow) = self.registers[x].overflowing_sub(self.registers[y]);
        self.registers[x] = result;
        self.registers[0xF] = !overflow as Byte;

        Ok(())
    }

    fn rshift_vx(&mut self, x: usize) -> Result<()> {
        self.registers[0xF] = self.registers[x] & 0x1;
        self.registers[x] >>= 1;

        Ok(())
    }

    fn sub_vx_from_vy(&mut self, x: usize, y: usize) -> Result<()> {
        let (result, overflow) = self.registers[y].overflowing_sub(self.registers[x]);
        self.registers[x] = result;
        self.registers[0xF] = !overflow as Byte;

        Ok(())
    }

    fn lshift_vx(&mut self, x: usize) -> Result<()> {
        self.registers[0xF] = (self.registers[x] & 0x80) >> 7;
        self.registers[x] <<= 1;

        Ok(())
    }

    fn skip_if_vx_ne_vy(&mut self, x: usize, y: usize) -> Result<()> {
        if self.registers[x] != self.registers[y] {
            self.pc += 2;
        }

        Ok(())
    }

    fn set_vx_vi(&mut self, address: Word) -> Result<()> {
        self.index = address;

        Ok(())
    }

    fn jmp_v0_off_nnn(&mut self, address: Word) -> Result<()> {
        self.pc = (self.registers[0] as Word) + address;

        Ok(())
    }

    fn vx_rand_and_nn(&mut self, x: usize, nn: Byte) -> Result<()> {
        let mut rng = rand::thread_rng();
        let rand: Byte = rng.gen();
        self.registers[x] = rand & nn;

        Ok(())
    }

    fn draw(&mut self, x: usize, y: usize, n: u8) -> Result<()> {
        let mut collision = false;
        for i in 0..n {
            let byte = self.memory[self.index + i as usize];
            for j in 0..8 {
                let bit = (byte >> (7 - j)) & 0x1;
                let x = (self.registers[x] as usize + j) % 64;
                let y = (self.registers[y] as usize + i as usize) % 32;
                let prev_bit = self.vram[y][x];
                self.vram[y][x] ^= bit;
                if prev_bit == 1 && self.vram[y][x] == 0 {
                    collision = true;
                }
            }
        }
        self.registers[0xF] = collision as Byte;

        Ok(())
    }

    fn skip_if_key(&mut self, x: usize) -> Result<()> {
        if self.keyboard[self.registers[x] as usize] {
            self.pc += 2;
        }

        Ok(())
    }

    fn skip_if_not_key(&mut self, x: usize) -> Result<()> {
        if !self.keyboard[self.registers[x] as usize] {
            self.pc += 2;
        }

        Ok(())
    }

    fn set_vx_dt(&mut self, x: usize) -> Result<()> {
        self.registers[x] = self.delay_timer;

        Ok(())
    }

    fn wait_for_key(&mut self, x: usize) -> Result<()> {
        for i in 0..KEY_COUNT {
            if self.keyboard[i] {
                self.registers[x] = i as Byte;
                return Ok(());
            }
        }
        self.pc -= 2;

        Ok(())
    }

    fn set_dt(&mut self, x: usize) -> Result<()> {
        self.delay_timer = self.registers[x];

        Ok(())
    }

    fn set_st(&mut self, x: usize) -> Result<()> {
        self.sound_timer = self.registers[x];

        Ok(())
    }

    fn add_vx_to_vi(&mut self, x: usize) -> Result<()> {
        self.index += self.registers[x] as Word;

        Ok(())
    }

    fn set_font_address_vi(&mut self, x: usize) -> Result<()> {
        self.index = (self.registers[x] as Word) * 5;

        Ok(())
    }

    fn bcd_vx(&mut self, x: usize) -> Result<()> {
        let vx = self.registers[x];
        self.memory[self.index] = vx / 100;
        self.memory[self.index + 1] = (vx / 10) % 10;
        self.memory[self.index + 2] = vx % 10;

        Ok(())
    }

    fn set_v0_sub_vx_info_i(&mut self, x: usize) -> Result<()> {
        for i in 0..=x {
            self.memory[self.index + i] = self.registers[i];
        }

        Ok(())
    }

    fn load_i_into_v0_vx(&mut self, x: usize) -> Result<()> {
        for i in 0..=x {
            self.registers[i] = self.memory[self.index + i];
        }

        Ok(())
    }
}
