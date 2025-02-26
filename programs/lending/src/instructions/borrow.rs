use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint,TokenInterface, TokenAccount}};

use crate::{DepositAccount, Reserve, TREASURY_SEED};
#[derive(Accounts)]
pub struct Borrow<'info> {
 
   #[account(mut)]
   pub signer: Signer<'info>,
   pub mint: InterfaceAccount<'info, Mint>,

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
      associated_token::token_program = token_program
    
   )]
   pub user_token_account: InterfaceAccount<'info, TokenAccount>,
   

//    pub deposit_account: Box<Account<'info, DepositAccount>>,
   pub token_program: Interface<'info, TokenInterface>,
   pub associated_token_program: Program<'info, AssociatedToken>,
   pub system_program: Program<'info, System>

}