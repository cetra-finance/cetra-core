use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::deposit_raydium_vault::{
    deposit_vault as tulip_deposit_vault, DepositFarm as TulipDepositFarm,
};

#[derive(Accounts)]
pub struct DepositFarm<'info> {
    pub authority: AccountInfo<'info>,
    pub user_farm: AccountInfo<'info>,
    pub obligation_vault_address: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub vault_program: AccountInfo<'info>,
    pub authority_token_account: AccountInfo<'info>,
    pub vault_pda_account: AccountInfo<'info>,
    pub vault: AccountInfo<'info>,
    pub lp_token_account: AccountInfo<'info>,
    pub user_balance_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub stake_program_id: AccountInfo<'info>,
    pub pool_id: AccountInfo<'info>,
    pub pool_authority: AccountInfo<'info>,
    pub vault_info_account: AccountInfo<'info>,
    pub pool_lp_token_account: AccountInfo<'info>,
    pub user_reward_a_token_account: AccountInfo<'info>,
    pub pool_reward_a_token_account: AccountInfo<'info>,
    pub user_reward_b_token_account: AccountInfo<'info>,
    pub pool_reward_b_token_account: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program_id: Program<'info, token::Token>,
    pub user_balance_metadata: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub user_farm_obligation: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
}

impl<'info> From<&DepositFarm<'info>> for TulipDepositFarm {
    fn from(args: &DepositFarm<'info>) -> TulipDepositFarm {
        TulipDepositFarm {
            authority: args.authority.key(),
            user_farm: args.user_farm.key(),
            obligation_vault_address: args.obligation_vault_address.key(),
            leveraged_farm: args.leveraged_farm.key(),
            vault_program: args.vault_program.key(),
            authority_token_account: args.authority_token_account.key(),
            vault_pda_account: args.vault_pda_account.key(),
            vault: args.vault.key(),
            lp_token_account: args.lp_token_account.key(),
            user_balance_account: args.user_balance_account.key(),
            system_program: args.system_program.key(),
            stake_program_id: args.stake_program_id.key(),
            pool_id: args.pool_id.key(),
            pool_authority: args.pool_authority.key(),
            vault_info_account: args.vault_info_account.key(),
            pool_lp_token_account: args.pool_lp_token_account.key(),
            user_reward_a_token_account: args.user_reward_a_token_account.key(),
            pool_reward_a_token_account: args.pool_reward_a_token_account.key(),
            user_reward_b_token_account: args.user_reward_b_token_account.key(),
            pool_reward_b_token_account: args.pool_reward_b_token_account.key(),
            clock: args.clock.key(),
            rent: args.rent.key(),
            token_program_id: args.token_program_id.key(),
            user_balance_metadata: args.user_balance_metadata.key(),
        }
    }
}

#[allow(unused)]
pub fn deposit_raydium_vault<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<DepositFarm<'info>>>,
    nonce: u8,
    meta_nonce: u8,
    obligation_index: u64,
) -> Result<()> {
    let ix = tulip_deposit_vault(
        Box::new(ctx.accounts.as_ref().into()),
        ctx.accounts.lending_market_account.key(),
        ctx.accounts.user_farm_obligation.key(),
        ctx.accounts.lending_market_authority.key(),
        ctx.accounts.lending_program.key(),
        nonce,
        meta_nonce,
        obligation_index,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.accounts.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
