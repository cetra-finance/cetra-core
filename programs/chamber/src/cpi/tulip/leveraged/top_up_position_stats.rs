use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::top_up_position_stats::{
    top_up_position_stats as tulip_top_up_position_stats,
    DepositObligationCollateral as TulipDepositObligationCollateral,
};

#[derive(Accounts)]
pub struct DepositObligationCollateral<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    // this is the obligation account
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    #[account(mut)]
    pub coin_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    // false positive
    //#[soteria(ignore)]
    pub coin_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pc_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pc_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub coin_deposit_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    pub pc_deposit_reserve_account: AccountInfo<'info>,
    pub coin_reserve_liquidity_oracle: AccountInfo<'info>,
    pub pc_reserve_liquidity_oracle: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub derived_lending_market_authority: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub lending_program: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub position_info_account: AccountInfo<'info>,
}

impl<'info> From<&DepositObligationCollateral<'info>> for TulipDepositObligationCollateral<'info> {
    fn from(args: &DepositObligationCollateral<'info>) -> TulipDepositObligationCollateral<'info> {
        TulipDepositObligationCollateral {
            authority: args.authority.clone(),
            user_farm: args.user_farm.clone(),
            leveraged_farm: args.leveraged_farm.clone(),
            user_farm_obligation: args.user_farm_obligation.clone(),
            coin_source_token_account: args.coin_source_token_account.clone(),
            coin_destination_token_account: args.coin_destination_token_account.clone(),
            pc_source_token_account: args.pc_source_token_account.clone(),
            pc_destination_token_account: args.pc_destination_token_account.clone(),
            coin_deposit_reserve_account: args.coin_deposit_reserve_account.clone(),
            pc_deposit_reserve_account: args.pc_deposit_reserve_account.clone(),
            coin_reserve_liquidity_oracle: args.coin_reserve_liquidity_oracle.clone(),
            pc_reserve_liquidity_oracle: args.pc_reserve_liquidity_oracle.clone(),
            lending_market_account: args.lending_market_account.clone(),
            derived_lending_market_authority: args.derived_lending_market_authority.clone(),
            clock: args.clock.clone(),
            lending_program: args.lending_program.clone(),
            token_program: args.token_program.to_account_info(),
        }
    }
}

#[allow(unused)]
pub fn top_up_position_stats<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<DepositObligationCollateral<'info>>>,
    coin_amount: u64,
    pc_amount: u64,
    obligation_index: u8,
) -> Result<()> {
    let ix = tulip_top_up_position_stats(
        ctx.accounts.as_ref().into(),
        &ctx.accounts.position_info_account,
        coin_amount,
        pc_amount,
        obligation_index,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
