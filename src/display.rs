use crate::vm::Byte;

pub struct Display {
    pub vram: Vec<Vec<Byte>>,
}

impl Display {
    pub const WIDTH: u32 = 64;
    pub const HEIGHT: u32 = 32;
    pub const TICKS_PER_FRAME: usize = 10;
    pub const SCALE: u32 = 20;
}
