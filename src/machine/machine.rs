use std::{u16, u8, usize};

use super::{instructions::parse_instruction, screen::Screen};

const RAM_SIZE: usize = 4096;
const SOFT_MIN: u16 = 0x200;
const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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
    pub screen: Screen,
}
impl Machine {
    pub fn push_to_stack(&mut self, value: u16) {
        self.stack_pointer = self.stack_pointer.overflowing_add(1).0;
        self.stack[self.stack_pointer as usize] = value;
    }

    pub fn pop_from_stack(&mut self) -> u16 {
        let value = self.stack[self.stack_pointer as usize];
        self.stack_pointer = self.stack_pointer.overflowing_sub(1).0;
        value
    }
}
impl Machine {
    pub fn exec(&mut self) {
        let instruction1 = self.read_ram(self.program_counter);
        let instruction2 = self.read_ram(self.program_counter.overflowing_add(1).0);
        let merged_instruction = ((instruction1 as u16) << 8) | (instruction2 as u16);

        parse_instruction(merged_instruction, self);
        self.increment_program_counter(2);
    }
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
        self.general_purpouse_registers[index]
    }

    pub fn write_to_general_purpouse_registers(&mut self, index: usize, value: u8) {
        self.general_purpouse_registers[index] = value;
    }
}

impl Machine {
    pub fn read_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn write_to_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }
}

impl Machine {
    pub fn read_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn write_to_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
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
        self.program_counter
    }
    pub fn write_to_program_counter(&mut self, value: u16) {
        self.program_counter = value;
    }
    pub fn increment_program_counter(&mut self, value: u16) {
        self.program_counter = self.program_counter.overflowing_add(value).0
    }
}
impl Machine {
    pub fn copy_to_ram(&mut self, data: Vec<u8>, start_address: u16) {
        // I  discovered that memory protection was not a thing for chip 8, so I will let them
        // write to the forbidden zone, but i want logs.
        // for out of bounds, the emulator should halt and display an error message
        const MIN: u16 = 0x200;
        // after validations
        if start_address < MIN {
            println!("accessing {:?} ", start_address)
        }

        for (i, line) in data.into_iter().enumerate() {
            self.ram[usize::from(start_address) + i] = line
        }
    }
    pub fn read_ram(&self, address: u16) -> u8 {
        return self.ram[usize::from(address)];
    }

    pub fn update_program_counter(&mut self, new_value: u16) {
        self.program_counter = new_value;
    }

    pub fn new() -> Self {
        let screen = Screen::new();
        let mut machine = Self {
            general_purpouse_registers: [0; 16],
            program_counter: SOFT_MIN,
            stack_pointer: 0,
            index_register: 0,
            sound_timer: 0,
            delay_timer: 0,
            ram: [0; 4096],
            stack: [0; 16],
            pressed_keys: [false; 16],
            screen,
        };

        let fontset: Vec<u8> = FONTSET.try_into().expect("convert fontset to vec");
        machine.copy_to_ram(fontset, 0x50);
        machine
    }
}
