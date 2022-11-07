use anchor_lang::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum UserAccountStatus {
    Ready,
    BeginDeposit,
    ProcessDeposit,
}
