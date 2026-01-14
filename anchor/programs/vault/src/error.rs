use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Vault is locked at the moment.")]
    VaultIsLocked,
}
