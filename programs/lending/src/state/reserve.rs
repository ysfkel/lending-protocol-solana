use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Reserve {
    pub authority:Pubkey,
    pub mint: Pubkey,         // Token mint (e.g., USDC, SOL)
    pub total_deposits: u64,  // Total deposits in the reserve
    pub total_shares: u64,    // Total LP shares issued
    pub total_borrows: u64,
    pub last_updated: i64, 
    // the following constants used to calculate if an account is healthy or not - are assigned based on the volatility of an asset
    // so they are asset specific contants
    pub liquidation_threshold: u64, // The loan to value under which a loan is defined as under collateralized and can be liquidated
    pub liquidiation_bonus: u64,  // Percent of liquidation that will be sent to liquidator as a bonus for processing liquidation
    pub liquidiation_close_factor: u64, // Percent of collateral that can be liquidated 
    pub max_ltv: u64 ,   // max percentage of collateral that can be borrowed for a specific asset
} 

impl Reserve {
    pub fn calculate_user_shares(&self, amount: u64) -> u64 {

        if self.total_deposits == 0 {
            return 0;
        }
        
        let user_shares = (amount * self.total_shares) / self.total_deposits;
        user_shares
    }
}