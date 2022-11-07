use crate::{cpi, error, state, utils};
use anchor_lang::prelude::*;
use anchor_spl::token;
use tulipv2_sdk_common::math::{
    common::{TryAdd, TryDiv, TryMul},
    decimal::Decimal,
    rate::Rate,
};

#[derive(Accounts)]
#[instruction(base_amount: u64, quote_amount: u64)]
pub struct BeginDepositChamber<'info> {
    pub chamber: Box<Account<'info, state::Chamber>>,

    #[account(
        mut,
        seeds = [
            utils::USER_ACCOUNT_PREFIX.as_bytes(),
            chamber.key().as_ref(),
            user.key().as_ref(),
        ],
        bump,
        constraint = user_account.chamber == chamber.key(),
        constraint = user_account.user == user.key(),
    )]
    pub user_account: Box<Account<'info, state::UserAccount>>,

    #[account(constraint = user_shares.mint == chamber_shares_mint.key())]
    pub user_shares: Box<Account<'info, token::TokenAccount>>,

    pub user_base_token: Box<Account<'info, token::TokenAccount>>,
    pub user_quote_token: Box<Account<'info, token::TokenAccount>>,

    #[account(constraint = chamber_shares_mint.key() == chamber.config.shares_mint)]
    pub chamber_shares_mint: Box<Account<'info, token::Mint>>,

    #[account(mut, constraint = chamber_base_token.key() == chamber.vault.base)]
    pub chamber_base_token: Box<Account<'info, token::TokenAccount>>,

    #[account(mut, constraint = chamber_quote_token.key() == chamber.vault.quote)]
    pub chamber_quote_token: Box<Account<'info, token::TokenAccount>>,

    /// CHECK: Pyth oracle for tracking base token price.
    #[account(constraint = chamber_base_oracle.key() == chamber.vault.base_oracle)]
    pub chamber_base_oracle: UncheckedAccount<'info>,

    /// CHECK: Pyth oracle for tracking quote token price.
    #[account(constraint = chamber_quote_oracle.key() == chamber.vault.quote_oracle)]
    pub chamber_quote_oracle: UncheckedAccount<'info>,

    /// CHECK: Chamber authority PDA.
    #[account(
        seeds = [
            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
            chamber.key().as_ref(),
        ],
        bump,
        constraint = chamber_authority.key() == chamber.config.authority
    )]
    pub chamber_authority: UncheckedAccount<'info>,

    /// CHECK: Program for `farm`.
    #[account(constraint = chamber_farm_program.key() == chamber.strategy.farm_program)]
    pub chamber_farm_program: UncheckedAccount<'info>,

    pub user: Signer<'info>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}

