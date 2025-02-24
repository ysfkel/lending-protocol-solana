use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self,Mint, TokenAccount, TokenInterface};

use crate::Pool;

pub fn init_pool(ctx: Context<InitPool>, liquidation_threshold: u64, max_ltx: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.mint = ctx.accounts.mint.key();
    pool.authority = ctx.accounts.signer.key();
    pool.liquidation_threshold = liquidation_threshold;
    pool.max_ltv = max_ltx;
    Ok(())
}
   
#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        space = 8 + Pool::INIT_SPACE,
        seeds = [mint.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init, 
        token::mint = mint,
        token::authority = pool_token_account,
        payer = signer,
        seeds = [b"treasury", mint.key().as_ref()],
        bump
    )]
    pub pool_token_account: InterfaceAccount<'info, TokenAccount>, 
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System> 
}
