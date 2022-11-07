use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::create_user_farm_obligation::{
    create_user_farm_obligation as tulip_create_user_farm_obligation,
    CreateUserFarmObligation as TulipCreateUserFarmObligation,
};

#[derive(Accounts)]
pub struct CreateUserFarmObligation<'info> {
    pub authority: AccountInfo<'info>,
    pub user_farm: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub user_farm_obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub obligation_vault_address: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub lending_program: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> From<&CreateUserFarmObligation<'info>> for TulipCreateUserFarmObligation {
    fn from(args: &CreateUserFarmObligation<'info>) -> TulipCreateUserFarmObligation {
        TulipCreateUserFarmObligation {
            authority: args.authority.key(),
            user_farm: args.user_farm.key(),
            leveraged_farm: args.leveraged_farm.key(),
            user_farm_obligation: args.user_farm_obligation.key(),
            lending_market: args.lending_market.key(),
            obligation_vault_address: args.obligation_vault_address.key(),
            clock: args.clock.key(),
            rent: args.rent.key(),
            lending_program: args.lending_program.key(),
            token_program: args.token_program.key(),
            system_program: args.system_program.key(),
        }
    }
}

#[allow(unused)]
pub fn create_user_farm_obligation<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<CreateUserFarmObligation<'info>>>,
) -> Result<()> {
    let ix = tulip_create_user_farm_obligation(ctx.accounts.as_ref().into())
        .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.accounts.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
