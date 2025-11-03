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

impl EscrowAccount {
    pub const INIT_SPACE: usize = 32 + 32 + 32 + 8 + 1 + 1;
}