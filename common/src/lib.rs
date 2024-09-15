pub mod instructions;
pub mod pseudo_instructions;

#[derive(Debug, PartialEq)]
pub enum NOperands {
    Zero,
    One,
    Two,
}

pub trait Sizeable {
    // fn size(&self) -> u8;
    fn n_operands(&self) -> NOperands;
}

pub type Word = u16;
