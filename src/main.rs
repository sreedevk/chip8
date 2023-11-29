mod fonts;
mod rom;
mod vm;
mod display;
mod device;

use anyhow::Result;
use rom::Ch8Rom;
use std::{env, path::PathBuf};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let rom = Ch8Rom::init(PathBuf::from(args[1].clone()))?;
    let mut device = device::Device::new(rom)?;

    device.initialize()?;
    device.run()?;

    Ok(())
}
