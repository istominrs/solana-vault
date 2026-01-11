use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::constants::SEED_TREASURY_TOKEN_ACCOUNT;
use crate::{error::ErrorCode, state::Vault};

#[derive(Accounts)]
#[instruction(vault_name: String)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    #[account(
		mut,
		seeds = [vault_name.as_ref()],
		bump = vault_account.bump,
		has_one = treasury_token_account,
        has_one = mint
	)]
    pub vault_account: Account<'info, Vault>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
		init_if_needed,
		payer = beneficiary,
        associated_token::mint = mint,
        associated_token::authority = beneficiary,
        associated_token::token_program = token_program
	)]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
    let vault_name = ctx.accounts.vault_account.vault_name.clone();
    let treasury_bump = ctx.accounts.vault_account.treasury_bump;

    let vault_account = &mut ctx.accounts.vault_account;
    let now = Clock::get()?.unix_timestamp;

    if now < vault_account.unlock_time {
        return Err(ErrorCode::ClaimNotAvailableYet.into());
    }

    if vault_account.total_amount == 0 {
        return Err(ErrorCode::NothingToClaim.into());
    }

    let transfer_cpi_accounts = TransferChecked {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.treasury_token_account.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let signer_seeds: &[&[&[u8]]] = &[&[
        SEED_TREASURY_TOKEN_ACCOUNT,
        vault_name.as_ref(),
        &[treasury_bump],
    ]];

    let cpi_context = CpiContext::new(cpi_program, transfer_cpi_accounts).with_signer(signer_seeds);

    let decimals = ctx.accounts.mint.decimals;
    transfer_checked(cpi_context, vault_account.total_amount, decimals)?;

    Ok(())
}
