// SPDX-License-Identifier: MIT
use anchor_lang::prelude::*;

declare_id!("ВСТАВ_СВІЙ_PROGRAM_ID");

#[program]
pub mod escrow {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.employer = *ctx.accounts.employer.key;
        escrow.freelancer = *ctx.accounts.freelancer.key;
        escrow.amount = amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(init, payer = employer, space = 128)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub employer: Signer<'info>,
    pub freelancer: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Escrow {
    pub employer: Pubkey,
    pub freelancer: Pubkey,
    pub amount: u64,
}
