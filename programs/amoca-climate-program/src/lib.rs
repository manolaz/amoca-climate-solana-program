use anchor_lang::prelude::*;

declare_id!("3bE6S3QCr943zm5cKF5Wh1F3sCYYw1AgHRwRHZTBJTpG");

#[program]
pub mod amoca_climate_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
