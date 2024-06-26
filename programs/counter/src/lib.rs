use anchor_lang::prelude::*;

declare_id!("3PwTwex4PiS1J1B5qrKxq77ZkBz5G5zmGhs3tfKneU32");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
