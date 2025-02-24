use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub owner: Pubkey ,
    pub last_updated: i64,  
}

#[account]
#[derive(InitSpace)]
pub struct UserAssetBalance {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub deposited: u64,
    pub deposited_shares: u64,
    pub borrowed: u64,
    pub borrowed_shares: u64,
}