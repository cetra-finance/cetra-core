use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::swap_tokens_orca_stats::{
    swap_tokens_orca_stats as tulip_swap_tokens_orca_stats, MarketAccounts as TulipMarketAccounts,
    NewSerumSwap as TulipNewSerumSwap,
};

#[derive(Accounts)]
pub struct NewSerumSwap<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    // ensure global account of leveraged farm matches the given global account
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    // #[account(mut)]
    // pub user_farm_manager: Loader<'info, UserFarmManager>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    // i believe this is the wallet that will receive funds??
    // or is this the wallet that will contain the price coin
    #[account(mut)]
    pub pc_wallet: AccountInfo<'info>,
    pub market: MarketAccounts<'info>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
    pub dex_program: AccountInfo<'info>,
    pub vault_signer: AccountInfo<'info>,
    pub serum_fee_recipient: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
    pub lp_mint: AccountInfo<'info>,
    pub position_info_account: AccountInfo<'info>,
}

impl<'info> From<&NewSerumSwap<'info>> for TulipNewSerumSwap<'info> {
    fn from(args: &NewSerumSwap<'info>) -> TulipNewSerumSwap<'info> {
        TulipNewSerumSwap {
            authority: args.authority.clone(),
            leveraged_farm: args.leveraged_farm.clone(),
            user_farm: args.user_farm.clone(),
            user_farm_obligation: args.user_farm_obligation.clone(),
            pc_wallet: args.pc_wallet.clone(),
            market: TulipMarketAccounts {
                market: args.market.market.clone(),
                open_orders: args.market.open_orders.clone(),
                request_queue: args.market.request_queue.clone(),
                event_queue: args.market.event_queue.clone(),
                bids: args.market.bids.clone(),
                asks: args.market.asks.clone(),
                order_payer_token_account: args.market.order_payer_token_account.clone(),
                coin_vault: args.market.coin_vault.clone(),
                pc_vault: args.market.pc_vault.clone(),
                vault_signer: args.market.vault_signer.clone(),
                coin_wallet: args.market.coin_wallet.clone(),
            },
            token_program: args.token_program.to_account_info(),
            rent: args.rent.clone(),
            dex_program: args.dex_program.clone(),
            vault_signer: args.vault_signer.clone(),
        }
    }
}

// Market accounts are the accounts used to place orders against the dex minus
// common accounts, i.e., program ids, sysvars, and the `pc_wallet`.
#[derive(Accounts)]
pub struct MarketAccounts<'info> {
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub request_queue: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    // The `spl_token::Account` that funds will be taken from, i.e., transferred
    // from the user into the market's vault.
    //
    // For bids, this is the base currency. For asks, the quote.
    #[account(mut)]
    pub order_payer_token_account: AccountInfo<'info>,
    // Also known as the "base" currency. For a given A/B market,
    // this is the vault for the A mint.
    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,
    // Also known as the "quote" currency. For a given A/B market,
    // this is the vault for the B mint.
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
    // PDA owner of the DEX's token accounts for base + quote currencies.
    pub vault_signer: AccountInfo<'info>,
    // User wallets.
    #[account(mut)]
    pub coin_wallet: AccountInfo<'info>,
}

#[allow(unused)]
pub fn swap_tokens_orca_stats<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, NewSerumSwap<'info>>,
    obligation_index: u8,
) -> Result<()> {
    let ix = tulip_swap_tokens_orca_stats(
        (&ctx.accounts).into(),
        &ctx.accounts.serum_fee_recipient,
        &ctx.accounts.lending_market_account,
        &ctx.accounts.lending_market_authority,
        &ctx.accounts.lending_program,
        &ctx.accounts.lp_mint,
        &ctx.accounts.position_info_account,
        obligation_index,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.accounts.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
