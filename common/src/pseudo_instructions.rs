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
    match token.to_uppercase().as_str() {
        "START" => Some(PseudoInstruction::Start),
        "INTDEF" => Some(PseudoInstruction::Intdef),
        "INTUSE" => Some(PseudoInstruction::Intuse),
        "END" => Some(PseudoInstruction::End),
        "CONST" => Some(PseudoInstruction::Const),
        "SPACE" => Some(PseudoInstruction::Space),
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
