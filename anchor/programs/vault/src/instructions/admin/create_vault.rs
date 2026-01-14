use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::SEED_TREASURY_TOKEN_ACCOUNT, state::Vault};

#[derive(Accounts)]
#[instruction(vault_name: String)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        space = 8 + Vault::INIT_SPACE,
        payer = authority,
        seeds = [vault_name.as_bytes()],
        bump
    )]
    pub vault_account: Account<'info, Vault>,

    #[account(
        init,
        token::mint = mint,
        token::authority = vault_account,
        payer = authority,
        seeds = [SEED_TREASURY_TOKEN_ACCOUNT, vault_name.as_bytes()],
        bump
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn process_create_vault(ctx: Context<CreateVault>, vault_name: String) -> Result<()> {
    ctx.accounts.vault_account.set_inner(Vault {
        authority: ctx.accounts.authority.key(),
        mint: ctx.accounts.mint.key(),
        token_account: ctx.accounts.token_account.key(),
        vault_name: vault_name,
        is_locked: false,
        unlock_timestamp: 0,
        bump: ctx.bumps.token_account,
    });

    Ok(())
}
