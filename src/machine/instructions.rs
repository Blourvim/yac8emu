use std::{str::Matches, u16, u8};

use super::machine::{self, Machine};

#[rustfmt::skip]
pub enum Operation {

    ///	Execute machine language subroutine at address NNN
    Op0nnnSys {address: u16}, 
///	Clear the screen
    Op00e0Cls, 
///	Return from a subroutine
    Op00eeRet, 
///	Jump to address NNN
    Op1nnnJmp { address: u16 }, 
///	Execute subroutine starting at address NNN
    Op2nnnCall { address: u16 }, 
///	Skip the following instruction if the value of register VX equals NN
    Op3xnnSe { register: u8, value: u8 }, 
///	Skip the following instruction if the value of register VX is not equal to NN
    Op4xnnSne { register: u8, value: u8 }, 
///	Skip the following instruction if the value of register VX is equal to the value of register VY
    Op5xy0Se { register_x: u8, register_y: u8 }, 
///	Store number NN in register VX
    Op6xnnMov { register_x: u8, number: u8 }, 
///	Add the value NN to register VX
    Op7xnnAdd { register_x: u8, number: u8 }, 
///	Store the value of register VY in register VX
    Op8xy0Ymovx { register_x: u8, register_y: u8 }, 
///	Set VX to VX OR VY
    Op8xy1Setvx2vxorvy { register_x: u8, register_y: u8 }, 
///	Set VX to VX AND VY
    Op8xy2Setvx2vxandvy { register_x: u8, register_y: u8 }, 
///	Set VX to VX XOR VY
    Op8xy3Setvx2vxxorvy { register_x: u8, register_y: u8 }, 
///	Add the value of register VY to register VXSet VF to 01 if a carry occursSet VF to 00 if a carry does not occur
    Op8xy4Add { register_x: u8, register_y: u8 }, 
///	Subtract the value of register VY from register VXSet VF to 00 if a borrow occursSet VF to 01 if a borrow does not occur
    Op8xy5Sub { register_x: u8, register_y: u8 }, 
///	Store the value of register VY shifted right one bit in register VX¹Set register VF to the least significant bit prior to the shiftVY is unchanged
    Op8xy6Shr { register_x: u8 }, 
///	Set register VX to the value of VY minus VXSet VF to 00 if a borrow occursSet VF to 01 if a borrow does not occur
    Op8xy7Sub { register_x: u8, register_y: u8 }, 
///	Store the value of register VY shifted left one bit in register VX¹Set register VF to the most significant bit prior to the shiftVY is unchanged
    Op8xyeShl { register_x: u8 }, 
///	Skip the following instruction if the value of register VX is not equal to the value of register VY
    Op9xy0Sne { register_x: u8, register_y: u8 }, 
///	Store memory address NNN in register I
    OpAnnnMovI { address: u16 }, 
///	Jump to address NNN + V0
    OpBnnnJmpPlusV0 { value_nnn: u16 }, 
///	Set VX to a random number with a mask of NN
    OpCxnnMovRand { register_x: u8, mask: u8 }, 
///	Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in ISet VF to 01 if any set pixels are changed to unset, and 00 otherwise
    OpDxynDrw { register_x: u8, register_y: u8, height: u8 }, 
///	Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed
    OpEx9eSkprs { register_x: u8 }, 
///	Skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed
    OpExa1Sknprs { register_x: u8 }, 
///	Store the current value of the delay timer in register VX
    OpFx07MovDt { register_x: u8 }, 
///	Wait for a keypress and store the result in register VX
    OpFx0aWaitKey { register_x: u8 }, 
///	Set the delay timer to the value of register VX
    OpFx15SetDly { register_x: u8 }, 
///	Set the sound timer to the value of register VX
    OpFx18SetSt { register_x: u8 }, 
///	Add the value stored in register VX to register I
    OpFx1eMovVi { register_x: u8 }, 
///	Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
    OpFx29 { register_x: u8 }, 
///	Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I + 1, and I + 2
    OpFx33 { register_x: u8 }, 
///	Store the values of registers V0 to VX inclusive in memory starting at address II is set to I + X + 1 after operation²
    OpFx55 { register_x: u8 }, 
///	Fill registers V0 to VX inclusive with the values stored in memory starting at address II is set to I + X + 1 after operation²
    OpFx65 { register_x: u8 }, 
}

