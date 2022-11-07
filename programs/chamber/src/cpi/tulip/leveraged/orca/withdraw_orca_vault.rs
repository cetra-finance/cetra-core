use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::withdraw_orca_vault::{
    withdraw_orca_vault_close as tulip_withdraw_orca_vault_close,
    WithdrawOrcaFarm as TulipWithdrawOrcaFarm,
};

#[derive(Accounts)]
pub struct WithdrawOrcaFarm<'info> {
    pub authority: AccountInfo<'info>,
    pub vault_account: AccountInfo<'info>,
    pub vault_user_account: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
    pub vault_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub user_farm_owner: AccountInfo<'info>,
    pub user_transfer_authority: AccountInfo<'info>,
    pub user_base_token_account: AccountInfo<'info>,
    pub user_farm_token_account: AccountInfo<'info>,
    pub user_reward_token_account: AccountInfo<'info>,
    pub global_base_token_vault: AccountInfo<'info>,
    pub farm_token_mint: AccountInfo<'info>,
    pub global_farm: AccountInfo<'info>,
    pub orca_user_farm: AccountInfo<'info>,
    pub global_reward_token_vault: AccountInfo<'info>,
    pub convert_authority: AccountInfo<'info>,
    pub aqua_farm_program: AccountInfo<'info>,
    pub receiving_token_account: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub leveraged_user_farm: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub solfarm_vault_program: AccountInfo<'info>,
    pub obligation_vault_address: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub user_farm_obligation: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
}

impl<'info> From<&WithdrawOrcaFarm<'info>> for TulipWithdrawOrcaFarm {
    fn from(args: &WithdrawOrcaFarm<'info>) -> TulipWithdrawOrcaFarm {
        TulipWithdrawOrcaFarm {
            authority: args.authority.key(),
            vault_account: args.vault_account.key(),
            vault_user_account: args.vault_user_account.key(),
            token_program: args.token_program.key(),
            rent: args.rent.key(),
            vault_pda: args.vault_pda.key(),
            system_program: args.system_program.key(),
            user_farm_owner: args.user_farm_owner.key(),
            user_transfer_authority: args.user_transfer_authority.key(),
            user_base_token_account: args.user_base_token_account.key(),
            user_farm_token_account: args.user_farm_token_account.key(),
            user_reward_token_account: args.user_reward_token_account.key(),
            global_base_token_vault: args.global_base_token_vault.key(),
            farm_token_mint: args.farm_token_mint.key(),
            global_farm: args.global_farm.key(),
            orca_user_farm: args.orca_user_farm.key(),
            global_reward_token_vault: args.global_reward_token_vault.key(),
            convert_authority: args.convert_authority.key(),
            aqua_farm_program: args.aqua_farm_program.key(),
            receiving_token_account: args.receiving_token_account.key(),
            clock: args.clock.key(),
            leveraged_user_farm: args.leveraged_user_farm.key(),
            leveraged_farm: args.leveraged_farm.key(),
            solfarm_vault_program: args.solfarm_vault_program.key(),
            obligation_vault_address: args.obligation_vault_address.key(),
        }
    }
}

#[allow(unused)]
pub fn withdraw_orca_vault<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, WithdrawOrcaFarm<'info>>,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8,
) -> Result<()> {
    let ix = tulip_withdraw_orca_vault_close(
        Box::new((&ctx.accounts).into()),
        ctx.accounts.lending_market_account.key(),
        ctx.accounts.user_farm_obligation.key(),
        ctx.accounts.lending_market_authority.key(),
        ctx.accounts.lending_program.key(),
        obligation_index,
        withdraw_percent,
        close_method,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.accounts.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
