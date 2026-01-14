use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    #[max_len(50)]
    pub vault_name: String,
    pub is_locked: bool,
    pub unlock_timestamp: i64,
    pub bump: u8,
}