pub struct Instruction {
    operation: Operation,
}

impl Instruction {
    pub fn new(opcode: Operation, operand: u16) -> Self {
        Self { operation: opcode }
    }
}

#[rustfmt::skip]
impl Instruction {
    fn execute(&mut self,machine:&mut Machine){
        match self.operation {
            Operation::Op0nnnSys { address } => {machine.op_0nnn_sys(address);}
            Operation::Op00e0Cls => {machine.op_00e0_cls();}
            Operation::Op00eeRet => machine.op_00ee_ret(),
            Operation::Op1nnnJmp { address } => machine.op_1nnn_jmp(address),
            Operation::Op2nnnCall { address } => machine.op_2nnn_call(address),
            Operation::Op3xnnSe { register, value } => machine.op_3xnn_se(register, value),
            Operation::Op4xnnSne { register, value } => machine.op_4xnn_sne(register, value),
            Operation::Op5xy0Se { register_x, register_y } => machine.op_5xy0_se(register_x, register_y),
            Operation::Op6xnnMov { register_x, number } => machine.op_6xnn_mov(register_x, number),
            Operation::Op7xnnAdd { register_x, number } => machine.op_7xnn_add(register_x, number),
            Operation::Op8xy0Ymovx { register_x, register_y } => machine.op_8xy0_ymovx(register_x, register_y),
            Operation::Op8xy1Setvx2vxorvy { register_x, register_y } => machine.op_8xy1_setvx2vxorvy(register_x, register_y),
            Operation::Op8xy2Setvx2vxandvy { register_x, register_y } => machine.op_8xy2_setvx2vxandvy(register_x, register_y),
            Operation::Op8xy3Setvx2vxxorvy { register_x, register_y } => machine.op_8xy3_setvx2vxxorvy(register_x, register_y),
            Operation::Op8xy4Add { register_x, register_y } => machine.op_8xy4_add(register_x, register_y),
            Operation::Op8xy5Sub { register_x, register_y } => machine.op_8xy5_sub(register_x, register_y),
            Operation::Op8xy6Shr { register_x } => machine.op_8xy6_shr(register_x, register_x),
            Operation::Op8xy7Sub { register_x, register_y } => machine.op_8xy7_sub(register_x, register_y),
            Operation::Op8xyeShl { register_x } => machine.op_8xye_shl(register_x, register_x),
            Operation::Op9xy0Sne { register_x, register_y } => machine.op_9xy0_sne(register_x, register_y),
            Operation::OpAnnnMovI { address } => machine.op_annn_movi(address),
            Operation::OpBnnnJmpPlusV0 { value_nnn } => machine.op_bnnn_jmp_plus_v0(value_nnn),
            Operation::OpCxnnMovRand { register_x, mask } => machine.op_cxnn_mov_rand(register_x, mask),
            Operation::OpDxynDrw { register_x, register_y, height } => machine.op_dxyn_drw(register_x, register_y, height),
            Operation::OpEx9eSkprs { register_x } => machine.op_ex9e_skprs(register_x),
            Operation::OpExa1Sknprs { register_x } => machine.op_exa1_sknprs(register_x),
            Operation::OpFx07MovDt { register_x } => machine.op_fx07_mov_dt(register_x),
            Operation::OpFx0aWaitKey { register_x } => machine.op_fx0a_wait_key(register_x),
            Operation::OpFx15SetDly { register_x } => machine.op_fx15_set_dly(register_x),
            Operation::OpFx18SetSt { register_x } => machine.op_fx18_set_st(register_x),
            Operation::OpFx1eMovVi { register_x } => machine.op_fx1e_mov_vi(register_x),
            Operation::OpFx29 { register_x } => machine.op_fx29(register_x),
            Operation::OpFx33 { register_x } => machine.op_fx33(register_x),
            Operation::OpFx55 { register_x } => machine.op_fx55(register_x),
            Operation::OpFx65 { register_x } => machine.op_fx65(register_x),
        }
    }
}