impl<'c, 'info> BeginDepositChamber<'info> {
    pub fn process(
        &mut self,
        remaining_accounts: &'c [AccountInfo<'info>],
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<()> {
        // 1. Deposit base token into `Chamber`
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                token::Transfer {
                    from: self.user_base_token.to_account_info(),
                    to: self.chamber_base_token.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
            base_amount,
        )?;

        // 2. Deposit quote token into `Chamber`
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                token::Transfer {
                    from: self.user_quote_token.to_account_info(),
                    to: self.chamber_quote_token.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
            quote_amount,
        )?;

        // 3. Ensure, that `UserAccount` in correct state
        self.user_account
            .assert_status(state::UserAccountStatus::Ready)?;

        // 4. Ensure, that `User` is depositing something valuable
        if base_amount == 0 && quote_amount == 0 {
            return Err(error::ChamberError::InsufficientFunds.into());
        }

        // 5. Process market specific logic
        match self.chamber.strategy.market {
            state::ChamberMarket::Tulip => {
                let user_farm = &remaining_accounts[0];
                let leveraged_farm = &remaining_accounts[1];
                let user_farm_obligation = &remaining_accounts[2];
                let coin_destination_token_account = &remaining_accounts[3];
                let pc_destination_token_account = &remaining_accounts[4];
                let coin_deposit_reserve_account = &remaining_accounts[5];
                let pc_deposit_reserve_account = &remaining_accounts[6];
                let lending_market_account = &remaining_accounts[7];
                let derived_lending_market_authority = &remaining_accounts[8];
                let lending_program = &remaining_accounts[9];
                let coin_source_reserve_liquidity_token_account = &remaining_accounts[10];
                let pc_source_reserve_liquidity_token_account = &remaining_accounts[11];
                let coin_reserve_liquidity_fee_receiver = &remaining_accounts[12];
                let pc_reserve_liquidity_fee_receiver = &remaining_accounts[13];
                let borrow_authorizer = &remaining_accounts[14];
                let lp_pyth_price_account = &remaining_accounts[15];
                let vault_account = &remaining_accounts[16];
                let position_info_account = &remaining_accounts[17];

                // 6. Get base token price and decimals
                let base_price = tulipv2_sdk_common::pyth::load_pyth_price(
                    &self.chamber_base_oracle.data.as_ref().borrow(),
                )?;

                // 7. Calculate user base token value in `base_price`
                let user_base_value = base_price
                    .try_mul(base_amount)?
                    .try_div(self.chamber.vault.base_decimals)?;

                // 8. Get quote token price and decimals
                let quote_price = tulipv2_sdk_common::pyth::load_pyth_price(
                    &self.chamber_quote_oracle.data.as_ref().borrow(),
                )?;

                // 9. Calculate user quote token value in `quote_price`
                let user_quote_value = quote_price
                    .try_mul(quote_amount)?
                    .try_div(self.chamber.vault.quote_decimals)?;

                // 10. Calculate total user deposit value
                let user_total_value = user_base_value.try_add(user_quote_value)?;

                // 11. Calculate base and quote borrow amount
                let (user_base_borrow_amount, user_quote_borrow_amount) = {
                    let (volatile_price, underlying_price) =
                        if self.chamber.strategy.is_base_volatile {
                            (base_price, quote_price)
                        } else {
                            (quote_price, base_price)
                        };

                    let volatile_borrow_amount = user_total_value
                        .try_mul(3)?
                        .try_div(4)?
                        .try_mul(self.chamber.strategy.leverage - 1)?
                        .try_div(volatile_price)?
                        .try_floor_u64()?;

                    let underlying_borrow_amount = user_total_value
                        .try_mul(1)?
                        .try_div(4)?
                        .try_mul(self.chamber.strategy.leverage - 1)?
                        .try_div(underlying_price)?
                        .try_floor_u64()?;

                    if self.chamber.strategy.is_base_volatile {
                        (volatile_borrow_amount, underlying_borrow_amount)
                    } else {
                        (underlying_borrow_amount, volatile_borrow_amount)
                    }
                };

                // 11. Deposit and borrow tokens with leverage
                cpi::tulip::leveraged::deposit_borrow_dual(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::DepositBorrowDual {
                            authority: self.chamber_authority.to_account_info(),
                            user_farm: user_farm.to_account_info(),
                            leveraged_farm: leveraged_farm.to_account_info(),
                            user_farm_obligation: user_farm_obligation.to_account_info(),
                            coin_source_token_account: self.chamber_base_token.to_account_info(),
                            coin_destination_token_account: coin_destination_token_account
                                .to_account_info(),
                            pc_source_token_account: self.chamber_quote_token.to_account_info(),
                            pc_destination_token_account: pc_destination_token_account
                                .to_account_info(),
                            coin_deposit_reserve_account: coin_deposit_reserve_account
                                .to_account_info(),
                            pc_deposit_reserve_account: pc_deposit_reserve_account
                                .to_account_info(),
                            coin_reserve_liquidity_oracle: self
                                .chamber_base_oracle
                                .to_account_info()
                                .to_account_info(),
                            pc_reserve_liquidity_oracle: self
                                .chamber_quote_oracle
                                .to_account_info()
                                .to_account_info(),
                            lending_market_account: lending_market_account.to_account_info(),
                            derived_lending_market_authority: derived_lending_market_authority
                                .to_account_info(),
                            lending_program: lending_program.to_account_info(),
                            coin_source_reserve_liquidity_token_account:
                                coin_source_reserve_liquidity_token_account.to_account_info(),
                            pc_source_reserve_liquidity_token_account:
                                pc_source_reserve_liquidity_token_account.to_account_info(),
                            coin_reserve_liquidity_fee_receiver:
                                coin_reserve_liquidity_fee_receiver.to_account_info(),
                            pc_reserve_liquidity_fee_receiver: pc_reserve_liquidity_fee_receiver
                                .to_account_info(),
                            borrow_authorizer: borrow_authorizer.to_account_info(),
                            lp_pyth_price_account: lp_pyth_price_account.to_account_info(),
                            vault_account: vault_account.to_account_info(),
                            position_info_account: position_info_account.to_account_info(),
                            rent: self.rent_sysvar.clone(),
                            token_program: self.token_program.clone(),
                            system_program: self.system_program.clone(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    base_amount,
                    quote_amount,
                    user_base_borrow_amount,
                    user_quote_borrow_amount,
                    0,
                )?;

                // TODO: Obtain total liquidity from underlying position
                let chamber_total_liquidity = self
                    .chamber
                    .vault
                    .get_total_value(&base_price, &quote_price)?;

                // 12. Calculate user shares
                let shares_rate = Rate::try_from(
                    Decimal::from(self.chamber_shares_mint.supply)
                        .try_div(Decimal::from(chamber_total_liquidity))?,
                )?;
                let user_shares = user_total_value.try_mul(shares_rate)?.try_floor_u64()?;

                // 13. Update `UserAccount` status, lock provided tokens
                // amount for next deposit stages
                self.user_account
                    .begin_deposit(base_amount, quote_amount, user_shares);
            }
        };

        Ok(())
    }
}
