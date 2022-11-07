use crate::{cpi, error, state, utils};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::{associated_token, token};

#[derive(Accounts)]
#[instruction(market: state::ChamberMarket, leverage: u64, is_base_volatile: bool, chamber_nonce: u8, authority_bump: u8)]
pub struct InitializeChamber<'info> {
    #[account(
        init,
        payer = payer,
        space = state::Chamber::LEN,
        seeds = [
            utils::CHAMBER_PREFIX.as_bytes(),
            farm.key().as_ref(),
            base_mint.key().as_ref(),
            quote_mint.key().as_ref(),
            chamber_nonce.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub chamber: Box<Account<'info, state::Chamber>>,

    /// CHECK: Associated `market` farm.
    pub farm: UncheckedAccount<'info>,

    /// CHECK: Uninitialized associated base token account for `base_mint`.
    #[account(mut)]
    pub base_token: UncheckedAccount<'info>,

    /// CHECK: Uninitialized associated quote token account for `quote_mint`.
    #[account(mut)]
    pub quote_token: UncheckedAccount<'info>,

    pub base_mint: Box<Account<'info, token::Mint>>,
    pub quote_mint: Box<Account<'info, token::Mint>>,

    /// CHECK: Uninitialized shares mint.
    #[account(mut)]
    pub shares_mint: UncheckedAccount<'info>,

    /// CHECK: Pyth oracle for tracking base token price.
    pub base_oracle: UncheckedAccount<'info>,

    /// CHECK: Pyth oracle for tracking quote token price.
    pub quote_oracle: UncheckedAccount<'info>,

    /// CHECK: Chamber authority.
    #[account(mut, seeds = [utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(), chamber.key().as_ref()], bump = authority_bump)]
    pub authority: UncheckedAccount<'info>,

    /// CHECK: Chamber fee manager(receiver).
    pub fee_manager: UncheckedAccount<'info>,

    /// CHECK: Program for `farm`.
    pub farm_program: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub owner: Signer<'info>,

    pub clock_sysvar: Sysvar<'info, Clock>,
    pub rent_sysvar: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'c, 'info> InitializeChamber<'info> {
    pub fn process(
        &mut self,
        remaining_accounts: &'c [AccountInfo<'info>],
        market: state::ChamberMarket,
        leverage: u64,
        is_base_volatile: bool,
        chamber_nonce: u8,
        authority_bump: u8,
    ) -> Result<()> {
        // 1. Create base token ata
        associated_token::create(CpiContext::new(
            self.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: self.payer.to_account_info(),
                associated_token: self.base_token.to_account_info(),
                authority: self.authority.to_account_info(),
                mint: self.base_mint.to_account_info(),
                rent: self.rent_sysvar.to_account_info(),
                token_program: self.token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        ))?;

        // 2. Create quote token ata
        associated_token::create(CpiContext::new(
            self.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: self.payer.to_account_info(),
                associated_token: self.quote_token.to_account_info(),
                authority: self.authority.to_account_info(),
                mint: self.quote_mint.to_account_info(),
                rent: self.rent_sysvar.to_account_info(),
                token_program: self.token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        ))?;

        // 3. Create shares token mint
        token::initialize_mint(
            CpiContext::new(
                self.token_program.to_account_info(),
                token::InitializeMint {
                    mint: self.shares_mint.to_account_info(),
                    rent: self.rent_sysvar.to_account_info(),
                },
            ),
            utils::SHARES_DECIMALS,
            self.authority.key,
            None,
        )?;

        // 4. Fund authority account
        system_program::transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                system_program::Transfer {
                    from: self.payer.to_account_info(),
                    to: self.authority.to_account_info(),
                },
            ),
            self.rent_sysvar
                .minimum_balance(tulipv2_sdk_levfarm::accounts::USER_FARM_ACCOUNT_SIZE)
                * 2
                + self
                    .rent_sysvar
                    .minimum_balance(tulipv2_sdk_levfarm::accounts::POSITION_INFO_ACCOUNT_SIZE)
                    * 2
                + self
                    .rent_sysvar
                    .minimum_balance(tulipv2_sdk_levfarm::accounts::LEVERAGED_FARM_ACCOUNT_SIZE)
                    * 2,
        )?;

        // 5. Initialize `market` related accounts
        match market {
            state::ChamberMarket::Tulip => {
                // TODO: Check first & second obligation relations
                let user_farm = &remaining_accounts[0];
                let user_farm_obligation = &remaining_accounts[1];
                let user_farm_obligation_1 = &remaining_accounts[2];
                let lending_market = &remaining_accounts[3];
                let obligation_vault_address = &remaining_accounts[4];
                let obligation_vault_address_1 = &remaining_accounts[5];
                let global = &remaining_accounts[6];
                let lending_program = &remaining_accounts[7];
                let solfarm_vault_program = &remaining_accounts[8];

                cpi::tulip::leveraged::create_user_farm(
                    CpiContext::new_with_signer(
                        self.farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::CreateUserFarm {
                            authority: self.authority.to_account_info(),
                            user_farm: user_farm.to_account_info(),
                            user_farm_obligation: user_farm_obligation.to_account_info(),
                            lending_market: lending_market.to_account_info(),
                            global: global.to_account_info(),
                            leveraged_farm: self.farm.to_account_info(),
                            clock: self.clock_sysvar.clone(),
                            rent: self.rent_sysvar.clone(),
                            system_program: self.system_program.clone(),
                            lending_program: lending_program.to_account_info(),
                            token_program: self.token_program.clone(),
                            obligation_vault_address: obligation_vault_address.to_account_info(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[authority_bump],
                        ]],
                    ),
                    solfarm_vault_program.key,
                )?;

                cpi::tulip::leveraged::create_user_farm_obligation(CpiContext::new_with_signer(
                    self.farm_program.to_account_info(),
                    Box::new(cpi::tulip::leveraged::CreateUserFarmObligation {
                        authority: self.authority.to_account_info(),
                        user_farm: user_farm.to_account_info(),
                        leveraged_farm: self.farm.to_account_info(),
                        user_farm_obligation: user_farm_obligation_1.to_account_info(),
                        lending_market: lending_market.to_account_info(),
                        obligation_vault_address: obligation_vault_address_1.to_account_info(),
                        clock: self.clock_sysvar.clone(),
                        rent: self.rent_sysvar.clone(),
                        lending_program: lending_program.to_account_info(),
                        token_program: self.token_program.clone(),
                        system_program: self.system_program.clone(),
                    }),
                    &[&[
                        utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                        self.chamber.key().as_ref(),
                        &[authority_bump],
                    ]],
                ))?;
            }
        };

        // 6. Initialize `chamber` state
        self.chamber.init(
            &state::ChamberStrategy::new(
                market,
                self.farm.key,
                self.farm_program.key,
                leverage,
                is_base_volatile,
            ),
            &state::ChamberVault::new(
                self.base_token.key,
                self.quote_token.key,
                &self.base_mint.key(),
                &self.quote_mint.key(),
                self.base_oracle.key,
                self.quote_oracle.key,
                self.base_mint
                    .decimals
                    .try_into()
                    .map_err(|_| error::ChamberError::MathOverflow)?,
                self.quote_mint
                    .decimals
                    .try_into()
                    .map_err(|_| error::ChamberError::MathOverflow)?,
            ),
            &state::ChamberConfig::new(
                self.authority.key,
                self.owner.key,
                self.fee_manager.key,
                self.shares_mint.key,
                authority_bump,
                chamber_nonce,
            ),
        );

        Ok(())
    }
}
