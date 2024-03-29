use crate::processor::Processor;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo], // Data to read/write
    instruction_data: &[u8],  // Parameter Input
) -> ProgramResult {
    Processor::process_instruction(program_id, accounts, instruction_data)
}
