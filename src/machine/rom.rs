use core::panic;
use std::fs;

use super::machine::Machine;

pub struct Rom {}

impl Rom {
    pub fn load(file_path: &str, machine: Machine) -> Machine {
        println!("In file {file_path}");

        let contents: Vec<u8> = fs::read(file_path).expect("should be able to read the file");
        let copy_result = machine.copy_to_ram(contents, 0x200);
        match copy_result {
            Ok(result) => result,
            Err(err) => {
                println!("{:?}", err);
                panic!();
            }
        }
    }
}
