pub mod instructions;
pub mod pseudo_instructions;

pub enum NOperands {
    Zero,
    One,
    Two,
}

pub(crate) trait Sizeable {
    // fn size(&self) -> u8;
    fn n_operands(&self) -> NOperands;
}

pub type Word = u16;
