// pipeline
// Read the file
// split into instructions
// each instruction then needs to be evaluated
//
//
//
//
//
//
pub enum Operation {
    ClearScreen,
    Return,
    Jump { address: u16 },
    Call { address: u16 },
    SkipEqualByte { register: u8, value: u8 },
    SkipNotEqualByte { register: u8, value: u8 },
    SkipEqualReg { x: u8, y: u8 },
    LoadByte { register: u8, value: u8 },
    AddByte { register: u8, value: u8 },
    LoadReg { x: u8, y: u8 },
    Or { x: u8, y: u8 },
    And { x: u8, y: u8 },
    Xor { x: u8, y: u8 },
    AddReg { x: u8, y: u8 },
    SubReg { x: u8, y: u8 },
    ShiftRight { x: u8, y: u8 },
    SubN { x: u8, y: u8 },
    ShiftLeft { x: u8, y: u8 },
    SkipNotEqualReg { x: u8, y: u8 },
    LoadI { address: u16 },
    JumpV0 { address: u16 },
    Rand { register: u8, mask: u8 },
    Draw { x: u8, y: u8, height: u8 },
    SkipKeyPressed { register: u8 },
    SkipKeyNotPressed { register: u8 },
    LoadDelayTimer { register: u8 },
    WaitKey { register: u8 },
    SetDelayTimer { register: u8 },
    SetSoundTimer { register: u8 },
    AddI { register: u8 },
    LoadSprite { register: u8 },
    Bcd { register: u8 },
    StoreRegisters { x: u8 },
    LoadRegisters { x: u8 },
    Unknown,
}

pub struct Instruction {
    opcode: Operation,
    operand: u16, // maybe write an u12 module, I don't want to use the crate but whatever, should
                  // be fine for now
}

impl Instruction {
    pub fn new(opcode: Operation, operand: u16) -> Self {
        Self { opcode, operand }
    }
}
pub fn parse_instruction(instruction: u16) -> Instruction {
    // let op_code:String = input
    let operation: Operation = match instruction {
        1 => Operation::Unknown,
        _ => Operation::Unknown,
    };

    Instruction::new(operation, instruction)
}
