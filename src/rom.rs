use std::{path::PathBuf, io::Read};
use anyhow::Result;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct Ch8Rom {
    pub memory: Vec<u8>,
    pub size: usize
}

impl Ch8Rom {
    pub fn init(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path)?;
        let romsize = file.metadata()?.len();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(Ch8Rom { memory: buffer, size: romsize as usize })
    }
}
