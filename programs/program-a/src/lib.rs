use anchor_lang::prelude::*;
use program_b::program::ProgramB;

declare_id!("G8aR4QssEGg5xXaj1Ha33kJCbEw7dsAYn6pHxQxUdwmd");

#[program]
pub mod program_a {
    use anchor_lang::solana_program::{
        program::{invoke, invoke_signed},
        system_instruction,
    };

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: Program A");

        // Get Accounts from Initialize Context
        let pda_address = ctx.accounts.pda_account.key();
        let singer_address = ctx.accounts.signer.key();

        // Get bump from Initialize Context
        let bump = ctx.bumps.pda_account;

        // Create instructions for transfering (from, to, amount)
        let instruction =
            &system_instruction::transfer(&pda_address, &singer_address, 1_000_000_000);

        // Create Accounts array [Sender, Receiver, Program]
        let account_infos = [
            ctx.accounts.pda_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];

        // Create Signer Seeds with bump
        let signer_seeds: &[&[&[u8]]] = &[&[b"something", singer_address.as_ref(), &[bump]]];

        // invoke System Program with PDA sign
        invoke_signed(instruction, &account_infos, signer_seeds)?;

        // Create CPI context with signer to program_b
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.program_b.to_account_info(),
            program_b::cpi::accounts::Initialize{
                pda_account:ctx.accounts.pda_account.to_account_info()
            },
            signer_seeds,
        );

        // Use CPI Context for initialize function call
        program_b::cpi::initialize(cpi_context)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: aaa
    // create pda_account with attributes:
    #[account(
        mut,
        seeds =[b"something",signer.key().as_ref()],
        bump,
    )]
    pub pda_account: AccountInfo<'info>,

    // transfer updates signer account
    #[account(mut)]
    pub signer: Signer<'info>,

    // System Program variable
    pub system_program: Program<'info, System>,

    // progrma-b variable
    pub program_b: Program<'info, ProgramB>,
}
