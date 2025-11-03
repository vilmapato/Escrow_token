use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;

use instructions::*; 


declare_id!("3Y2KY8Yy2o15iwJsKnJ7jMsWW9AVxQr617tMZfXsnG4F");

#[program]
pub mod escrow_pats {
    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
        instructions::initialize_escrow(ctx, amount)
    }

    pub fn release_escrow(ctx: Context<ReleaseEscrow>) -> Result<()> {
         instructions::release_escrow(ctx)
    }
}