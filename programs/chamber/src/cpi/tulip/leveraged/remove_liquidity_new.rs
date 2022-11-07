use crate::error::ChamberError;
use anchor_lang::{
    prelude::*,
    solana_program::{self, instruction::Instruction},
};
use anchor_spl::token;
use sighashdb::GlobalSighashDB;

/// TODO: Function depends on internal AMM's(include orca support).
#[derive(Accounts)]
pub struct RemoveLiquidityNew<'info> {
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub liquidity_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub amm_id: AccountInfo<'info>,
    #[account(mut)]
    pub amm_authority: AccountInfo<'info>,
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub amm_quantities_or_target_orders: AccountInfo<'info>,
    #[account(mut)]
    pub lp_mint_address: AccountInfo<'info>,
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pool_withdraw_queue: AccountInfo<'info>,
    #[account(mut)]
    pub pool_temp_lp_token_account: AccountInfo<'info>,
    pub serum_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub serum_vault_signer: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    #[account(mut)]
    pub lev_farm_coin_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub lev_farm_pc_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_lp_token_account: AccountInfo<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,

    /// Remaining accounts section:
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_obligation_account: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market_authority: AccountInfo<'info>,
    pub lending_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub user_position_info: AccountInfo<'info>,
    // TODO: Raydium related accounts:
    /* #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub serum_market_bids: AccountInfo<'info>,
    #[account(mut)]
    pub serum_market_asks: AccountInfo<'info>, */
}

pub fn tulip_remove_liquidity_new(
    accounts: Box<RemoveLiquidityNew>,
    obligation_index: u8,
    obligation_vault_nonce: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("remove_liquidity_new")?;
    let mut ix_data = Vec::with_capacity(10);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_vault_nonce).unwrap());

    let accounts = accounts.to_account_metas(None);

    Some(Instruction {
        program_id: tulipv2_sdk_levfarm::ID,
        accounts,
        data: ix_data,
    })
}

#[allow(unused)]
pub fn remove_liquidity_new<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<RemoveLiquidityNew<'info>>>,
    obligation_index: u8,
    obligation_vault_nonce: u8,
) -> Result<()> {
    let account_infos = ctx.accounts.to_account_infos();

    let ix = tulip_remove_liquidity_new(ctx.accounts, obligation_index, obligation_vault_nonce)
        .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &account_infos, ctx.signer_seeds)
        .map_err(Into::into)
}
