use anchor_lang::prelude::*;

declare_id!("3dY4fgav22wK1PvFY6BadcAqX8NrYQH4CPMRLdRhVyQV");

#[program]
pub mod new_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.data = data;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub user_account: Account<'info, UserData>,
    #[account(mut)]
    pub user: Signer<'info>, // signer for sign user 
    pub system_program: Program<'info, System>, // system program needed for creating account
}

#[derive(Accounts)]

pub struct Update<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserData>,
}

#[account]
pub struct UserData {
    pub data: u64,
}
