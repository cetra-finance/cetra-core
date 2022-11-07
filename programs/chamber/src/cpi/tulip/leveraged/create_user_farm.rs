use crate::error::ChamberError;
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token;
use tulipv2_sdk_levfarm::instructions::create_user_farm::{
    create_user_farm as tulip_create_user_farm, CreateUserFarm as TulipCreateUserFarm,
};

#[derive(Accounts)]
pub struct CreateUserFarm<'info> {
    pub authority: AccountInfo<'info>,
    pub user_farm: AccountInfo<'info>,
    pub user_farm_obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub global: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub lending_program: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
    pub obligation_vault_address: AccountInfo<'info>,
}

impl<'info> From<&CreateUserFarm<'info>> for TulipCreateUserFarm {
    fn from(args: &CreateUserFarm<'info>) -> TulipCreateUserFarm {
        TulipCreateUserFarm {
            authority: args.authority.key(),
            user_farm: args.user_farm.key(),
            user_farm_obligation: args.user_farm_obligation.key(),
            lending_market: args.lending_market.key(),
            global: args.global.key(),
            leveraged_farm: args.leveraged_farm.key(),
            clock: args.clock.key(),
            rent: args.rent.key(),
            system_program: args.system_program.key(),
            lending_program: args.lending_program.key(),
            token_program: args.token_program.key(),
            obligation_vault_address: args.obligation_vault_address.key(),
        }
    }
}

#[allow(unused)]
pub fn create_user_farm<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Box<CreateUserFarm<'info>>>,
    solfarm_vault_program: &Pubkey,
) -> Result<()> {
    let ix = tulip_create_user_farm(ctx.accounts.as_ref().into(), *solfarm_vault_program)
        .ok_or(ChamberError::CpiInstructionFormationFailed)?;

    solana_program::program::invoke_signed(&ix, &ctx.accounts.to_account_infos(), ctx.signer_seeds)
        .map_err(Into::into)
}
