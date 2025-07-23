use std::io::ErrorKind;

pub struct Components {
    general_purpouse_registers: [u8; 16],
    program_counter: u16,
    stack_pointer: u8,
    index_register: u16,
    sound_timer: u8,
    delay_timer: u8,
    ram: [u8; 4096],
}

impl Components {
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

        //after validations

        for (i, line) in data.into_iter().enumerate() {
            self.ram[usize::from(start_address) + i] = line
        }
        return Ok(self);
    }
}
