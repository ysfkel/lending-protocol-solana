use anchor_lang::prelude::*;

use crate::User;

pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.owner = ctx.accounts.signer.key();
    user.last_updated = Clock::get()?.unix_timestamp;
    Ok(())
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        space = 8 + User::INIT_SPACE,
        payer = signer,
        seeds = [signer.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}