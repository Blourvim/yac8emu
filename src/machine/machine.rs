use std::{io::ErrorKind, u8, usize};

const RAM_SIZE: usize = 4096;
const RAM_START: usize = 0;
const SOFT_MIN: usize = 0x200;

#[derive(Clone)]
pub struct Machine {
    general_purpouse_registers: [u8; 16],
    program_counter: u16,
    stack_pointer: u8,
    index_register: u16,
    sound_timer: u8,
    delay_timer: u8,
    ram: [u8; RAM_SIZE],
    stack: [u16; 16],
}

impl Machine {
    pub fn read_general_purpouse_registers(&self, index: usize) -> u8 {
        // TODO add validation for 16 index
        self.general_purpouse_registers[index]
    }

    pub fn write_to_general_purpouse_registers(&mut self, index: usize, value: u8) {
        // TODO add validation for 16 index
        self.general_purpouse_registers[index] = value;
    }
}

impl Machine {
    pub fn read_index_register(&self) -> u16 {
        self.index_register
    }
}
impl Machine {
    pub fn copy_to_ram(
        mut self,
        data: Vec<u8>,
        start_address: u16,
    ) -> Result<Self, std::io::Error> {
        // I  discovered that memory protection was not a thing for chip 8, so I will let them
        // write to the forbidden zone, but i want logs.
        // for out of bounds, the emulator should halt and display an error message
        const MIN: u16 = 0x200;
        const MAX: u16 = 1000;
        // after validations
        if start_address < MIN {
            println!("accessing {:?} ", start_address)
        }

        // I should do better error handlingz
        if usize::from(start_address) + data.len() > usize::from(MAX) {
            eprintln!("Memory address needed to complete the operation is out of bounds");
            return Err(std::io::Error::new(
                ErrorKind::Other,
                "Invalid start index: not enough memory",
            ));
        }

        for (i, line) in data.into_iter().enumerate() {
            self.ram[usize::from(start_address) + i] = line
        }
        return Ok(self);
    }
    pub fn write_to_ram() {}
    pub fn read_ram(&self, address: u16) -> u8 {
        return self.ram[usize::from(address)];
    }

    pub fn update_program_counter(&mut self, new_value: u16) {
        self.program_counter = new_value;
    }

    pub fn new() -> Self {
        Self {
            general_purpouse_registers: [0; 16],
            program_counter: 0x200, // standard CHIP-8 program start
            stack_pointer: 0,
            index_register: 0,
            sound_timer: 0,
            delay_timer: 0,
            ram: [0; 4096],
            stack: [0; 16],
        }
    }
}
