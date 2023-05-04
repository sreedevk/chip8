use crate::core::{self, VM};
use anyhow::{anyhow, Result};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::env;
use std::fs::File;
use std::io::Read;

const SCALE: u32 = 20;
const WINDOW_WIDTH: u32 = (core::DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (core::DISPLAY_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 10;

pub fn init() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/rom");
        return Err(anyhow!("invalid args"));
    }

    // SDL2 Setup
    let sdl_context = sdl2::init().map_err(anyhow::Error::msg)?;
    let video_subsys = sdl_context.video().map_err(anyhow::Error::msg)?;
    let window = video_subsys
        .window("Chip8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().present_vsync().build()?;

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().map_err(anyhow::Error::msg)?;
    let mut chip8 = VM::new();
    let mut rom = File::open(&args[1]).expect("Unable to read provided ROM {&args[1]}");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)?;
    chip8.load_rom(&buffer)?;

    'vmevloop: loop {
        dbg!(&chip8);
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'vmevloop,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = update_keypress(key) {
                        chip8.presskey(k, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = update_keypress(key) {
                        chip8.presskey(k, false);
                    }
                }
                _ => (),
            }

        }

        for _ in 0..TICKS_PER_FRAME {
            chip8.tick()?;
        }

        chip8.update_timers();
        draw_screen(&chip8, &mut canvas)?;
    }

    Ok(())
}

fn update_keypress(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

fn draw_screen(vm: &VM, canvas: &mut Canvas<Window>) -> Result<()> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buffer = vm.display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for (i, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            let x = (i % core::DISPLAY_WIDTH) as u32;
            let y = (i % core::DISPLAY_HEIGHT) as u32;

            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).map_err(anyhow::Error::msg)?;
        }
    }

    canvas.present();

    Ok(())
}
