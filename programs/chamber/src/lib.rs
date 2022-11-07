#![allow(clippy::result_large_err)]

mod cpi;
pub mod error;
mod processor;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use processor::*;

declare_id!("cmbrLdggVpadQMe54SMWVvSA6ajswMSBtwnLG2xyqZE");

#[program]
mod chamber {
    use super::*;

    pub fn initialize_chamber<'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, InitializeChamber<'info>>,
        market: crate::state::ChamberMarket,
        leverage: u64,
        is_base_volatile: bool,
        chamber_nonce: u8,
        authority_bump: u8,
    ) -> Result<()> {
        ctx.accounts.process(
            ctx.remaining_accounts,
            market,
            leverage,
            is_base_volatile,
            chamber_nonce,
            authority_bump,
        )
    }

    pub fn create_user_account<'info>(ctx: Context<CreateUserAccount<'info>>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn begin_deposit_chamber<'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, BeginDepositChamber<'info>>,
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .process(ctx.remaining_accounts, base_amount, quote_amount)
    }

    pub fn process_deposit_chamber<'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, ProcessDepositChamber<'info>>,
    ) -> Result<()> {
        ctx.accounts.process(ctx.remaining_accounts)
    }

    pub fn end_deposit_chamber<'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, EndDepositChamber<'info>>,
    ) -> Result<()> {
        ctx.accounts.process(ctx.remaining_accounts)
    }

    pub fn deposit_chamber<'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, DepositChamber<'info>>,
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .process(ctx.remaining_accounts, base_amount, quote_amount)
    }

    pub fn withdraw_chamber<'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, WithdrawChamber<'info>>,
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .process(ctx.remaining_accounts, base_amount, quote_amount)
    }

    pub fn rebalance_chamber<'c, 'info>(
        ctx: Context<'_, '_, 'c, 'info, RebalanceChamber<'info>>,
    ) -> Result<()> {
        ctx.accounts.process(ctx.remaining_accounts)
    }
}
