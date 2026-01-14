use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::{constants::SEED_TREASURY_TOKEN_ACCOUNT, error::ErrorCode, state::Vault};

#[derive(Accounts)]
#[instruction(vault_name: String)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
		mut,
		associated_token::mint = mint,
        associated_token::authority = depositor,
	)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
		mut,
		seeds = [SEED_TREASURY_TOKEN_ACCOUNT, vault_name.as_bytes()],
		bump = vault_account.bump,
		token::mint = mint,
	)]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    pub vault_account: Account<'info, Vault>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn process_deposit_tokens(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
    if ctx.accounts.vault_account.is_locked {
        return Err(ErrorCode::VaultIsLocked.into());
    }

    let transfer_accounts = TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: ctx.accounts.depositor.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
    );

    transfer_checked(cpi_context, amount, ctx.accounts.mint.decimals)?;

    Ok(())
} 
