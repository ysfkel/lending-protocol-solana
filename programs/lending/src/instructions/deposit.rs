use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{self,Mint, TokenAccount,TokenInterface, TransferChecked}};

use crate::seeds::USER_ASSET_BALANCE_SEED;
use crate::{Reserve, UserAssetBalance, TREASURY_SEED};

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {

    let decimals = ctx.accounts.mint.decimals;

    // reserve
    let reserve = &mut ctx.accounts.reserve;
    let user_shares = if reserve.total_deposits == 0 {
        reserve.total_deposits = amount;
        reserve.total_shares = amount;
         amount
    } else {
        let user_shares = reserve.calculate_user_shares(amount);
        reserve.total_deposits += amount; 
        reserve.total_shares += user_shares;
        user_shares
    };
    
    // update user_asset_balance 
    let user_asset_balance = &mut ctx.accounts.user_asset_balance;
    if user_asset_balance.mint == Pubkey::default() {
        user_asset_balance.owner = ctx.accounts.signer.key();
        user_asset_balance.mint = ctx.accounts.mint.key();
        user_asset_balance.deposited = amount;
        user_asset_balance.deposited_shares = user_shares;  
    } else  {
        user_asset_balance.deposited +=amount ;
        user_asset_balance.deposited_shares +=user_shares;
    }

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.reserve_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.signer.to_account_info()
    };

    let cpi_program: AccountInfo<'_> = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program,cpi_accounts);
    token_interface::transfer_checked(cpi_context, amount, decimals)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [mint.key().as_ref()],
        bump,
    )]
    pub reserve: Account<'info, Reserve>,

    #[account(
        mut,
        seeds = [TREASURY_SEED, mint.key().as_ref()],
        bump
    )]
    pub reserve_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        space = 8 + UserAssetBalance::INIT_SPACE,
        payer = signer, 
        seeds = [USER_ASSET_BALANCE_SEED, signer.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub user_asset_balance: Account<'info, UserAssetBalance>, 


    pub associated_token: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}