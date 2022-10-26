use anchor_lang::prelude::*;

mod helpers;

use helpers::*;

declare_id!("3dY4fgav22wK1PvFY6BadcAqX8NrYQH4CPMRLdRhVyQV");

#[program]
pub mod new_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.vault = ctx.accounts.vault.key();
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        transfer_sol(
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            1000000000
        )?;
        counter.count += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        transfer_sol(
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            500000000
        )?;
        counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub counter: Account<'info, Counter>,
    #[account()]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>, // signer for sign user
    pub system_program: Program<'info, System>, // system program needed for creating account
}

#[derive(Accounts)]

pub struct Update<'info> {
    #[account(mut, has_one = vault)]
    pub counter: Account<'info, Counter>,
    #[account()]
    pub vault: AccountInfo<'info>,
    pub payer: Signer<'info>,
}

#[account]
pub struct Counter {
    pub vault: Pubkey,
    pub count: u64,
}