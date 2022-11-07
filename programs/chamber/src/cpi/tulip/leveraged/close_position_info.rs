use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use tulipv2_sdk_levfarm::instructions::close_position_info::{
    close_position_info_account as tulip_close_position_info_account, ClosePositionInfoAccount,
};

#[allow(unused)]
pub fn close_position_info_account<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<ClosePositionInfoAccount<'info>>>,
) -> Result<()> {
    let account_infos = ctx.accounts.to_account_infos();

    let ix = tulip_close_position_info_account(*ctx.accounts)
        .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &account_infos, ctx.signer_seeds)
        .map_err(Into::into)
}
