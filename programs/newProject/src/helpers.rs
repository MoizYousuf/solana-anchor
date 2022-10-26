use anchor_lang::prelude::*;

pub fn transfer_sol(from: AccountInfo, to: AccountInfo, lamports: u64) -> Result<()> {
    **from.try_borrow_mut_lamports()? -= lamports;
    **to.try_borrow_mut_lamports()? += lamports;
    Ok(())
}