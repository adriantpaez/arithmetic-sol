use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::ArithmeticErrors;
use crate::instruction::ArithmeticInstructions;
use crate::state::Term;
use crate::utils::*;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = ArithmeticInstructions::try_from_slice(&instruction_data[..1])
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let result = match instruction {
            ArithmeticInstructions::Set => process_set(_program_id, accounts, &instruction_data),
            _ => process_binary_operation(_program_id, accounts, instruction),
        };
        result
    }
}

fn process_set(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() != 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    if accounts.len() != 1 {
        return Err(ProgramError::from(ArithmeticErrors::InvalidAccountsLen));
    }

    let value: u64 = get_u64(instruction_data, 1);

    let ai = &accounts[0];
    let mut term = Term::try_from_slice(&ai.data.borrow())?;
    term.value = value;
    term.serialize(&mut *ai.data.borrow_mut())?;
    Ok(())
}

fn process_binary_operation(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: ArithmeticInstructions,
) -> ProgramResult {
    if accounts.len() != 3 {
        return Err(ProgramError::from(ArithmeticErrors::InvalidAccountsLen));
    }
    let a: u64 = (Term::try_from_slice(&accounts[0].data.borrow())?).value;
    let b: u64 = (Term::try_from_slice(&accounts[1].data.borrow())?).value;
    let mut term_c = Term::try_from_slice(&accounts[2].data.borrow())?;
    match instruction_data {
        ArithmeticInstructions::Addition => term_c.value = a + b,
        ArithmeticInstructions::Subtraction => term_c.value = a - b,
        ArithmeticInstructions::Multiplication => term_c.value = a * b,
        ArithmeticInstructions::Division => term_c.value = a / b,
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    term_c.serialize(&mut *accounts[2].data.borrow_mut())?;
    Ok(())
}
