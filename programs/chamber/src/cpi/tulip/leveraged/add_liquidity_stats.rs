use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::add_liquidity_stats::{
    add_liquidity_stats as tulip_add_liquidity_stats, AddLiquidity as TulipAddLiquidity,
};

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    pub authority: AccountInfo<'info>,
    pub user_farm: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub liquidity_program_id: AccountInfo<'info>,
    pub amm_id: AccountInfo<'info>,
    pub amm_authority: AccountInfo<'info>,
    pub amm_open_orders: AccountInfo<'info>,
    pub amm_quantities_or_target_orders: AccountInfo<'info>,
    pub lp_mint_address: AccountInfo<'info>,
    pub pool_coin_token_account: AccountInfo<'info>,
    pub pool_pc_token_account: AccountInfo<'info>,
    pub serum_market: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub lev_farm_coin_token_account: AccountInfo<'info>,
    pub lev_farm_pc_token_account: AccountInfo<'info>,
    pub user_lp_token_account: AccountInfo<'info>,
    pub pyth_price_account: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub user_farm_obligation: AccountInfo<'info>,
    pub derived_lending_market_authority: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub dex_program: AccountInfo<'info>,
    pub position_info_account: AccountInfo<'info>,
}

impl<'info> From<&AddLiquidity<'info>> for TulipAddLiquidity {
    fn from(args: &AddLiquidity<'info>) -> TulipAddLiquidity {
        TulipAddLiquidity {
            authority: args.authority.key(),
            user_farm: args.user_farm.key(),
            leveraged_farm: args.leveraged_farm.key(),
            liquidity_program_id: args.liquidity_program_id.key(),
            amm_id: args.amm_id.key(),
            amm_authority: args.amm_authority.key(),
            amm_open_orders: args.amm_open_orders.key(),
            amm_quantities_or_target_orders: args.amm_quantities_or_target_orders.key(),
            lp_mint_address: args.lp_mint_address.key(),
            pool_coin_token_account: args.pool_coin_token_account.key(),
            pool_pc_token_account: args.pool_pc_token_account.key(),
            serum_market: args.serum_market.key(),
            token_program: args.token_program.key(),
            lev_farm_coin_token_account: args.lev_farm_coin_token_account.key(),
            lev_farm_pc_token_account: args.lev_farm_pc_token_account.key(),
            user_lp_token_account: args.user_lp_token_account.key(),
            pyth_price_account: args.pyth_price_account.key(),
            lending_market_account: args.lending_market_account.key(),
            user_farm_obligation: args.user_farm_obligation.key(),
            derived_lending_market_authority: args.derived_lending_market_authority.key(),
            lending_program: args.lending_program.key(),
            clock: args.clock.key(),
            dex_program: args.dex_program.key(),
        }
    }
}

#[allow(unused)]
pub fn add_liquidity_stats<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<AddLiquidity<'info>>>,
    obligation_index: u8,
) -> Result<()> {
    let position_info_account = ctx.accounts.position_info_account.key();

    let ix = tulip_add_liquidity_stats(
        Box::new(ctx.accounts.as_ref().into()),
        position_info_account,
        obligation_index,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
