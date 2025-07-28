use std::{u16, u8};

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
    fn execute(&self,machine:&mut Machine) ->Machine {
        match self.operation {
            Operation::Op0nnnSys { address } => todo!(),
            Operation::Op00e0Cls => todo!(),
            Operation::Op00eeRet => todo!(),
            Operation::Op1nnnJmp { address } => {todo!()},
            Operation::Op2nnnCall { address } => todo!(),
            Operation::Op3xnnSe { register, value } => todo!(),
            Operation::Op4xnnSne { register, value } => todo!(),
            Operation::Op5xy0Se { register_x, register_y } => todo!(),
            Operation::Op6xnnMov { register_x, number } => todo!(),
            Operation::Op7xnnAdd { register_x, number } => todo!(),
            Operation::Op8xy0Ymovx { register_x, register_y } => todo!(),
            Operation::Op8xy1Setvx2vxorvy { register_x, register_y } => todo!(),
            Operation::Op8xy2Setvx2vxandvy { register_x, register_y } => todo!(),
            Operation::Op8xy3Setvx2vxxorvy { register_x, register_y } => todo!(),
            Operation::Op8xy4Add { register_x, register_y } => {todo!()},
            Operation::Op8xy5Sub { register_x, register_y } => {todo!()},
            Operation::Op8xy6Shr { register_x } => todo!(),
            Operation::Op8xy7Sub { register_x, register_y } => {todo!()},
            Operation::Op8xyeShl { register_x } => todo!(),
            Operation::Op9xy0Sne { register_x, register_y } => todo!(),
            Operation::OpAnnnMovI { address } => todo!(),
            Operation::OpBnnnJmpPlusV0 { value_nnn } => todo!(),
            Operation::OpCxnnMovRand { register_x, mask } => todo!(),
            Operation::OpDxynDrw { register_x, register_y, height } => todo!(),
            Operation::OpEx9eSkprs { register_x } => todo!(),
            Operation::OpExa1Sknprs { register_x } => todo!(),
            Operation::OpFx07MovDt { register_x } => todo!(),
            Operation::OpFx0aWaitKey { register_x } => todo!(),
            Operation::OpFx15SetDly { register_x } => todo!(),
            Operation::OpFx18SetSt { register_x } => todo!(),
            Operation::OpFx1eMovVi { register_x } => todo!(),
            Operation::OpFx29 { register_x } => todo!(),
            Operation::OpFx33 { register_x } => todo!(),
            Operation::OpFx55 { register_x } => todo!(),
            Operation::OpFx65 { register_x } => {todo!()}
        }
    }
}

pub fn parse_instruction(instruction: u16) -> Instruction {
    const FIRST_DIGIT_MASK: u16 = 0xF000;
    const SECOND_DIGIT_MASK: u16 = 0xFF00;
    const THIRD_DIGIT_MASK: u16 = 0xFFF0;

    if instruction == 0x00E0 {}

    let operation: Operation = match instruction & FIRST_DIGIT_MASK {
        0x0000 => {
            // match instruction & SECOND_DIGIT_MASK {}
            todo!("handle 0x0___")
        }
        0x1000 => {
            todo!("handle 0x1___")
        }
        0x2000 => {
            todo!("handle 0x2___")
        }
        0x3000 => {
            todo!("handle 0x3___")
        }
        0x4000 => {
            todo!("handle 0x4___")
        }
        0x5000 => {
            todo!("handle 0x5___")
        }
        0x6000 => {
            todo!("handle 0x6___")
        }
        0x7000 => {
            todo!("handle 0x7___")
        }
        0x8000 => {
            todo!("handle 0x8___")
        }
        0x9000 => {
            todo!("handle 0x9___")
        }
        0xA000 => {
            todo!("handle 0xA___")
        }
        0xB000 => {
            todo!("handle 0xB___")
        }
        0xC000 => {
            todo!("handle 0xC___")
        }
        0xD000 => {
            todo!("handle 0xD___")
        }
        0xE000 => {
            todo!("handle 0xE___")
        }
        0xF000 => {
            todo!("handle 0xF___")
        }
        _ => todo!(),
    };

    Instruction::new(operation, instruction)
}
