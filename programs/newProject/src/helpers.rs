use anchor_lang::prelude::*;
use crate::errors::*;

pub fn transfer_sol(source: &AccountInfo, destination: &AccountInfo, lamports: u64) -> Result<()> {
    **destination.lamports.borrow_mut() = destination
    .lamports()
    .checked_add(lamports)
    .ok_or(CustomError::Overflow)?;

    let mut source_account_data = source.data.borrow_mut();
    **source.lamports.borrow_mut() -= lamports;
    let data_len = source_account_data.len();
    anchor_lang::solana_program::program_memory::sol_memset(*source_account_data, 0, data_len);
    Ok(())
}