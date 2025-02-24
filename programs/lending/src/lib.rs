pub mod state;
pub mod instructions;

use anchor_lang::prelude::*;
pub use state::*;
pub use instructions::*;

declare_id!("8ATesfjiC98pCAJnoQWumi6F4kBpaWd4Un44bKMnDMcQ");

#[program]
pub mod lending {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>,liquidation_threshold: u64, max_ltx: u64) -> Result<()> {
        instructions::init_pool(ctx, liquidation_threshold, max_ltx)?;
        msg!("Pool initialized {:?}", ctx.accounts.pool.key());
        Ok(())
    }

    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        instructions::init_user(ctx)?;
        msg!("User initialized {:?}", ctx.accounts.user.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
