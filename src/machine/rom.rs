use std::{error::Error, fs};

use super::machine::Machine;

pub struct Rom;

impl Rom {
    pub fn load(file_path: &str, mut machine: Machine) -> Result<Machine, Box<dyn Error>> {
        println!("Loading file: {}", file_path);
        let contents = fs::read(file_path)?;
        machine.copy_to_ram(contents, 0x200);
        Ok(machine)
    }
}
