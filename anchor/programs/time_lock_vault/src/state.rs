use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Vault {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub treasury_token_account: Pubkey,
    pub unlock_time: i64,
    #[max_len(50)]
    pub vault_name: String,
    pub total_amount: u64,
    pub treasury_bump: u8,
    pub bump: u8,
}
