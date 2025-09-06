use anchor_lang::prelude::*;

declare_id!("AHaLKdGhGT1e5Knzoibz1eWyJhEUZbeombZY2Z4P5N5K");

pub mod constants;
pub mod errors;
pub mod instructions;

#[program]
pub mod escrowman {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
