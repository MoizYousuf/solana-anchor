use anchor_lang::prelude::*;

mod errors;
mod helpers;

use helpers::*;

declare_id!("3dY4fgav22wK1PvFY6BadcAqX8NrYQH4CPMRLdRhVyQV");

#[program]
pub mod new_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.key = ctx.accounts.key.key();
        counter.vault = ctx.accounts.vault.key();
        counter.vault_bump = ctx.bumps["vault"];
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &ctx.accounts.vault.key(),
            10000000,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;

        counter.count += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.vault.key(),
            &ctx.accounts.payer.key(),
            5000000,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.payer.to_account_info(),
            ],
        )?;
        counter.count -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub key: AccountInfo<'info>,

    #[account(
        init, payer = user, space = 8 + 73,
            seeds = [
                b"counter",
                key.key().as_ref()
            ],
            bump
        )]
    pub counter: Account<'info, Counter>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        seeds = [
            b"vault",
            key.key().as_ref()
        ],
        bump
    )]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>, // signer for sign user
    pub system_program: Program<'info, System>, // system program needed for creating account
}

#[derive(Accounts)]

pub struct Update<'info> {
    #[account(mut, has_one = vault)]
    pub counter: Account<'info, Counter>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        mut,
        seeds = [
            b"vault",
            counter.key.as_ref()
        ],
        bump = counter.vault_bump
    )]
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    /// System Program
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub key: Pubkey,
    pub vault: Pubkey,
    pub vault_bump: u8,
    pub count: u64,
}
