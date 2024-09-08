#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    HALT,
    ADD,
    CALL,
    JUMP,
    MULT,
    READ,
    RET,
    SUB,
    WRITE,
}

impl Instruction {
    fn opcode(&self) -> u8 {
        *self as u8
    }

    fn size(&self) -> u8 {
        match &self {
            Self::HALT => 1,
            Self::ADD => 2,
            Self::CALL => 2,
            Self::JUMP => 2,
            Self::MULT => 2,
            Self::READ => 2,
            Self::RET => 1,
            Self::SUB => 2,
            Self::WRITE => 2,
        }
    }
}
