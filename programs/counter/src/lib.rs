use anchor_lang::prelude::*;

declare_id!("3PwTwex4PiS1J1B5qrKxq77ZkBz5G5zmGhs3tfKneU32");

#[program]
mod counter{
    use super::*;

pub fn create_counter(ctx: Context<CreateCounter>, count: u64) -> Result<()>{
        ctx.accounts.counter.count = count;
        ctx.accounts.counter.authority = ctx.accounts.authority.key();
        msg!("creando un contador con numero inicial {} ", count);
        Ok(())  
    }

pub fn delete_counter(_ctx: Context<DeleteCounter>) -> Result<()>{
    msg!("Contador eliminado");
    Ok(())
}

}

#[derive(Accounts)]
#[instruction(count: u64)]
pub struct CreateCounter<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 32 )] // 8 bytes para el discriminador + los que ocupe la estructura
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteCounter<'info>{
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = counter.authority == counter.key(),
        close = authority
    )]
    pub counter:Account<'info, Counter> 
}

#[account]
pub struct Counter {
    count: u64, // este numero ocupa 8 bytes
    authority: Pubkey, // 32 bytes de espacio
}