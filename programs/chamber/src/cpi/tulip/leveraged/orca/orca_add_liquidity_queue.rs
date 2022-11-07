use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::orca_add_liquidity_queue::{
    orca_add_liquidity_queue as tulip_orca_add_liquidity_queue,
    OrcaAddLiquidityQueue as TulipOrcaAddLiquidityQueue,
};

#[derive(Accounts)]
pub struct OrcaAddLiquidityQueue<'info> {
    pub authority: AccountInfo<'info>,
    pub user_farm: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub vault_account: AccountInfo<'info>,
    pub vault_user_account: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
    pub vault_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub lev_farm_coin_token_account: AccountInfo<'info>,
    pub lev_farm_pc_token_account: AccountInfo<'info>,
    pub pool_coin_token_account: AccountInfo<'info>,
    pub pool_pc_token_account: AccountInfo<'info>,
    pub liquidity_program_id: AccountInfo<'info>,
    pub amm_id: AccountInfo<'info>,
    pub amm_authority: AccountInfo<'info>,
    pub vault_deposit_queue: AccountInfo<'info>,
    pub lp_mint_address: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub user_farm_obligation: AccountInfo<'info>,
    pub derived_lending_market_authority: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    pub solfarm_vault_program: AccountInfo<'info>,
    pub obligation_vault_address: AccountInfo<'info>,
    pub position_info_account: AccountInfo<'info>,
}

impl<'info> From<&OrcaAddLiquidityQueue<'info>> for TulipOrcaAddLiquidityQueue {
    fn from(args: &OrcaAddLiquidityQueue<'info>) -> TulipOrcaAddLiquidityQueue {
        TulipOrcaAddLiquidityQueue {
            authority: args.authority.key(),
            user_farm: args.user_farm.key(),
            leveraged_farm: args.leveraged_farm.key(),
            vault_account: args.vault_account.key(),
            vault_user_account: args.vault_user_account.key(),
            token_program: args.token_program.key(),
            rent: args.rent.key(),
            vault_pda: args.vault_pda.key(),
            system_program: args.system_program.key(),
            lev_farm_coin_token_account: args.lev_farm_coin_token_account.key(),
            lev_farm_pc_token_account: args.lev_farm_pc_token_account.key(),
            pool_coin_token_account: args.pool_coin_token_account.key(),
            pool_pc_token_account: args.pool_pc_token_account.key(),
            liquidity_program_id: args.liquidity_program_id.key(),
            amm_id: args.amm_id.key(),
            amm_authority: args.amm_authority.key(),
            vault_deposit_queue: args.vault_deposit_queue.key(),
            lp_mint_address: args.lp_mint_address.key(),
            lending_market_account: args.lending_market_account.key(),
            user_farm_obligation: args.user_farm_obligation.key(),
            derived_lending_market_authority: args.derived_lending_market_authority.key(),
            lending_program: args.lending_program.key(),
            dex_program: args.dex_program.key(),
            solfarm_vault_program: args.solfarm_vault_program.key(),
            obligation_vault_address: args.obligation_vault_address.key(),
        }
    }
}

#[allow(unused)]
pub fn orca_add_liquidity_queue<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, OrcaAddLiquidityQueue<'info>>,
    account_nonce: u8,
    obligation_index: u8,
) -> Result<()> {
    let ix = tulip_orca_add_liquidity_queue(
        Box::new((&ctx.accounts).into()),
        ctx.accounts.position_info_account.key(),
        account_nonce,
        obligation_index,
    )
    .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.accounts.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