#[rustfmt::skip]
pub fn parse_instruction(instruction: u16, machine: &mut Machine) {
    const FIRST_DIGIT_MASK: u16 = 0xF000;
    const SECOND_DIGIT_MASK: u16 = 0x0F00;
    const THIRD_DIGIT_MASK: u16 = 0x00F0;
    const FOURTH_DIGIT_MASK: u16 = 0x000F;

    let o_first = instruction & FIRST_DIGIT_MASK;
    let o_second = instruction & SECOND_DIGIT_MASK;
    let o_third = instruction & THIRD_DIGIT_MASK;
    let o_fourth = instruction & FOURTH_DIGIT_MASK;

    let first:u8 = o_first.try_into().unwrap();
    let second:u8 = o_second.try_into().unwrap();
    let third:u8 =  o_third.try_into().unwrap();
    let fourth:u8 = o_fourth.try_into().unwrap();

    match (first, second, third, fourth) {
 (0x0, 0x0, 0xE, 0x0) => machine.op_00e0_cls(),
    (0x0, 0x0, 0xE, 0xE) => machine.op_00ee_ret(),
    (0x1, _, _, _) => machine.op_1nnn_jmp((instruction & 0x0FFF) as u16),
    (0x2, _, _, _) => machine.op_2nnn_call((instruction & 0x0FFF) as u16),
    (0x3, x, _, _) => machine.op_3xnn_se(x, (instruction & 0x00FF) as u8),
    (0x4, x, _, _) => machine.op_4xnn_sne(x, (instruction & 0x00FF) as u8),
    (0x5, x, y, 0x0) => machine.op_5xy0_se(x, y),
    (0x6, x, _, _) => machine.op_6xnn_mov(x, (instruction & 0x00FF) as u8),
    (0x7, x, _, _) => machine.op_7xnn_add(x, (instruction & 0x00FF) as u8),
    (0x8, x, y, 0x0) => machine.op_8xy0_ymovx(x, y),
    (0x8, x, y, 0x1) => machine.op_8xy1_setvx2vxorvy(x, y),
    (0x8, x, y, 0x2) => machine.op_8xy2_setvx2vxandvy(x, y),
    (0x8, x, y, 0x3) => machine.op_8xy3_setvx2vxxorvy(x, y),
    (0x8, x, y, 0x4) => machine.op_8xy4_add(x, y),
    (0x8, x, y, 0x5) => machine.op_8xy5_sub(x, y),
    (0x8, x, y, 0x6) => machine.op_8xy6_shr(x,y),
    (0x8, x, y, 0x7) => machine.op_8xy7_sub(x, y),
    (0x8, x, y, 0xE) => machine.op_8xye_shl(x,y),
    (0x9, x, y, 0x0) => machine.op_9xy0_sne(x, y),
    (0xA, _, _, _) => machine.op_annn_movi(instruction & 0x0FFF),
    (0xB, _, _, _) => machine.op_bnnn_jmp_plus_v0(instruction & 0x0FFF),
    (0xC, x, _, _) => machine.op_cxnn_mov_rand(x, (instruction & 0x00FF) as u8),
    (0xD, x, y, n) => machine.op_dxyn_drw(x, y, n),
    (0xE, x, 0x9, 0xE) => machine.op_ex9e_skprs(x),
    (0xE, x, 0xA, 0x1) => machine.op_exa1_sknprs(x),
    (0xF, x, 0x0, 0x7) => machine.op_fx07_mov_dt(x),
    (0xF, x, 0x0, 0xA) => machine.op_fx0a_wait_key(x),
    (0xF, x, 0x1, 0x5) => machine.op_fx15_set_dly(x),
    (0xF, x, 0x1, 0x8) => machine.op_fx18_set_st(x),
    (0xF, x, 0x1, 0xE) => machine.op_fx1e_mov_vi(x),
    (0xF, x, 0x2, 0x9) => machine.op_fx29(x),
    (0xF, x, 0x3, 0x3) => machine.op_fx33(x),
    (0xF, x, 0x5, 0x5) => machine.op_fx55(x),
    (0xF, x, 0x6, 0x5) => machine.op_fx65(x),
    _ => panic!("Unknown instruction: {:04x}", instruction),
    }
}
