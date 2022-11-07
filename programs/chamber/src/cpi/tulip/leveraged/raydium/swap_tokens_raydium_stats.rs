use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::swap_tokens_raydium_stats::{
    swap_tokens_raydium_stats as tulip_swap_tokens_raydium_stats, RaydiumSwap as TulipRaydiumSwap,
};

#[derive(Accounts)]
pub struct RaydiumSwap<'info> {
    pub authority: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub user_farm: AccountInfo<'info>,
    pub user_farm_obligation: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub vault_signer: AccountInfo<'info>,
    pub swap_or_liquidity_program_id: AccountInfo<'info>,
    pub amm_id: AccountInfo<'info>,
    pub amm_authority: AccountInfo<'info>,
    pub amm_open_orders: AccountInfo<'info>,
    pub amm_quantities_or_target_orders: AccountInfo<'info>,
    pub pool_coin_tokenaccount: AccountInfo<'info>,
    pub pool_pc_tokenaccount: AccountInfo<'info>,
    pub serum_program_id: AccountInfo<'info>,
    pub serum_market: AccountInfo<'info>,
    pub serum_bids: AccountInfo<'info>,
    pub serum_asks: AccountInfo<'info>,
    pub serum_event_queue: AccountInfo<'info>,
    pub serum_coin_vault_account: AccountInfo<'info>,
    pub serum_pc_vault_account: AccountInfo<'info>,
    pub serum_vault_signer: AccountInfo<'info>,
    pub coin_wallet: AccountInfo<'info>,
    pub pc_wallet: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
    pub position_info_account: AccountInfo<'info>,
}

impl<'info> From<&RaydiumSwap<'info>> for TulipRaydiumSwap {
    fn from(args: &RaydiumSwap<'info>) -> TulipRaydiumSwap {
        TulipRaydiumSwap {
            authority: args.authority.key(),
            leveraged_farm: args.leveraged_farm.key(),
            user_farm: args.user_farm.key(),
            user_farm_obligation: args.user_farm_obligation.key(),
            token_program: args.token_program.key(),
            vault_signer: args.vault_signer.key(),
            swap_or_liquidity_program_id: args.swap_or_liquidity_program_id.key(),
            amm_id: args.amm_id.key(),
            amm_authority: args.amm_authority.key(),
            amm_open_orders: args.amm_open_orders.key(),
            amm_quantities_or_target_orders: args.amm_quantities_or_target_orders.key(),
            pool_coin_tokenaccount: args.pool_coin_tokenaccount.key(),
            pool_pc_tokenaccount: args.pool_pc_tokenaccount.key(),
            serum_program_id: args.serum_program_id.key(),
            serum_market: args.serum_market.key(),
            serum_bids: args.serum_bids.key(),
            serum_asks: args.serum_asks.key(),
            serum_event_queue: args.serum_event_queue.key(),
            serum_coin_vault_account: args.serum_coin_vault_account.key(),
            serum_pc_vault_account: args.serum_pc_vault_account.key(),
            serum_vault_signer: args.serum_vault_signer.key(),
            coin_wallet: args.coin_wallet.key(),
            pc_wallet: args.pc_wallet.key(),
        }
    }
}

#[allow(unused)]
pub fn swap_tokens_raydium_stats<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<RaydiumSwap<'info>>>,
    obligation_index: u8,
) -> Result<()> {
    let ix = tulip_swap_tokens_raydium_stats(
        Box::new(ctx.accounts.as_ref().into()),
        ctx.accounts.lending_market_account.key(),
        ctx.accounts.lending_market_authority.key(),
        ctx.accounts.lending_program.key(),
        ctx.accounts.position_info_account.key(),
        obligation_index,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.accounts.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
