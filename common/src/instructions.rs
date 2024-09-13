use super::{NOperands, Sizeable, Word};

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
    COPY,
}

// const MAPPINGS: &[(Instruction, Word)] = &[
//     (Instruction::HALT, 28),
//     (Instruction::ADD, 28),
//     (Instruction::HALT, 28),
//     (Instruction::HALT, 28),
//     (Instruction::HALT, 28),
//     (Instruction::HALT, 28),
//     (Instruction::HALT, 28),
//     (Instruction::HALT, 28),
//     (Instruction::HALT, 28),
// ];

// pub fn instr_by_opcode(opcode: Word) -> Instruction {
//     opcode as Instruction
// }

impl Instruction {
    fn opcode(&self) -> u8 {
        *self as u8
    }
}

impl Sizeable for Instruction {
    // fn size(&self) -> u8 {
    //     match &self {
    //         Self::HALT => 1,
    //         Self::ADD => 2,
    //         Self::CALL => 2,
    //         Self::JUMP => 2,
    //         Self::MULT => 2,
    //         Self::READ => 2,
    //         Self::RET => 1,
    //         Self::SUB => 2,
    //         Self::WRITE => 2,
    //         Self::COPY => 3,
    //     }
    // }

    fn n_operands(&self) -> super::NOperands {
        match &self {
            Self::HALT => NOperands::Zero,
            Self::ADD => NOperands::One,
            Self::CALL => NOperands::One,
            Self::JUMP => NOperands::One,
            Self::MULT => NOperands::One,
            Self::READ => NOperands::One,
            Self::RET => NOperands::Zero,
            Self::SUB => NOperands::One,
            Self::WRITE => NOperands::One,
            Self::COPY => NOperands::Two,
        }
    }
}
