use anchor_lang::prelude::*;

declare_id!("HJRfF8vQs4SozB9MWGETrVm4HRvjndUM65MbkQW8RsYh");

#[program]
pub mod solana_helloworld {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
