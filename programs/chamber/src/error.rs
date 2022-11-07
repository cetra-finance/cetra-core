use anchor_lang::prelude::*;

#[error_code]
pub enum ChamberError {
    #[msg("CPI instruction formation is failed")]
    CpiInstructionFormationFailed,

    #[msg("Invalid user account status")]
    InvalidUserAccountStatus,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Math overflow")]
    MathOverflow,
}
