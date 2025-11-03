use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022, TokenAccount, Mint, TransferChecked, transfer_checked};
use crate::state::EscrowAccount;

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,

    #[account(mut)]
    pub recipient_ata: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"escrow", escrow.initializer.as_ref()],
        bump = escrow.bump,
        has_one = recipient
    )]
    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
}

pub fn release_escrow(ctx: Context<ReleaseEscrow>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    require!(!escrow.is_released, EscrowError::AlreadyReleased);

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.recipient_ata.to_account_info(),
        authority: ctx.accounts.recipient.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_ctx, escrow.amount, 6)?;

    escrow.is_released = true;
    Ok(())
}

#[error_code]
pub enum EscrowError {
    #[msg("Escrow already released.")]
    AlreadyReleased,
}