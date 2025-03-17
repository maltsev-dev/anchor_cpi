use anchor_lang::prelude::*;

declare_id!("6grzSV1SpnQjt9Jf7cMtqQDUqaC7SZDXvKE5GCGrVHxZ");

#[program]
pub mod program_b {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: Progrma B");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub pda_account: Signer<'info>,
}
