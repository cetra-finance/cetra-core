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
pub struct DepositChamber<'info> {
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

    pub clock_sysvar: Sysvar<'info, Clock>,
    pub rent_sysvar: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}

impl<'c, 'info> DepositChamber<'info> {
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
                let lending_market_authority = &remaining_accounts[8];
                let lending_program = &remaining_accounts[9];
                let coin_source_reserve_liquidity_token_account = &remaining_accounts[10];
                let pc_source_reserve_liquidity_token_account = &remaining_accounts[11];
                let coin_reserve_liquidity_fee_receiver = &remaining_accounts[12];
                let pc_reserve_liquidity_fee_receiver = &remaining_accounts[13];
                let borrow_authorizer = &remaining_accounts[14];
                let lp_pyth_price_account = &remaining_accounts[15];
                let vault_account = &remaining_accounts[16];
                let position_info_account = &remaining_accounts[17];
                let vault_signer = &remaining_accounts[18];
                let swap_or_liquidity_program_id = &remaining_accounts[19];
                let amm_id = &remaining_accounts[20];
                let amm_authority = &remaining_accounts[21];
                let amm_open_orders = &remaining_accounts[22];
                let amm_quantities_or_target_orders = &remaining_accounts[23];
                let pool_coin_tokenaccount = &remaining_accounts[24];
                let pool_pc_tokenaccount = &remaining_accounts[25];
                let serum_program_id = &remaining_accounts[26];
                let serum_market = &remaining_accounts[27];
                let serum_bids = &remaining_accounts[28];
                let serum_asks = &remaining_accounts[29];
                let serum_event_queue = &remaining_accounts[30];
                let serum_coin_vault_account = &remaining_accounts[31];
                let serum_pc_vault_account = &remaining_accounts[32];
                let serum_vault_signer = &remaining_accounts[33];
                let coin_wallet = &remaining_accounts[34];
                let pc_wallet = &remaining_accounts[35];
                let liquidity_program_id = &remaining_accounts[36];
                let lp_mint_address = &remaining_accounts[37];
                let lev_farm_coin_token_account = &remaining_accounts[38];
                let lev_farm_pc_token_account = &remaining_accounts[39];
                let user_lp_token_account = &remaining_accounts[40];
                let pyth_price_account = &remaining_accounts[41];
                let dex_program = &remaining_accounts[42];
                let obligation_vault_address = &remaining_accounts[43];
                let vault_program = &remaining_accounts[44];
                let authority_token_account = &remaining_accounts[45];
                let vault_pda_account = &remaining_accounts[46];
                let vault = &remaining_accounts[47];
                let lp_token_account = &remaining_accounts[48];
                let user_balance_account = &remaining_accounts[49];
                let stake_program_id = &remaining_accounts[50];
                let pool_id = &remaining_accounts[51];
                let pool_authority = &remaining_accounts[52];
                let vault_info_account = &remaining_accounts[53];
                let pool_lp_token_account = &remaining_accounts[54];
                let user_reward_a_token_account = &remaining_accounts[55];
                let pool_reward_a_token_account = &remaining_accounts[56];
                let user_reward_b_token_account = &remaining_accounts[57];
                let pool_reward_b_token_account = &remaining_accounts[58];
                let user_balance_metadata = &remaining_accounts[59];

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

                // 12. Deposit and borrow tokens with leverage
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
                            derived_lending_market_authority: lending_market_authority
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

                // 13. Calculate user shares
                let shares_rate = Rate::try_from(
                    Decimal::from(self.chamber_shares_mint.supply)
                        .try_div(Decimal::from(chamber_total_liquidity))?,
                )?;
                let user_shares = user_total_value.try_mul(shares_rate)?.try_floor_u64()?;

                // 14. Swap tokens via AMM
                cpi::tulip::leveraged::raydium::swap_tokens_raydium_stats(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::raydium::RaydiumSwap {
                            authority: self.chamber_authority.to_account_info(),
                            leveraged_farm: leveraged_farm.to_account_info(),
                            user_farm: user_farm.to_account_info(),
                            user_farm_obligation: user_farm_obligation.to_account_info(),
                            token_program: self.token_program.clone(),
                            vault_signer: vault_signer.to_account_info(),
                            swap_or_liquidity_program_id: swap_or_liquidity_program_id
                                .to_account_info(),
                            amm_id: amm_id.to_account_info(),
                            amm_authority: amm_authority.to_account_info(),
                            amm_open_orders: amm_open_orders.to_account_info(),
                            amm_quantities_or_target_orders: amm_quantities_or_target_orders
                                .to_account_info(),
                            pool_coin_tokenaccount: pool_coin_tokenaccount.to_account_info(),
                            pool_pc_tokenaccount: pool_pc_tokenaccount.to_account_info(),
                            serum_program_id: serum_program_id.to_account_info(),
                            serum_market: serum_market.to_account_info(),
                            serum_bids: serum_bids.to_account_info(),
                            serum_asks: serum_asks.to_account_info(),
                            serum_event_queue: serum_event_queue.to_account_info(),
                            serum_coin_vault_account: serum_coin_vault_account.to_account_info(),
                            serum_pc_vault_account: serum_pc_vault_account.to_account_info(),
                            serum_vault_signer: serum_vault_signer.to_account_info(),
                            coin_wallet: coin_wallet.to_account_info(),
                            pc_wallet: pc_wallet.to_account_info(),
                            lending_market_account: lending_market_account.to_account_info(),
                            lending_market_authority: lending_market_authority.to_account_info(),
                            lending_program: lending_program.to_account_info(),
                            position_info_account: position_info_account.to_account_info(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    0,
                )?;

                // 15. Deposit tokens into lp
                cpi::tulip::leveraged::add_liquidity_stats(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::AddLiquidity {
                            authority: self.chamber_authority.to_account_info(),
                            user_farm: user_farm.to_account_info(),
                            leveraged_farm: leveraged_farm.to_account_info(),
                            liquidity_program_id: liquidity_program_id.to_account_info(),
                            amm_id: amm_id.to_account_info(),
                            amm_authority: amm_authority.to_account_info(),
                            amm_open_orders: amm_open_orders.to_account_info(),
                            amm_quantities_or_target_orders: amm_quantities_or_target_orders
                                .to_account_info(),
                            lp_mint_address: lp_mint_address.to_account_info(),
                            pool_coin_token_account: pool_coin_tokenaccount.to_account_info(),
                            pool_pc_token_account: pool_pc_tokenaccount.to_account_info(),
                            serum_market: serum_market.to_account_info(),
                            token_program: self.token_program.clone(),
                            lev_farm_coin_token_account: lev_farm_coin_token_account
                                .to_account_info(),
                            lev_farm_pc_token_account: lev_farm_pc_token_account.to_account_info(),
                            user_lp_token_account: user_lp_token_account.to_account_info(),
                            pyth_price_account: pyth_price_account.to_account_info(),
                            lending_market_account: lending_market_account.to_account_info(),
                            user_farm_obligation: user_farm_obligation.to_account_info(),
                            derived_lending_market_authority: lending_market_authority
                                .to_account_info(),
                            lending_program: lending_program.to_account_info(),
                            clock: self.clock_sysvar.clone(),
                            dex_program: dex_program.to_account_info(),
                            position_info_account: position_info_account.to_account_info(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    0,
                )?;

                // 16. Deposit lp tokens into tulip vault
                cpi::tulip::leveraged::raydium::deposit_raydium_vault(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::raydium::DepositFarm {
                            authority: self.chamber_authority.to_account_info(),
                            user_farm: user_farm.to_account_info(),
                            obligation_vault_address: obligation_vault_address.to_account_info(),
                            leveraged_farm: leveraged_farm.to_account_info(),
                            vault_program: vault_program.to_account_info(),
                            authority_token_account: authority_token_account.to_account_info(),
                            vault_pda_account: vault_pda_account.to_account_info(),
                            vault: vault.to_account_info(),
                            lp_token_account: lp_token_account.to_account_info(),
                            user_balance_account: user_balance_account.to_account_info(),
                            system_program: self.system_program.clone(),
                            stake_program_id: stake_program_id.to_account_info(),
                            pool_id: pool_id.to_account_info(),
                            pool_authority: pool_authority.to_account_info(),
                            vault_info_account: vault_info_account.to_account_info(),
                            pool_lp_token_account: pool_lp_token_account.to_account_info(),
                            user_reward_a_token_account: user_reward_a_token_account
                                .to_account_info(),
                            pool_reward_a_token_account: pool_reward_a_token_account
                                .to_account_info(),
                            user_reward_b_token_account: user_reward_b_token_account
                                .to_account_info(),
                            pool_reward_b_token_account: pool_reward_b_token_account
                                .to_account_info(),
                            clock: self.clock_sysvar.clone(),
                            rent: self.rent_sysvar.clone(),
                            token_program_id: self.token_program.clone(),
                            user_balance_metadata: user_balance_metadata.to_account_info(),
                            lending_market_account: lending_market_account.to_account_info(),
                            user_farm_obligation: user_farm_obligation.to_account_info(),
                            lending_market_authority: lending_market_authority.to_account_info(),
                            lending_program: lending_program.to_account_info(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    0,
                    0,
                    0,
                )?;

                // 17. Mint shares to user
                token::mint_to(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        token::MintTo {
                            mint: self.chamber_shares_mint.to_account_info(),
                            to: self.user_shares.to_account_info(),
                            authority: self.chamber_authority.to_account_info(),
                        },
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    user_shares,
                )?;

                // 18. Update `Chamber` state
                self.chamber.vault.deposit(base_amount, quote_amount)?;
            }
        };

        Ok(())
    }
}
