use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::withdraw_raydium_vault_close::{
    withdraw_raydium_vault_close as tulip_withdraw_raydium_vault_close,
    WithdrawFarm as TulipWithdrawFarm,
};

#[derive(Accounts)]
pub struct WithdrawFarm<'info> {
    pub authority: AccountInfo<'info>,
    pub user_farm: AccountInfo<'info>,
    pub obligation_vault_address: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub authority_token_account: AccountInfo<'info>,
    pub vault: AccountInfo<'info>,
    pub vault_program: AccountInfo<'info>,
    pub user_balance_account: AccountInfo<'info>,
    pub user_info_account: AccountInfo<'info>,
    pub user_lp_token_account: AccountInfo<'info>,
    pub user_reward_a_token_account: AccountInfo<'info>,
    pub pool_reward_a_token_account: AccountInfo<'info>,
    pub user_reward_b_token_account: AccountInfo<'info>,
    pub pool_reward_b_token_account: AccountInfo<'info>,
    pub token_program_id: Program<'info, token::Token>,
    pub clock: Sysvar<'info, Clock>,
    pub vault_pda_account: AccountInfo<'info>,
    pub pool_lp_token_account: AccountInfo<'info>,
    pub pool_authority: AccountInfo<'info>,
    pub pool_id: AccountInfo<'info>,
    pub stake_program_id: AccountInfo<'info>,
    pub user_balance_meta: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub user_farm_obligation: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
    pub position_info_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> From<&WithdrawFarm<'info>> for TulipWithdrawFarm {
    fn from(args: &WithdrawFarm<'info>) -> TulipWithdrawFarm {
        TulipWithdrawFarm {
            authority: args.authority.key(),
            user_farm: args.user_farm.key(),
            obligation_vault_address: args.obligation_vault_address.key(),
            leveraged_farm: args.leveraged_farm.key(),
            authority_token_account: args.authority_token_account.key(),
            vault: args.vault.key(),
            vault_program: args.vault_program.key(),
            user_balance_account: args.user_balance_account.key(),
            user_info_account: args.user_info_account.key(),
            user_lp_token_account: args.user_lp_token_account.key(),
            user_reward_a_token_account: args.user_reward_a_token_account.key(),
            pool_reward_a_token_account: args.pool_reward_a_token_account.key(),
            user_reward_b_token_account: args.user_reward_b_token_account.key(),
            pool_reward_b_token_account: args.pool_reward_b_token_account.key(),
            token_program_id: args.token_program_id.key(),
            clock: args.clock.key(),
            vault_pda_account: args.vault_pda_account.key(),
            pool_lp_token_account: args.pool_lp_token_account.key(),
            pool_authority: args.pool_authority.key(),
            pool_id: args.pool_id.key(),
            stake_program_id: args.stake_program_id.key(),
            user_balance_meta: args.user_balance_meta.key(),
        }
    }
}

#[allow(unused)]
pub fn withdraw_raydium_vault_close<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<WithdrawFarm<'info>>>,
    meta_nonce: u8,
    nonce: u8,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8,
) -> Result<()> {
    let ix = tulip_withdraw_raydium_vault_close(
        Box::new(ctx.accounts.as_ref().into()),
        ctx.accounts.lending_market_account.key(),
        ctx.accounts.user_farm_obligation.key(),
        ctx.accounts.lending_market_authority.key(),
        ctx.accounts.lending_program.key(),
        ctx.accounts.position_info_account.key(),
        ctx.accounts.system_program.key(),
        ctx.accounts.rent.key(),
        meta_nonce,
        nonce,
        obligation_index,
        withdraw_percent,
        close_method,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.accounts.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
