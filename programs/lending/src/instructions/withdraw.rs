use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
     token_interface::{self,
     Mint, TokenAccount, TokenInterface, AssociatedToken}};
use crate::{TREASURY_SEED, Reserve,UserAssetBalance};
#[derive(Accounts)]
pub struct Withdraw<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [mint.key().as_ref()],
        bump
    )]
    pub reserve: Box<Account<'info, Reserve>>,

    #[account(
        mut,
        seeds = [TREASURY_SEED, mint.key().as_ref()],
        bump
    )]
    pub reserve_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [USER_ASSET_BALANCE_SEED, signer.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub user_asset_balance: Box<Account<'info, UserAssetBalance>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>

}