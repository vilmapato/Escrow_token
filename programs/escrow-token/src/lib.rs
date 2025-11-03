use anchor_lang::prelude::*;

declare_id!("3Y2KY8Yy2o15iwJsKnJ7jMsWW9AVxQr617tMZfXsnG4F");

#[program]
pub mod escrow_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
