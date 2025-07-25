use std::{io::Read, u16, usize}; // 0.8.5

use super::machine::Machine;

impl Machine {
    /// Execute machine language subroutine at address NNN
    pub fn op_0nnn_sys(&mut self, address: u16) {}

    /// Clear the screen
    pub fn op_00e0_cls(&mut self) {}

    /// Return from a subroutine
    pub fn op_00ee_ret(&mut self) {}

    /// Jump to address NNN
    pub fn op_1nnn_jmp(&mut self, address: u16) {}

    /// Execute subroutine starting at address NNN
    pub fn op_2nnn_call(&mut self, address: u16) {}

    /// Skip the following instruction if the value of register VX equals NN
    pub fn op_3xnn_se(&mut self, register: u8, value: u8) {}

    /// Skip the following instruction if the value of register VX is not equal to NN
    pub fn op_4xnn_sne(&mut self, register: u8, value: u8) {}

    /// Skip the following instruction if the value of register VX is equal to the value of register VY
    pub fn op_5xy0_se(&mut self, register_x: u8, register_y: u8) {}

    /// Store number NN in register VX
    pub fn op_6xnn_mov(&mut self, register_x: u8, number: u8) {}

    /// Add the value NN to register VX
    pub fn op_7xnn_add(&mut self, register_x: u8, number: u8) {}

    /// Store the value of register VY in register VX
    pub fn op_8xy0_ymovx(&mut self, register_x: u8, register_y: u8) {}

    /// Set VX to VX OR VY
    pub fn op_8xy1_setvx2vxorvy(&mut self, register_x: u8, register_y: u8) {}

    /// Set VX to VX AND VY
    pub fn op_8xy2_setvx2vxandvy(&mut self, register_x: u8, register_y: u8) {}

    /// Set VX to VX XOR VY
    pub fn op_8xy3_setvx2vxxorvy(&mut self, register_x: u8, register_y: u8) {}

    /// Add the value of register VY to register VX
    /// Set VF to 01 if a carry occurs
    /// Set VF to 00 if a carry does not occur
    pub fn op_8xy4_add(&mut self, register_x: u8, register_y: u8) {}

    /// Subtract the value of register VY from register VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    pub fn op_8xy5_sub(&mut self, register_x: u8, register_y: u8) {}

    /// Store the value of register VY shifted right one bit in register VX
    /// Set register VF to the least significant bit prior to the shift
    /// VY is unchanged
    pub fn op_8xy6_shr(&mut self, register_x: u8) {}

    /// Set register VX to the value of VY minus VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    pub fn op_8xy7_sub(&mut self, register_x: u8, register_y: u8) {}

    /// Store the value of register VY shifted left one bit in register VX
    /// Set register VF to the most significant bit prior to the shift
    /// VY is unchanged
    pub fn op_8xye_shl(&mut self, register_x: u8) {}

    /// Skip the following instruction if the value of register VX is not equal to the value of register VY
    pub fn op_9xy0_sne(&mut self, register_x: u8, register_y: u8) {}

    /// Store memory address NNN in register I
    pub fn op_annn_movi(&mut self, address: u16) {}

    /// Jump to address NNN + V0
    pub fn op_bnnn_jmp_plus_v0(&mut self, value_nnn: u16) {
        let register_0_value = self.read_general_purpouse_registers(0);
        self.update_program_counter(value_nnn + register_0_value as u16);
    }

    /// Set VX to a random number with a mask of NN
    pub fn op_cxnn_mov_rand(&mut self, register_x: u8, mask: u8) {
        //TODO: would be fun to add a more deterministic option here
        let mut buf = [0; 16];
        let random_number = std::fs::File::open("/dev/urandom")
            .unwrap()
            .read_exact(&mut buf)
            .unwrap();
        let masked_number = buf[0] & mask;
        self.write_to_general_purpouse_registers(register_x as usize, masked_number);
    }

    /// Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
    /// Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
    pub fn op_dxyn_drw(&mut self, register_x: u8, register_y: u8, height: u8) {



    }

    /// Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed
    pub fn op_ex9e_skprs(&mut self, register_x: u8) {}

    /// Skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed
    pub fn op_exa1_sknprs(&mut self, register_x: u8) {}

    /// Store the current value of the delay timer in register VX
    pub fn op_fx07_mov_dt(&mut self, register_x: u8) {}

    /// Wait for a keypress and store the result in register VX
    pub fn op_fx0a_wait_key(&mut self, register_x: u8) {}

    /// Set the delay timer to the value of register VX
    pub fn op_fx15_set_dly(&mut self, register_x: u8) {}

    /// Set the sound timer to the value of register VX
    pub fn op_fx18_set_st(&mut self, register_x: u8) {}

    /// Add the value stored in register VX to register I
    pub fn op_fx1e_mov_vi(&mut self, register_x: u8) {}

    /// Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
    pub fn op_fx29(&mut self, register_x: u8) {}

    /// Store the binary-coded decimal equivalent of the value stored in register VX
    /// at addresses I, I + 1, and I + 2
    pub fn op_fx33(&mut self, register_x: u8) {}

    /// Store the values of registers V0 to VX inclusive in memory starting at address I
    /// I is set to I + X + 1 after operation²
    pub fn op_fx55(&mut self, register_x: u8) {}

    /// Fill registers V0 to VX inclusive with the values stored in memory starting at address I
    /// I is set to I + X + 1 after operation²
    pub fn op_fx65(&mut self, register_x: u8) {}
}
