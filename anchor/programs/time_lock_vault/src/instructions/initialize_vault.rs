use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::SEED_TREASURY_TOKEN_ACCOUNT, state::Vault};

#[derive(Accounts)]
#[instruction(vault_name: String)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
		init,
		payer = authority,
		space = 8 + Vault::INIT_SPACE,
		seeds = [vault_name.as_ref()],
		bump
	)]
    pub vault_account: Account<'info, Vault>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
		init,
		token::mint = mint,
		token::authority = treasury_token_account,
		payer = authority,
		seeds = [SEED_TREASURY_TOKEN_ACCOUNT, vault_name.as_bytes()],
		bump
	)]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn process_initialize_vault(
    ctx: Context<InitializeVault>,
    vault_name: String,
    unlock_time: i64,
    total_amount: u64,
) -> Result<()> {
    *ctx.accounts.vault_account = Vault {
        authority: ctx.accounts.authority.key(),
        mint: ctx.accounts.mint.key(),
        treasury_token_account: ctx.accounts.treasury_token_account.key(),
        vault_name: vault_name,
        unlock_time: unlock_time,
        total_amount: total_amount,
        treasury_bump: ctx.bumps.treasury_token_account,
        bump: ctx.bumps.vault_account,
    };

    msg!(
        "Initialized Vault Acccount:{:#?}",
        ctx.accounts.vault_account
    );
    Ok(())
}
