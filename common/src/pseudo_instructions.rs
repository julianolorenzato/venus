use super::{NOperands, Sizeable};

pub enum PseudoInstruction {
    Start,
    Intdef,
    Intuse,
    End,
    Const,
    Space,
}

pub fn token_to_pseudo_instr(token: &str) -> Option<PseudoInstruction> {
    match token {
        "INTUSE" => Some(PseudoInstruction::Intuse),
        "INTDEF" => Some(PseudoInstruction::Intdef),
        "CONST" => Some(PseudoInstruction::Const),
        "SPACE" => Some(PseudoInstruction::Space),
        "START" => Some(PseudoInstruction::Start),
        "END" => Some(PseudoInstruction::End),
        _ => None,
    }
}

impl Sizeable for PseudoInstruction {
    fn n_operands(&self) -> super::NOperands {
        match self {
            Self::End => NOperands::Zero,
            _ => NOperands::One,
        }
    }
}
