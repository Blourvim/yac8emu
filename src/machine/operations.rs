use std::{io::Read, u16, usize};

use super::machine::Machine;

///
/// updates to the program counter are done with "-2" since machine increments PC by 2 after each
/// execution for instructions that set their own pc
impl Machine {
    /// Execute machine language subroutine at address NNN
    pub fn op_0nnn_sys(&mut self, address: u16) {
        self.update_program_counter(address - 2);
    }

    /// Clear the screen
    pub fn op_00e0_cls(&mut self) {
        self.screen.clear_screen();
    }

    /// Return from a subroutine
    pub fn op_00ee_ret(&mut self) {
        let return_address = self.pop_from_stack();
        self.update_program_counter(return_address - 2);
    }

    /// Jump to address NNN
    pub fn op_1nnn_jmp(&mut self, address: u16) {
        self.write_to_program_counter(address - 2);
    }

    /// Execute subroutine starting at address NNN
    pub fn op_2nnn_call(&mut self, address: u16) {
        let return_address = self.read_program_counter();
        self.push_to_stack(return_address);
        self.update_program_counter(address - 2);
    }

    /// Skip the following instruction if the value of register VX equals NN
    pub fn op_3xnn_se(&mut self, register_x: u8, value: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);
        if register_x_value == value {
            self.increment_program_counter(2);
        }
    }

    /// Skip the following instruction if the value of register VX is not equal to NN
    pub fn op_4xnn_sne(&mut self, register_x: u8, value: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);
        if register_x_value != value {
            self.increment_program_counter(2);
        }
    }

    /// Skip the following instruction if the value of register VX is equal to the value of register VY
    pub fn op_5xy0_se(&mut self, register_x: u8, register_y: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);
        let register_y_value = self.read_general_purpouse_registers(register_y as usize);
        if register_x_value != register_y_value {
            self.increment_program_counter(2);
        }
    }

    /// Store number NN in register VX
    pub fn op_6xnn_mov(&mut self, register_x: u8, value: u8) {
        self.write_to_general_purpouse_registers(register_x as usize, value);
    }

    /// Add the value NN to register VX
    pub fn op_7xnn_add(&mut self, register_x: u8, value: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);

        let result = register_x_value + value;
        self.write_to_general_purpouse_registers(register_x as usize, result);
    }

    /// Store the value of register VY in register VX
    pub fn op_8xy0_ymovx(&mut self, register_x: u8, register_y: u8) {
        let register_y_value = self.read_general_purpouse_registers(register_y as usize);
        self.write_to_general_purpouse_registers(register_x as usize, register_y_value);
    }

    /// Set VX to VX OR VY
    pub fn op_8xy1_setvx2vxorvy(&mut self, register_x: u8, register_y: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);
        let register_y_value = self.read_general_purpouse_registers(register_y as usize);
        let or_operation_result = register_y_value | register_x_value;
        self.write_to_general_purpouse_registers(register_x as usize, or_operation_result);
    }

    /// Set VX to VX AND VY
    pub fn op_8xy2_setvx2vxandvy(&mut self, register_x: u8, register_y: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);

        let register_y_value = self.read_general_purpouse_registers(register_y as usize);

        let and_operation_result = register_x_value & register_y_value;

        self.write_to_general_purpouse_registers(register_x as usize, and_operation_result);
    }

    /// Set VX to VX XOR VY
    pub fn op_8xy3_setvx2vxxorvy(&mut self, register_x: u8, register_y: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);

        let register_y_value = self.read_general_purpouse_registers(register_y as usize);

        let xor_operation_result = register_x_value ^ register_y_value;

        self.write_to_general_purpouse_registers(register_x as usize, xor_operation_result);
    }

    /// Add the value of register VY to register VX
    /// Set VF to 01 if a carry occurs
    /// Set VF to 00 if a carry does not occur
    pub fn op_8xy4_add(&mut self, register_x: u8, register_y: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);

        let register_y_value = self.read_general_purpouse_registers(register_y as usize);

        let result = register_y_value.overflowing_add(register_x_value);

        self.write_to_general_purpouse_registers(register_x as usize, result.0);

        if result.1 {
            self.write_to_general_purpouse_registers(0xF, 0x01);
        } else {
            self.write_to_general_purpouse_registers(0xF, 0x00);
        }
    }

    /// Subtract the value of register VY from register VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    pub fn op_8xy5_sub(&mut self, register_x: u8, register_y: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);

        let register_y_value = self.read_general_purpouse_registers(register_y as usize);

        let result = register_y_value.overflowing_sub(register_x_value);

        self.write_to_general_purpouse_registers(register_x as usize, result.0);

        if result.1 {
            self.write_to_general_purpouse_registers(0xF, 0x00);
        } else {
            self.write_to_general_purpouse_registers(0xF, 0x01);
        }
    }

    /// Store the value of register VY shifted right one bit in register VX
    /// Set register VF to the least significant bit prior to the shift
    /// VY is unchanged
    pub fn op_8xy6_shr(&mut self, register_y: u8, register_x: u8) {
        let register_y_value = self.read_general_purpouse_registers(register_y as usize);
        let least_significant_bit = register_y_value & 1;
        self.write_to_general_purpouse_registers(0xF, least_significant_bit);
        let shifted_value = register_y_value.rotate_right(1);
        self.write_to_general_purpouse_registers(register_x as usize, shifted_value);
    }

    /// Set register VX to the value of VY minus VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    pub fn op_8xy7_sub(&mut self, register_x: u8, register_y: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);
        let register_y_value = self.read_general_purpouse_registers(register_y as usize);

        let sub_result = register_y_value.overflowing_sub(register_x_value);

        self.write_to_general_purpouse_registers(register_x as usize, register_x_value);

        if sub_result.1 == true {
            // means a borrow has occured
            self.write_to_general_purpouse_registers(0xF, 00);
        } else {
            self.write_to_general_purpouse_registers(0xF, 01);
        }
    }

    /// Store the value of register VY shifted left one bit in register VX
    /// Set register VF to the most significant bit prior to the shift
    /// VY is unchanged
    pub fn op_8xye_shl(&mut self, register_x: u8, register_y: u8) {
        let register_y_value = self.read_general_purpouse_registers(register_y as usize);

        let most_significant_bit = (register_y_value >> 7) & 1;
        let shifted_value = register_y_value.rotate_left(1);
        self.write_to_general_purpouse_registers(register_x as usize, shifted_value);
        self.write_to_general_purpouse_registers(0xF, most_significant_bit);
    }

    /// Skip the following instruction if the value of register VX is not equal to the value of register VY
    pub fn op_9xy0_sne(&mut self, register_x: u8, register_y: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);
        let register_y_value = self.read_general_purpouse_registers(register_y as usize);
        if register_x_value != register_y_value {
            self.increment_program_counter(2);
        }
    }

    /// Store memory address NNN in register I
    pub fn op_annn_movi(&mut self, address: u16) {
        self.write_to_index_register(address);
    }

    /// Jump to address NNN + V0
    pub fn op_bnnn_jmp_plus_v0(&mut self, value_nnn: u16) {
        let register_0_value = self.read_general_purpouse_registers(0);
        self.update_program_counter(value_nnn + register_0_value as u16 - 2);
    }

    /// Set VX to a random number with a mask of NN
    pub fn op_cxnn_mov_rand(&mut self, register_x: u8, mask: u8) {
        //TODO: would be fun to add a more deterministic option here
        let mut buf = [0; 16];
        std::fs::File::open("/dev/urandom")
            .unwrap()
            .read_exact(&mut buf)
            .unwrap();
        let masked_number = buf[0] & mask;
        self.write_to_general_purpouse_registers(register_x as usize, masked_number);
    }

    /// Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
    /// Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
    pub fn op_dxyn_drw(&mut self, register_x: u8, register_y: u8, n_bytes: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);
        let register_y_value = self.read_general_purpouse_registers(register_y as usize);
        let index_register_value = self.read_index_register();

        let values: Vec<u8> = (0..=n_bytes)
            .map(|f| self.read_ram(index_register_value + f as u16))
            .collect();

        let update_screen_state =
            self.screen
                .update_screen_state(register_x_value, register_y_value, values);
        match update_screen_state {
            true => self.write_to_general_purpouse_registers(0xF, 0x01),
            false => self.write_to_general_purpouse_registers(0xF, 0x00),
        }

        self.screen.draw();
    }

    /// Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed
    pub fn op_ex9e_skprs(&mut self, register_x: u8) {
        let pressed_key = self.read_general_purpouse_registers(register_x as usize);
        const MAX_KEYS: u8 = 16;
        if pressed_key > MAX_KEYS {
            // do nothing, unknown key is being checked
        } else {
            let pressed_keys = self.read_pressed_keys();
            if pressed_keys[pressed_key as usize] == true {
                self.increment_program_counter(2);
            }
        }
    }

    /// Skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed
    pub fn op_exa1_sknprs(&mut self, register_x: u8) {
        let pressed_key = self.read_general_purpouse_registers(register_x as usize);
        const MAX_KEYS: u8 = 16;
        if pressed_key > MAX_KEYS {
            // do nothing, unknown key is being checked
        } else {
            let pressed_keys = self.read_pressed_keys();
            if pressed_keys[pressed_key as usize] == false {
                self.increment_program_counter(2);
            }
        }
    }
    /// Store the current value of the delay timer in register VX
    pub fn op_fx07_mov_dt(&mut self, register_x: u8) {
        let delay_timer_value = self.read_delay_timer();
        self.write_to_general_purpouse_registers(register_x as usize, delay_timer_value);
    }

    /// Wait for a keypress and store the result in register VX
    pub fn op_fx0a_wait_key(&mut self, register_x: u8) {
        let keys = self.read_pressed_keys();
        let mut pressed = false;

        for i in 0..keys.len() {
            if keys[i] {
                self.write_to_general_purpouse_registers(register_x as usize, i as u8);
                pressed = true;
                break;
            }
        }

        if !pressed {
            // Redo opcode next cycle
            let program_counter = self.read_program_counter();
            self.write_to_program_counter(program_counter - 2);
        }
    }

    /// Set the delay timer to the value of register VX
    pub fn op_fx15_set_dly(&mut self, register_x: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);
        self.write_to_delay_timer(register_x_value);
    }

    /// Set the sound timer to the value of register VX
    pub fn op_fx18_set_st(&mut self, register_x: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);
        self.write_to_sound_timer(register_x_value);
    }

    /// Add the value stored in register VX to register I
    pub fn op_fx1e_mov_vi(&mut self, register_x: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);
        let register_i_value = self.read_index_register();
        self.write_to_index_register(register_i_value + register_x_value as u16);
    }

    /// Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
    pub fn op_fx29(&mut self, register_x: u8) {
        let register_x_value = self.read_general_purpouse_registers(register_x as usize);

        //invalid default sprite, only 16 are supported, just send 0 for now,
        // TODO:: make an invalid box to display
        if register_x_value > 0xF {
            self.write_to_index_register(0x50);
        }
        self.write_to_index_register(0x50 + (register_x_value as u16 * 5));
    }

    /// Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I + 1, and I + 2
    pub fn op_fx33(&mut self, register_x: u8) {
        let register_x_value: u8 = self.read_general_purpouse_registers(register_x as usize);
        let hundred = register_x_value / 100;
        let ten = (register_x_value - (hundred * 100)) / 10;
        let one = register_x_value - (hundred * 100) - (ten * 10);
        let data = vec![hundred, ten, one];
        let index_register_value = self.read_index_register();
        self.copy_to_ram(data, index_register_value);
    }

    /// Store the values of registers V0 to VX inclusive in memory starting at address I
    /// I is set to I + X + 1 after operation²
    pub fn op_fx55(&mut self, register_x: u8) {
        let values: Vec<u8> = (0..=register_x)
            .map(|register_address| self.read_general_purpouse_registers(register_address as usize))
            .collect();

        let index_register_value = self.read_index_register();

        self.copy_to_ram(values, index_register_value);

        // this should be just the x value in the instruction, not the value of x register
        // overflow may happen here, not sure if I need any involvement here but TODO: investigate
        let new_index_register_value = index_register_value + register_x as u16 + 1;
        self.write_to_index_register(new_index_register_value);
    }

    /// Fill registers V0 to VX inclusive with the values stored in memory starting at address I
    /// I is set to I + X + 1 after operation²
    pub fn op_fx65(&mut self, register_x: u8) {
        let index_register_value = self.read_index_register();

        let values: Vec<u8> = (0..=register_x)
            .map(|i| self.read_ram(index_register_value + i as u16))
            .collect();

        values.into_iter().enumerate().for_each(|f| {
            self.write_to_general_purpouse_registers(f.0, f.1);
        });
        let new_index_register_value = index_register_value + register_x as u16 + 1;
        self.write_to_index_register(new_index_register_value);
    }
}
