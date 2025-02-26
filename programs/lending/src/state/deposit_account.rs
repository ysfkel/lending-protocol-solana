use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DepositAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub shares: u64, 
}