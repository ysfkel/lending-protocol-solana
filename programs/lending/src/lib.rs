pub mod error;
pub mod state;
pub mod instructions;
pub mod seeds;


use anchor_lang::prelude::*;
pub use error::*;
pub use state::*;
pub use instructions::*;
pub use seeds::*;

declare_id!("8ATesfjiC98pCAJnoQWumi6F4kBpaWd4Un44bKMnDMcQ");

#[program]
pub mod lending {
    use super::*;

    pub fn init_reserve(ctx: Context<InitReserve>,liquidation_threshold: u64, max_ltx: u64) -> Result<()> {
        let reserve_key = ctx.accounts.reserve.key().clone();
        instructions::init_reserve(ctx, liquidation_threshold, max_ltx)?;
        msg!("Reserve initialized {:?}", reserve_key);
        Ok(())
    }

    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        let user = ctx.accounts.user.key().clone();
        instructions::init_user(ctx)?;
        msg!("User initialized {:?}", user);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw> , amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)?;
        Ok(())
    }
}
 