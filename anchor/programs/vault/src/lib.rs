use anchor_lang::prelude::*;
use state::*;
mod state;
use error::*;
mod error;
use constants::*;
mod constants;
use instructions::*;
mod instructions;

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod vault {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>, vault_name: String) -> Result<()> {
        process_create_vault(ctx, vault_name)
    }

    pub fn deposit_tokens(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
        process_deposit_tokens(ctx, amount)
    }
}
