use crate::error::ChamberError;
use anchor_lang::{
    prelude::*,
    solana_program::{self, instruction::Instruction},
};
use anchor_spl::token;
use sighashdb::GlobalSighashDB;

#[derive(Accounts)]
pub struct SwapToRepayOrca<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    #[account(mut)]
    pub pc_wallet: AccountInfo<'info>,

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
    #[account(mut)]
    pub order_payer_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
    pub market_vault_signer: AccountInfo<'info>,
    #[account(mut)]
    pub coin_wallet: AccountInfo<'info>,

    pub token_program: Program<'info, token::Token>,
    pub rent_sysvar: Sysvar<'info, Rent>,
    pub dex_program: AccountInfo<'info>,
    pub vault_signer: AccountInfo<'info>,

    /// Remaining accounts:
    #[account(mut)]
    pub serum_fee_recipient: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market_authority: AccountInfo<'info>,
    pub lending_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub user_position_info_address: AccountInfo<'info>,
    #[account(mut)]
    pub asset_price_account: AccountInfo<'info>,
    #[account(mut)]
    pub base_price_account: AccountInfo<'info>,
    #[account(mut)]
    pub quote_price_account: AccountInfo<'info>,
    #[account(mut)]
    pub asset_vault: AccountInfo<'info>,
    #[account(mut)]
    pub asset_lp_mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub first_reserve: AccountInfo<'info>,
    #[account(mut)]
    pub first_reserve_price: AccountInfo<'info>,
    #[account(mut)]
    pub second_reserve: AccountInfo<'info>,
    #[account(mut)]
    pub second_reserve_price: AccountInfo<'info>,
}

pub fn tulip_swap_to_repay_orca(
    accounts: Box<SwapToRepayOrca>,
    reserves: &Vec<Pubkey>,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("swap_to_repay_orca")?;
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
pub fn swap_to_repay_orca<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, SwapToRepayOrca<'info>>,
    reserves: &Vec<Pubkey>,
    obligation_index: u8,
) -> Result<()> {
    let account_infos = ctx.accounts.to_account_infos();

    let ix = tulip_swap_to_repay_orca(Box::new(ctx.accounts), reserves, obligation_index)
        .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &account_infos, ctx.signer_seeds)
        .map_err(Into::into)
}
