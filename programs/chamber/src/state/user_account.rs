use super::UserAccountStatus;
use crate::error;
use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct UserAccount {
    pub chamber: Pubkey,
    pub user: Pubkey,
    pub shares: Pubkey,
    pub status: UserAccountStatus,

    pub locked_base_amount: u64,
    pub locked_quote_amount: u64,
    pub locked_shares_amount: u64,
}

impl UserAccount {
    pub const LEN: usize = 8 + 32 * 3 + 1 + 8 * 3;

    pub fn init(&mut self, chamber: &Pubkey, user: &Pubkey, shares: &Pubkey) {
        self.chamber = *chamber;
        self.user = *user;
        self.shares = *shares;
        self.status = UserAccountStatus::Ready;
    }

    pub fn assert_status(&self, status: UserAccountStatus) -> Result<()> {
        if self.status != status {
            Err(error::ChamberError::InvalidUserAccountStatus.into())
        } else {
            Ok(())
        }
    }

    pub fn begin_deposit(
        &mut self,
        locked_base_amount: u64,
        locked_quote_amount: u64,
        locked_shares_amount: u64,
    ) {
        self.status = UserAccountStatus::BeginDeposit;
        self.locked_base_amount = locked_base_amount;
        self.locked_quote_amount = locked_quote_amount;
        self.locked_shares_amount = locked_shares_amount;
    }

    pub fn process_deposit(&mut self) {
        self.status = UserAccountStatus::ProcessDeposit;
    }

    pub fn end_deposit(&mut self) {
        self.status = UserAccountStatus::Ready;
        self.locked_base_amount = 0;
        self.locked_quote_amount = 0;
        self.locked_shares_amount = 0;
    }
}
