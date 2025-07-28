use std::{u16, u8, usize};

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
    pressed_keys: [bool; 16],
}
impl Machine {
    pub fn read_pressed_keys(&self) -> [bool; 16] {
        self.pressed_keys
    }

    // would trigger on key press down
    pub fn set_pressed_keys(&mut self, pressed_key: u16) {
        self.pressed_keys[pressed_key as usize] = true;
    }

    // would trigger on key release
    pub fn unset_pressed_keys(&mut self, released_key: u16) {
        self.pressed_keys[released_key as usize] = false;
    }
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
    pub fn read_delay_timer(&self) -> u8 {
        // TODO add validation for 16 index
        self.delay_timer
    }

    pub fn write_to_delay_timer(&mut self, value: u8) {
        // TODO add validation for 16 index
        self.delay_timer = value;
    }
}

impl Machine {
    pub fn read_sound_timer(&self) -> u8 {
        // TODO add validation for 16 index
        self.delay_timer
    }

    pub fn write_to_sound_timer(&mut self, value: u8) {
        // TODO add validation for 16 index
        self.delay_timer = value;
    }
}

impl Machine {
    pub fn read_index_register(&self) -> u16 {
        self.index_register
    }

    pub fn write_to_index_register(&mut self, value: u16) {
        self.index_register = value;
    }
}

impl Machine {
    pub fn read_program_counter(&self) -> u16 {
        self.index_register
    }
    pub fn write_to_program_counter(&mut self, value: u16) {
        self.index_register = value;
    }
    pub fn increment_program_counter(&mut self, value: u16) {
        self.index_register = self.index_register + value
    }
}
impl Machine {
    pub fn copy_to_ram(&mut self, data: Vec<u8>, start_address: u16) {
        // I  discovered that memory protection was not a thing for chip 8, so I will let them
        // write to the forbidden zone, but i want logs.
        // for out of bounds, the emulator should halt and display an error message
        const MIN: u16 = 0x200;
        const MAX: u16 = 1000;
        // after validations
        if start_address < MIN {
            println!("accessing {:?} ", start_address)
        }

        for (i, line) in data.into_iter().enumerate() {
            self.ram[usize::from(start_address) + i] = line
        }
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
            pressed_keys: [false; 16],
        }
    }
}
