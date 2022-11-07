use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::withdraw_orca_vault_dd_close::{
    withdraw_orca_vault_dd_close as tulip_withdraw_orca_vault_dd_close,
    WithdrawOrcaVaultDoubleDip as TulipWithdrawOrcaVaultDoubleDip,
};

#[derive(Accounts)]
pub struct WithdrawOrcaVaultDoubleDip<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault_user_account: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
    #[account(mut)]
    pub vault_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    // #[account(mut)]
    // pub user_farm_owner: AccountInfo<'info>,
    // pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    // this is the address of the vault's "converted" pool/lp token account
    pub user_farm_token_account: AccountInfo<'info>,
    // this is the address of vault's "double converted" pool/lp token account
    #[account(mut)]
    pub user_farm_dd_token_account: AccountInfo<'info>,
    #[account(mut)]
    // this is the address of the vault's reward token account
    pub user_reward_dd_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub global_base_dd_token_vault: AccountInfo<'info>,
    #[account(mut)]
    pub farm_dd_token_mint: AccountInfo<'info>,
    #[account(mut)]
    pub global_farm_dd: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_dd: AccountInfo<'info>,
    #[account(mut)]
    pub global_reward_dd_token_vault: AccountInfo<'info>,
    pub convert_authority_dd: AccountInfo<'info>,
    pub aqua_farm_program: AccountInfo<'info>,
    #[account(mut)]
    pub leveraged_user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    pub solfarm_vault_program: AccountInfo<'info>,
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub user_farm_obligation: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
    pub position_info_account: AccountInfo<'info>,
}

impl<'info> From<&WithdrawOrcaVaultDoubleDip<'info>> for TulipWithdrawOrcaVaultDoubleDip<'info> {
    fn from(args: &WithdrawOrcaVaultDoubleDip<'info>) -> TulipWithdrawOrcaVaultDoubleDip<'info> {
        TulipWithdrawOrcaVaultDoubleDip {
            authority: args.authority.clone(),
            vault_account: args.vault_account.clone(),
            vault_user_account: args.vault_user_account.clone(),
            token_program: args.token_program.to_account_info(),
            rent: args.rent.to_account_info(),
            vault_pda: args.vault_pda.clone(),
            system_program: args.system_program.to_account_info(),
            user_farm_token_account: args.user_farm_token_account.clone(),
            user_farm_dd_token_account: args.user_farm_dd_token_account.clone(),
            user_reward_dd_token_account: args.user_reward_dd_token_account.clone(),
            global_base_dd_token_vault: args.global_base_dd_token_vault.clone(),
            farm_dd_token_mint: args.farm_dd_token_mint.clone(),
            global_farm_dd: args.global_farm_dd.clone(),
            user_farm_dd: args.user_farm_dd.clone(),
            global_reward_dd_token_vault: args.global_reward_dd_token_vault.clone(),
            convert_authority_dd: args.convert_authority_dd.clone(),
            aqua_farm_program: args.aqua_farm_program.clone(),
            leveraged_user_farm: args.leveraged_user_farm.clone(),
            leveraged_farm: args.leveraged_farm.clone(),
            solfarm_vault_program: args.solfarm_vault_program.clone(),
            obligation_vault_address: args.obligation_vault_address.clone(),
        }
    }
}

#[allow(unused)]
pub fn withdraw_orca_vault_dd_close<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, WithdrawOrcaVaultDoubleDip<'info>>,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8,
) -> Result<()> {
    let ix = tulip_withdraw_orca_vault_dd_close(
        (&ctx.accounts).into(),
        &ctx.accounts.lending_market_account,
        &ctx.accounts.user_farm_obligation,
        &ctx.accounts.lending_market_authority,
        &ctx.accounts.lending_program,
        &ctx.accounts.position_info_account,
        obligation_index,
        withdraw_percent,
        close_method,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.accounts.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
