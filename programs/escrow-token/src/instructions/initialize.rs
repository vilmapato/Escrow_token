use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022, TokenAccount, Mint, TransferChecked, transfer_checked};
use crate::state::EscrowAccount;

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    /// CHECK: recipient can be any wallet
    pub recipient: AccountInfo<'info>,

    #[account(mut)]
    pub initializer_ata: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = initializer,
        space = 8 + EscrowAccount::INIT_SPACE,
        seeds = [b"escrow", initializer.key().as_ref()],
        bump
    )]
    pub escrow_account: Account<'info, EscrowAccount>,

    #[account(
        init,
        payer = initializer,
        token::mint = mint,
        token::authority = escrow_account,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_escrow(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow_account;

    escrow.initializer = ctx.accounts.initializer.key();
    escrow.recipient = ctx.accounts.recipient.key();
    escrow.mint = ctx.accounts.mint.key();
    escrow.amount = amount;
    escrow.is_released = false;
    escrow.bump = *ctx.bumps.get("escrow_account").unwrap();

    // Transfer tokens from initializer → escrow PDA ATA
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.initializer_ata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.escrow_token_account.to_account_info(),
        authority: ctx.accounts.initializer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_ctx, amount, 6)?; // 6 decimals for PATS

    Ok(())
}