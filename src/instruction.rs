use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq)]
pub enum ArithmeticInstructions {
    Set,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}
