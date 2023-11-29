use crate::display::Display;
use crate::rom::Ch8Rom;
use crate::vm::VM;
use anyhow::Result;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::{rect::Rect, render::Canvas};

pub struct Device {
    pub vm: VM,
}

impl Device {
    pub fn new(rom: Ch8Rom) -> Result<Self> {
        Ok(Device { vm: VM::new(rom)? })
    }

    pub fn initialize(&mut self) -> Result<()> {
        self.vm.initialize()?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        // SDL2 Setup
        let sdl_context = sdl2::init().map_err(anyhow::Error::msg)?;
        let video_subsys = sdl_context.video().map_err(anyhow::Error::msg)?;
        let window = video_subsys
            .window("Chip8 Emulator", Display::WIDTH * Display::SCALE, Display::HEIGHT * Display::SCALE)
            .position_centered()
            .opengl()
            .build()?;

        let mut canvas = window.into_canvas().present_vsync().build()?;

        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().map_err(anyhow::Error::msg)?;
        'vmevloop: loop {
            for evt in event_pump.poll_iter() {
                match evt {
                    Event::Quit { .. }
                        | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'vmevloop,
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => match key {
                            Keycode::Num1 => self.vm.keyboard[0] = true,
                            Keycode::Num2 => self.vm.keyboard[1] = true,
                            Keycode::Num3 => self.vm.keyboard[2] = true,
                            Keycode::Num4 => self.vm.keyboard[3] = true,
                            Keycode::Q => self.vm.keyboard[4] = true,
                            Keycode::W => self.vm.keyboard[5] = true,
                            Keycode::E => self.vm.keyboard[6] = true,
                            Keycode::R => self.vm.keyboard[7] = true,
                            Keycode::A => self.vm.keyboard[8] = true,
                            Keycode::S => self.vm.keyboard[9] = true,
                            Keycode::D => self.vm.keyboard[10] = true,
                            Keycode::F => self.vm.keyboard[11] = true,
                            Keycode::Z => self.vm.keyboard[12] = true,
                            Keycode::X => self.vm.keyboard[13] = true,
                            Keycode::C => self.vm.keyboard[14] = true,
                            Keycode::V => self.vm.keyboard[15] = true,
                            _ => (),
                        },
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => match key {
                            Keycode::Num1 => self.vm.keyboard[0] = false,
                            Keycode::Num2 => self.vm.keyboard[1] = false,
                            Keycode::Num3 => self.vm.keyboard[2] = false,
                            Keycode::Num4 => self.vm.keyboard[3] = false,
                            Keycode::Q => self.vm.keyboard[4] = false,
                            Keycode::W => self.vm.keyboard[5] = false,
                            Keycode::E => self.vm.keyboard[6] = false,
                            Keycode::R => self.vm.keyboard[7] = false,
                            Keycode::A => self.vm.keyboard[8] = false,
                            Keycode::S => self.vm.keyboard[9] = false,
                            Keycode::D => self.vm.keyboard[10] = false,
                            Keycode::F => self.vm.keyboard[11] = false,
                            Keycode::Z => self.vm.keyboard[12] = false,
                            Keycode::X => self.vm.keyboard[13] = false,
                            Keycode::C => self.vm.keyboard[14] = false,
                            Keycode::V => self.vm.keyboard[15] = false,
                            _ => (),
                        },
                    _ => (),
                }
            }

            for _ in 0..Display::TICKS_PER_FRAME {
                self.vm.tick()?;
            }

            self.draw_screen(&mut canvas)?;
        }

        Ok(())
    }


    fn draw_screen(&self, canvas: &mut Canvas<Window>) -> Result<()> {
        for (y, row) in self.vm.vram.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * Display::SCALE;
                let y = (y as u32) * Display::SCALE;
                if col == 0 {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                }

                let _ = canvas
                    .fill_rect(Rect::new(x as i32, y as i32, Display::SCALE, Display::SCALE));
            }
        }
        canvas.present();

        Ok(())
    }
}
