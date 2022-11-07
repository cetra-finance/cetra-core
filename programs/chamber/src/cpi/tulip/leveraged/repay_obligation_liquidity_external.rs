use crate::error::ChamberError;
use anchor_lang::{
    prelude::*,
    solana_program::{self, instruction::Instruction},
};
use anchor_spl::token;
use sighashdb::GlobalSighashDB;

#[derive(Accounts)]
pub struct RepayObligationLiquidityExternal<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    #[account(mut)]
    pub coin_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub coin_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pc_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pc_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub coin_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    pub pc_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market_account: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market_authority: AccountInfo<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, token::Token>,
    pub lending_program: AccountInfo<'info>,
    pub lp_pyth_price_account: AccountInfo<'info>,
    pub coin_price_account: AccountInfo<'info>,
    pub pc_price_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_coin_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_pc_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub position_info_account: AccountInfo<'info>,
    #[account(mut)]
    pub first_reserve: AccountInfo<'info>,
    pub first_reserve_price: AccountInfo<'info>,
    #[account(mut)]
    pub second_reserve: AccountInfo<'info>,
    pub second_reserve_price: AccountInfo<'info>,
}

pub fn tulip_repay_obligation_liquidity_external(
    accounts: Box<RepayObligationLiquidityExternal>,
    reserves: &Vec<Pubkey>,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("repay_obligation_liquidity_external")?;
    let mut ix_data = Vec::with_capacity(73);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&reserves).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());

    let accounts = accounts.to_account_metas(None);

    Some(Instruction {
        program_id: tulipv2_sdk_levfarm::ID,
        accounts,
        data: ix_data,
    })
}

#[allow(unused)]
pub fn repay_obligation_liquidity_external<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<RepayObligationLiquidityExternal<'info>>>,
    reserves: &Vec<Pubkey>,
    obligation_index: u8,
) -> Result<()> {
    let account_infos = ctx.accounts.to_account_infos();

    let ix = tulip_repay_obligation_liquidity_external(ctx.accounts, reserves, obligation_index)
        .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &account_infos, ctx.signer_seeds)
        .map_err(Into::into)
}
