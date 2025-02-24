pub mod state;
pub mod instructions;
pub mod seeds;

use anchor_lang::prelude::*;
pub use state::*;
pub use instructions::*;
pub use seeds::*;

declare_id!("8ATesfjiC98pCAJnoQWumi6F4kBpaWd4Un44bKMnDMcQ");

#[program]
pub mod lending {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>,liquidation_threshold: u64, max_ltx: u64) -> Result<()> {
        let pool_key = ctx.accounts.pool.key().clone();
        instructions::init_pool(ctx, liquidation_threshold, max_ltx)?;
        msg!("Pool initialized {:?}", pool_key);
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
}
 