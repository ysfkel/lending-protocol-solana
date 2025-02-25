use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self,Mint, TokenAccount, TokenInterface};

use crate::{Reserve, TREASURY_SEED};

pub fn init_reserve(ctx: Context<InitReserve>, liquidation_threshold: u64, max_ltx: u64) -> Result<()> {
    let reserve = &mut ctx.accounts.reserve;
    reserve.mint = ctx.accounts.mint.key();
    reserve.authority = ctx.accounts.signer.key();
    reserve.liquidation_threshold = liquidation_threshold;
    reserve.max_ltv = max_ltx;
    Ok(())
}
   
#[derive(Accounts)]
pub struct InitReserve<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        space = 8 + Reserve::INIT_SPACE,
        seeds = [mint.key().as_ref()],
        bump,
    )]
    pub reserve: Account<'info, Reserve>,

    #[account(
        init, 
        token::mint = mint,
        token::authority = reserve_token_account,
        payer = signer,
        seeds = [TREASURY_SEED, mint.key().as_ref()],
        bump
    )]
    pub reserve_token_account: InterfaceAccount<'info, TokenAccount>, 
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System> 
}
