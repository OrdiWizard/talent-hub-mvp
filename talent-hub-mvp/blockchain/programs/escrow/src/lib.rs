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
        escrow.completed = false;
        Ok(())
    }

    pub fn complete_escrow(ctx: Context<CompleteEscrow>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        require!(!escrow.completed, EscrowError::AlreadyCompleted);

        **ctx.accounts.freelancer.to_account_info().try_borrow_mut_lamports()? += escrow.amount;
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? -= escrow.amount;

        escrow.completed = true;
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

#[derive(Accounts)]
pub struct CompleteEscrow<'info> {
    #[account(mut, has_one = employer, has_one = freelancer)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub employer: Signer<'info>,
    #[account(mut)]
    pub freelancer: SystemAccount<'info>,
}

#[account]
pub struct Escrow {
    pub employer: Pubkey,
    pub freelancer: Pubkey,
    pub amount: u64,
    pub completed: bool,
}

#[error_code]
pub enum EscrowError {
    #[msg("Escrow has already been completed.")]
    AlreadyCompleted,
}
