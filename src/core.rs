#![allow(unused)]

use itertools::Itertools;
use rand::random;

use crate::fonts::{FONT_SPRITES, FONT_SPRITES_SIZE};
use anyhow::Result;
const INIT_ADDR: u16 = 0x200;
const RAM_SIZE: usize = 4096;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
const V_REG_COUNT: usize = 16;
const MAX_STACK_DEPTH: usize = 16;
const KEYS_COUNT: usize = 16;

pub struct VM {
    pub pc: u16,
    pub ram: [u8; RAM_SIZE],
    pub screen: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pub v_reg: [u8; V_REG_COUNT],
    pub i_reg: u16,
    pub stack: [u16; MAX_STACK_DEPTH],
    pub keyboard: [bool; KEYS_COUNT],
    pub sp: u16,
    pub dt: u8,
    pub st: u8,
}

impl std::fmt::Debug for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let current_opcode = self.ram[self.pc as usize];
        write!(f, "Opcode: {:#08x} | ", current_opcode);
        write!(f, "Program Counter: {:#08x} | ", self.pc)
    }
}

impl VM {
    pub fn new() -> Self {
        Self {
            pc: INIT_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            v_reg: [0; V_REG_COUNT],
            i_reg: 0,
            stack: [0; MAX_STACK_DEPTH],
            sp: 0,
            keyboard: [false; KEYS_COUNT],
            dt: 0,
            st: 0,
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) -> Result<()> {
        let start_addr = INIT_ADDR as usize;
        let end_addr = (INIT_ADDR as usize) + data.len();
        self.ram[start_addr..end_addr].copy_from_slice(data);

        Ok(())
    }

    pub fn load_sprites(&mut self) {
        self.ram[..FONT_SPRITES_SIZE].copy_from_slice(&FONT_SPRITES);
    }

    pub fn display(&self) -> &[bool] {
        &self.screen
    }

    pub fn presskey(&mut self, idx: usize, pressed: bool) {
        self.keyboard[idx] = pressed;
    }

    pub fn tick(&mut self) -> Result<()> {
        let opcode = self.fetch()?;
        let execution_result = self.execute(opcode)?;

        Ok(())
    }

    pub fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // BEEP
            }
            self.st -= 1;
        }
    }

    fn decode(opcode: u16) -> (u16, u16, u16, u16) {
        let digit1 = (opcode & 0xF000) >> 12;
        let digit2 = (opcode & 0x0F00) >> 8;
        let digit3 = (opcode & 0x00F0) >> 4;
        let digit4 = (opcode & 0x000F);

        (digit1, digit2, digit3, digit4)
    }

    fn execute(&mut self, opcode: u16) -> Result<()> {
        match Self::decode(opcode) {
            (0, 0, 0, 0) => Ok(()),
            (0, 0, 0xE, 0) => self.clear_screen(),
            (0, 0, 0xE, 0xE) => self.ret(),
            (1, _, _, _) => self.jmp(opcode & 0x0FFF),
            (2, _, _, _) => self.bl(opcode & 0xFFF),
            (3, x, _, _) => self.skip_if_vx_eq_nn((x as usize), (opcode & 0xFF) as u8),
            (4, x, _, _) => self.skip_if_vx_ne_nn((x as usize), (opcode & 0xFF) as u8),
            (5, x, y, 0) => self.skip_if_vx_eq_vy((x as usize), (y as usize)),
            (6, x, _, _) => self.set_vx_nn((x as usize), (opcode & 0xFF) as u8),
            (7, x, _, _) => self.add_nn_to_vx((x as usize), (opcode & 0xFF) as u8),
            (8, x, y, 0) => self.set_vx_vy((x as usize), (y as usize)),
            (8, x, y, 1) => self.vx_or_eq_vy((x as usize), (y as usize)),
            (8, x, y, 2) => self.vx_and_eq_vy((x as usize), (y as usize)),
            (8, x, y, 3) => self.vx_xor_eq_vy((x as usize), (y as usize)),
            (8, x, y, 4) => self.add_vy_to_vx((x as usize), (y as usize)),
            (8, x, y, 5) => self.sub_vy_from_vx((x as usize), (y as usize)),
            (8, x, _, 6) => self.rshift_vx((x as usize)),
            (8, x, y, 7) => self.sub_vx_from_vy((x as usize), (y as usize)),
            (8, x, _, 0xE) => self.lshift_vx((x as usize)),
            (9, x, y, 0) => self.skip_if_vx_ne_vy((x as usize), (y as usize)),
            (0xa, _, _, _) => self.set_vx_vi((opcode & 0xFFF)),
            (0xb, _, _, _) => self.jmp_v0_off_nnn((opcode & 0xFFF)),
            (0xc, x, _, _) => self.vx_rand_and_nn((x as usize), (opcode & 0xFF) as u8),
            (0xd, x, y, n) => self.draw((x as usize), (y as usize), n),
            (0xe, x, 9, 0xe) => self.skip_if_key((x as usize)),
            (0xe, x, 0xa, 1) => self.skip_if_not_key((x as usize)),
            (0xf, x, 0, 7) => self.set_vx_dt((x as usize)),
            (0xf, x, 0, 0xa) => self.wait_for_key((x as usize)),
            (0xf, x, 1, 5) => self.set_dt((x as usize)),
            (0xf, x, 1, 8) => self.set_st((x as usize)),
            (0xf, x, 1, 0xe) => self.add_vx_to_vi((x as usize)),
            (0xf, x, 2, 9) => self.set_font_address_vi((x as usize)),
            (0xf, x, 3, 3) => self.bcd_vx((x as usize)),
            (0xf, x, 5, 5) => self.set_v0_sub_vx_info_i((x as usize)),
            (0xf, x, 6, 5) => self.load_i_into_v0_vx((x as usize)),
            (_, _, _, _) => unimplemented!("Unimplemented Opcode: {:#08x}", opcode),
        }
    }

    fn load_i_into_v0_vx(&mut self, vx: usize) -> Result<()> {
        let ival = self.i_reg as usize;
        for idx in 0..=vx {
            self.v_reg[idx] = self.ram[ival + idx];
        }

        Ok(())
    }

    fn set_v0_sub_vx_info_i(&mut self, vx: usize) -> Result<()> {
        let ival = self.i_reg as usize;
        for idx in 0..=vx {
            self.ram[ival + idx] = self.v_reg[idx];
        }

        Ok(())
    }

    fn bcd_vx(&mut self, vx: usize) -> Result<()> {
        let cvx = self.v_reg[vx] as f32;
        let hs = (cvx / 100.0).floor() as u8;
        let ts = ((cvx / 10.0) % 10.0).floor() as u8;
        let os = (cvx % 10.0) as u8;

        self.ram[self.i_reg as usize] = hs;
        self.ram[(self.i_reg + 1) as usize] = ts;
        self.ram[(self.i_reg + 2) as usize] = os;

        Ok(())
    }

    fn set_font_address_vi(&mut self, vx: usize) -> Result<()> {
        let c = self.v_reg[vx] as u16;
        self.i_reg = c * 5;
        Ok(())
    }

    fn add_vx_to_vi(&mut self, vx: usize) -> Result<()> {
        let cvx = self.v_reg[vx] as u16;
        self.i_reg = self.i_reg.wrapping_add(cvx);

        Ok(())
    }

    fn set_st(&mut self, vx: usize) -> Result<()> {
        self.st = self.v_reg[vx];

        Ok(())
    }

    fn set_dt(&mut self, vx: usize) -> Result<()> {
        self.dt = self.v_reg[vx];

        Ok(())
    }

    fn wait_for_key(&mut self, vx: usize) -> Result<()> {
        let mut pressed = false;
        for i in 0..self.keyboard.len() {
            if self.keyboard[i] {
                self.v_reg[vx] = i as u8;
                pressed = true;
                break;
            }
        }

        if !pressed {
            self.pc -= 2;
        }
        Ok(())
    }

    fn set_vx_dt(&mut self, vx: usize) -> Result<()> {
        self.v_reg[vx] = self.dt;

        Ok(())
    }

    fn skip_if_not_key(&mut self, vx: usize) -> Result<()> {
        let cvx = self.v_reg[vx];
        let key = self.keyboard[cvx as usize];
        if !key {
            self.pc += 2;
        }
        Ok(())
    }

    fn skip_if_key(&mut self, vx: usize) -> Result<()> {
        let cvx = self.v_reg[vx];
        let key = self.keyboard[cvx as usize];
        if key {
            self.pc += 2;
        }

        Ok(())
    }

    fn draw(&mut self, vx: usize, vy: usize, n: u16) -> Result<()> {
        let x_coordinate = self.v_reg[vx] as u16;
        let y_coordinate = self.v_reg[vy] as u16;
        let mut flipped = false;

        for y_line in 0..n {
            let addr = self.i_reg + y_line as u16;
            let pixels = self.ram[addr as usize];
            for x_line in 0..8 {
                if (pixels & (0b1000_0000 >> x_line)) != 0 {
                    let x = (x_coordinate + x_line) as usize % DISPLAY_WIDTH;
                    let y = (y_coordinate + y_line) as usize % DISPLAY_HEIGHT;

                    let idx = x + DISPLAY_WIDTH * y;
                    flipped |= self.screen[idx];

                    self.screen[idx] ^= true;
                }
            }
        }

        self.v_reg[0xf] = if flipped { 1 } else { 0 };

        Ok(())
    }

    fn vx_rand_and_nn(&mut self, vx: usize, nn: u8) -> Result<()> {
        let rng: u8 = random();
        self.v_reg[vx] = rng & nn;

        Ok(())
    }

    fn jmp_v0_off_nnn(&mut self, nnn: u16) -> Result<()> {
        self.pc = (self.v_reg[0] as u16) + nnn;
        Ok(())
    }

    fn set_vx_vi(&mut self, nnn: u16) -> Result<()> {
        self.i_reg = nnn;
        Ok(())
    }

    fn skip_if_vx_ne_vy(&mut self, vx: usize, vy: usize) -> Result<()> {
        if self.v_reg[vx] != self.v_reg[vy] {
            self.pc += 2
        }

        Ok(())
    }

    fn lshift_vx(&mut self, vx: usize) -> Result<()> {
        let msb = (self.v_reg[vx] >> 7) & 1;
        self.v_reg[vx] <<= 1;
        self.v_reg[0xf] = msb;

        Ok(())
    }

    fn sub_vx_from_vy(&mut self, vx: usize, vy: usize) -> Result<()> {
        let (new_vx, borrow) = self.v_reg[vy].overflowing_sub(self.v_reg[vx]);
        let new_vf = if borrow { 0 } else { 1 };

        self.v_reg[vx] = new_vx;
        self.v_reg[0xf] = new_vf;

        Ok(())
    }

    fn rshift_vx(&mut self, vx: usize) -> Result<()> {
        let lsb = self.v_reg[vx] & 1;
        self.v_reg[vx] >>= 1;
        self.v_reg[0xf] = lsb;

        Ok(())
    }

    fn sub_vy_from_vx(&mut self, vx: usize, vy: usize) -> Result<()> {
        let (new_vx, borrow) = self.v_reg[vx].overflowing_sub(self.v_reg[vy]);
        let new_vf = if borrow { 0 } else { 1 };

        self.v_reg[vx] = new_vx;
        self.v_reg[0xf] = new_vf;

        Ok(())
    }

    fn add_vy_to_vx(&mut self, vx: usize, vy: usize) -> Result<()> {
        let (new_vx, carry) = self.v_reg[vx].overflowing_add(self.v_reg[vy]);
        let new_vf = if carry { 0 } else { 1 };
        self.v_reg[vx] = new_vx;
        self.v_reg[0xf] = new_vf;

        Ok(())
    }

    fn vx_xor_eq_vy(&mut self, vx: usize, vy: usize) -> Result<()> {
        self.v_reg[vx] ^= self.v_reg[vy];

        Ok(())
    }

    fn vx_and_eq_vy(&mut self, vx: usize, vy: usize) -> Result<()> {
        self.v_reg[vx] &= self.v_reg[vy];

        Ok(())
    }

    fn vx_or_eq_vy(&mut self, vx: usize, vy: usize) -> Result<()> {
        self.v_reg[vx] |= self.v_reg[vy];

        Ok(())
    }

    fn set_vx_vy(&mut self, vx: usize, vy: usize) -> Result<()> {
        self.v_reg[vx] = self.v_reg[vy];

        Ok(())
    }

    fn add_nn_to_vx(&mut self, vx: usize, nn: u8) -> Result<()> {
        self.v_reg[vx] = self.v_reg[vx].wrapping_add(nn);

        Ok(())
    }

    fn set_vx_nn(&mut self, vx: usize, nn: u8) -> Result<()> {
        self.v_reg[vx] = nn;

        Ok(())
    }

    fn skip_if_vx_eq_vy(&mut self, vx: usize, vy: usize) -> Result<()> {
        if self.v_reg[vx] == self.v_reg[vy] {
            self.pc += 2;
        }

        Ok(())
    }

    fn skip_if_vx_ne_nn(&mut self, vx: usize, nn: u8) -> Result<()> {
        if self.v_reg[vx] != nn {
            self.pc += 2;
        }

        Ok(())
    }

    fn skip_if_vx_eq_nn(&mut self, vx: usize, nn: u8) -> Result<()> {
        if self.v_reg[vx] == nn {
            self.pc += 2;
        }

        Ok(())
    }

    /* branch & link */
    fn bl(&mut self, subroutine: u16) -> Result<()> {
        self.push(self.pc);
        self.pc = subroutine;

        Ok(())
    }

    fn jmp(&mut self, addr: u16) -> Result<()> {
        self.pc = addr;

        Ok(())
    }

    fn ret(&mut self) -> Result<()> {
        let ret_addr = self.pop();
        self.pc = ret_addr;

        Ok(())
    }

    fn clear_screen(&mut self) -> Result<()> {
        self.screen.fill(false);

        Ok(())
    }

    fn fetch(&mut self) -> Result<u16> {
        let ubyte = self.ram[self.pc as usize] as u16;
        let lbyte = self.ram[(self.pc + 1) as usize] as u16;
        self.pc += 2;

        Ok((ubyte << 8) | lbyte)
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
