use super::{NOperands, Sizeable, Word};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction {
    Halt,
    Add,
    Call,
    Jump,
    Mult,
    Read,
    Ret,
    Sub,
    Write,
    Copy,
}

pub fn token_to_instr(token: &str) -> Option<Instruction> {
    match token.to_uppercase().as_str() {
        "ADD" => Some(Instruction::Add),
        "CALL" => Some(Instruction::Call),
        "COPY" => Some(Instruction::Copy),
        "HALT" => Some(Instruction::Halt),
        "JUMP" => Some(Instruction::Jump),
        "MULT" => Some(Instruction::Mult),
        "READ" => Some(Instruction::Read),
        "RET" => Some(Instruction::Ret),
        "SUB" => Some(Instruction::Sub),
        "WRITE" => Some(Instruction::Write),
        _ => None,
    }
}

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
            Self::Halt => NOperands::Zero,
            Self::Add => NOperands::One,
            Self::Call => NOperands::One,
            Self::Jump => NOperands::One,
            Self::Mult => NOperands::One,
            Self::Read => NOperands::One,
            Self::Ret => NOperands::Zero,
            Self::Sub => NOperands::One,
            Self::Write => NOperands::One,
            Self::Copy => NOperands::Two,
        }
    }
}
