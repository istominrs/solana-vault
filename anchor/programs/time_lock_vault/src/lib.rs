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
pub mod time_lock_vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        vault_name: String,
        unlock_time: i64,
        total_amount: u64,
    ) -> Result<()> {
        process_initialize_vault(ctx, vault_name, unlock_time, total_amount)
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>, _vault_name: String) -> Result<()> {
        process_claim_tokens(ctx)
    }
}
