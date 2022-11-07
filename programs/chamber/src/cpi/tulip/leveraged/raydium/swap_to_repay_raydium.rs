use crate::error::ChamberError;
use anchor_lang::{
    prelude::*,
    solana_program::{self, instruction::Instruction},
};
use anchor_spl::token;
use sighashdb::GlobalSighashDB;

#[derive(Accounts)]
pub struct SwapToRepayRaydium<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub vault_signer: AccountInfo<'info>,
    pub swap_or_liquidity_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub amm_id: AccountInfo<'info>,
    #[account(mut)]
    pub amm_authority: AccountInfo<'info>,
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub amm_quantities_or_target_orders: AccountInfo<'info>,
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    pub serum_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub serum_vault_signer: AccountInfo<'info>,
    #[account(mut)]
    pub coin_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub pc_wallet: AccountInfo<'info>,

    /// Remaining accounts:
    #[account(mut)]
    pub lending_market_account: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market_authority: AccountInfo<'info>,
    pub lending_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub asset_price_account: AccountInfo<'info>,
    #[account(mut)]
    pub base_price_account: AccountInfo<'info>,
    #[account(mut)]
    pub quote_price_account: AccountInfo<'info>,
    #[account(mut)]
    pub asset_vault: AccountInfo<'info>,
    #[account(mut)]
    pub user_position_info: AccountInfo<'info>,
    #[account(mut)]
    pub first_reserve: AccountInfo<'info>,
    pub first_reserve_price: AccountInfo<'info>,
    #[account(mut)]
    pub second_reserve: AccountInfo<'info>,
    pub second_reserve_price: AccountInfo<'info>,
}

pub fn tulip_swap_to_repay_raydium(
    accounts: Box<SwapToRepayRaydium>,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("swap_to_repay_raydium")?;
    let mut ix_data = Vec::with_capacity(9);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());

    let accounts = accounts.to_account_metas(None);

    Some(Instruction {
        program_id: tulipv2_sdk_levfarm::ID,
        accounts,
        data: ix_data,
    })
}

#[allow(unused)]
pub fn swap_to_repay_raydium<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<SwapToRepayRaydium<'info>>>,
    obligation_index: u8,
) -> Result<()> {
    let account_infos = ctx.accounts.to_account_infos();

    let ix = tulip_swap_to_repay_raydium(ctx.accounts, obligation_index)
        .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &account_infos, ctx.signer_seeds)
        .map_err(Into::into)
}
