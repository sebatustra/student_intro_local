use solana_program::{
    pubkey::Pubkey,
    entrypoint,
    entrypoint::ProgramResult,
    account_info::AccountInfo,
    msg
};
use crate::processor;

entrypoint!(process_intruction);

pub fn process_intruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    msg!("Started program execution");

    processor::process_intruction(program_id, accounts, instruction_data)
}