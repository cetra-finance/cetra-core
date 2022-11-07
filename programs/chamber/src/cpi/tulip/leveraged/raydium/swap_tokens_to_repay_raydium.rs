use super::SwapToRepayRaydium;
use crate::error::ChamberError;
use anchor_lang::{
    prelude::*,
    solana_program::{self, instruction::Instruction},
};
use sighashdb::GlobalSighashDB;

pub fn tulip_swap_tokens_to_repay_raydium(
    accounts: Box<SwapToRepayRaydium>,
    obligation_index: u8,
    min_coin_swap: u64,
    min_pc_swap: u64,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("swap_tokens_to_repay_raydium")?;
    let mut ix_data = Vec::with_capacity(25);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&min_coin_swap).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&min_pc_swap).unwrap());

    let accounts = accounts.to_account_metas(None);

    Some(Instruction {
        program_id: tulipv2_sdk_levfarm::ID,
        accounts,
        data: ix_data,
    })
}

#[allow(unused)]
pub fn swap_tokens_to_repay_raydium<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<SwapToRepayRaydium<'info>>>,
    obligation_index: u8,
    min_coin_swap: u64,
    min_pc_swap: u64,
) -> Result<()> {
    let account_infos = ctx.accounts.to_account_infos();

    let ix = tulip_swap_tokens_to_repay_raydium(
        ctx.accounts,
        obligation_index,
        min_coin_swap,
        min_pc_swap,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &account_infos, ctx.signer_seeds)
        .map_err(Into::into)
}
