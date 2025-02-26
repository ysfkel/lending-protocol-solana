use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{self,Mint, TokenAccount,TokenInterface, TransferChecked}};

use crate::{Reserve, DepositAccount,DEPOSIT_ACCOUNT_SEED, TREASURY_SEED, error::ErrorCode};

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {

    require_gt!(amount, 0, ErrorCode::AmountZero);

    let decimals = ctx.accounts.mint.decimals;

    // reserve
    let reserve = &mut ctx.accounts.reserve;

    let user_shares = reserve.increase_deposits_and_shares(amount)?;
    
    // update user_asset_balance 
    let deposit_account = &mut ctx.accounts.deposit_account;

    if deposit_account.mint == Pubkey::default() {
        deposit_account.owner = ctx.accounts.signer.key();
        deposit_account.mint = ctx.accounts.mint.key();
        deposit_account.amount = amount;
        deposit_account.shares = user_shares;  
    } else  {
        deposit_account.amount +=amount ;
        deposit_account.shares +=user_shares;
    }

    // transfer amount 
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
    pub reserve: Box<Account<'info, Reserve>>,

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
        space = 8 + DepositAccount::INIT_SPACE,
        payer = signer, 
        seeds = [DEPOSIT_ACCOUNT_SEED, signer.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub deposit_account: Box<Account<'info, DepositAccount>>, 


    pub associated_token: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}