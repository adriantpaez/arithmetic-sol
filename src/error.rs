use solana_program::program_error::ProgramError;
use thiserror::Error;

// Enum with all the posible errors
#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum ArithmeticErrors {
    #[error("Instruction not implemented")]
    NotImplemented,
    #[error("Invalid accounts length")]
    InvalidAccountsLen,
}

// Implement translation from ArithmeticErrors to ProgramError
impl From<ArithmeticErrors> for ProgramError {
    fn from(e: ArithmeticErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}
