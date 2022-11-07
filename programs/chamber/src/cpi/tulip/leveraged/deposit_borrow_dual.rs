use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::deposit_borrow_dual::{
    deposit_borrow_dual as tulip_deposit_borrow_dual, DepositBorrowDual as TulipDepositBorrowDual,
};

#[derive(Accounts)]
pub struct DepositBorrowDual<'info> {
    pub authority: AccountInfo<'info>,
    pub user_farm: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub user_farm_obligation: AccountInfo<'info>,
    pub coin_source_token_account: AccountInfo<'info>,
    pub coin_destination_token_account: AccountInfo<'info>,
    pub pc_source_token_account: AccountInfo<'info>,
    pub pc_destination_token_account: AccountInfo<'info>,
    pub coin_deposit_reserve_account: AccountInfo<'info>,
    pub pc_deposit_reserve_account: AccountInfo<'info>,
    pub coin_reserve_liquidity_oracle: AccountInfo<'info>,
    pub pc_reserve_liquidity_oracle: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub derived_lending_market_authority: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub lending_program: AccountInfo<'info>,
    pub coin_source_reserve_liquidity_token_account: AccountInfo<'info>,
    pub pc_source_reserve_liquidity_token_account: AccountInfo<'info>,
    pub coin_reserve_liquidity_fee_receiver: AccountInfo<'info>,
    pub pc_reserve_liquidity_fee_receiver: AccountInfo<'info>,
    pub borrow_authorizer: AccountInfo<'info>,
    pub lp_pyth_price_account: AccountInfo<'info>,
    pub vault_account: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub position_info_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> From<&DepositBorrowDual<'info>> for TulipDepositBorrowDual {
    fn from(args: &DepositBorrowDual<'info>) -> TulipDepositBorrowDual {
        TulipDepositBorrowDual {
            authority: args.authority.key(),
            user_farm: args.user_farm.key(),
            leveraged_farm: args.leveraged_farm.key(),
            user_farm_obligation: args.user_farm_obligation.key(),
            coin_source_token_account: args.coin_source_token_account.key(),
            coin_destination_token_account: args.coin_destination_token_account.key(),
            pc_source_token_account: args.pc_source_token_account.key(),
            pc_destination_token_account: args.pc_destination_token_account.key(),
            coin_deposit_reserve_account: args.coin_deposit_reserve_account.key(),
            pc_deposit_reserve_account: args.pc_deposit_reserve_account.key(),
            coin_reserve_liquidity_oracle: args.coin_reserve_liquidity_oracle.key(),
            pc_reserve_liquidity_oracle: args.pc_reserve_liquidity_oracle.key(),
            lending_market_account: args.lending_market_account.key(),
            derived_lending_market_authority: args.derived_lending_market_authority.key(),
            token_program: args.token_program.key(),
            lending_program: args.lending_program.key(),
            coin_source_reserve_liquidity_token_account: args
                .coin_source_reserve_liquidity_token_account
                .key(),
            pc_source_reserve_liquidity_token_account: args
                .pc_source_reserve_liquidity_token_account
                .key(),
            coin_reserve_liquidity_fee_receiver: args.coin_reserve_liquidity_fee_receiver.key(),
            pc_reserve_liquidity_fee_receiver: args.pc_reserve_liquidity_fee_receiver.key(),
            borrow_authorizer: args.borrow_authorizer.key(),
            lp_pyth_price_account: args.lp_pyth_price_account.key(),
            vault_account: args.vault_account.key(),
            rent: args.rent.key(),
        }
    }
}

#[allow(unused)]
pub fn deposit_borrow_dual<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<DepositBorrowDual<'info>>>,
    coin_amount: u64,
    pc_amount: u64,
    coin_borrow_amount: u64,
    pc_borrow_amount: u64,
    obligation_index: u8,
) -> Result<()> {
    let ix = tulip_deposit_borrow_dual(
        ctx.accounts.as_ref().into(),
        ctx.accounts.position_info_account.key(),
        ctx.accounts.system_program.key(),
        coin_amount,
        pc_amount,
        coin_borrow_amount,
        pc_borrow_amount,
        obligation_index,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.accounts.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
