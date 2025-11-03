use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
    pub initializer: Pubkey,
    pub recipient: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub is_released: bool,
    pub bump: u8,
}