use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{self,
        TransferChecked,
     Mint, TokenAccount, TokenInterface}};
use crate::{TREASURY_SEED,DEPOSIT_ACCOUNT_SEED, Reserve,UserAssetBalance, error::ErrorCode};


pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    
    require_gt!(amount, 0, ErrorCode::AmountZero);

    require_gte!(ctx.accounts.user_asset_balance.deposited, amount, ErrorCode::InsufficientBalance);

    let reserve = &mut ctx.accounts.reserve;
    let shares_to_remove = reserve.calculate_shares(amount)?;
   
    //  update user_asset_balance
    let user_asset_balance = &mut ctx.accounts.user_asset_balance;
    user_asset_balance.deposited  -= amount;
    user_asset_balance.deposited_shares -= shares_to_remove;

    //  update reserve
    reserve.total_deposits -= amount;
    reserve.total_shares -= shares_to_remove;

    // Transfer amount 
    let mint = ctx.accounts.mint.to_account_info();
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.reserve_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.reserve_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info()
    };

    let mint_key = ctx.accounts.mint.key();
    let signer_seeds: &[&[&[u8]]] = &[
        &[
            TREASURY_SEED,
            mint_key.as_ref(),
            &[ctx.bumps.reserve_token_account]
        ]
    ];

    let cpi_program =ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program,cpi_accounts).with_signer(signer_seeds);
    
    token_interface::transfer_checked(cpi_context, amount, ctx.accounts.mint.decimals)?;

    Ok(())
}

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
        seeds = [DEPOSIT_ACCOUNT_SEED, signer.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub user_asset_balance: Box<Account<'info, UserAssetBalance>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>

}