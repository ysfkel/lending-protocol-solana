use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("ZeroAmount")]
    AmountZero,
    #[msg("OverflowError")]
    OverflowError,
    #[msg("IntegerConversionError")]
    IntegerConversionError
}

