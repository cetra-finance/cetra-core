use crate::{cpi, state, utils};
use anchor_lang::prelude::*;
use anchor_spl::token;
use tulipv2_sdk_common::math::common::{TryAdd, TryDiv, TryMul};

pub struct RebalanceChamberLookupTable<'a, 'info> {
    pub user_farm: &'a AccountInfo<'info>,
    pub obligation_vault_address: &'a AccountInfo<'info>,
    pub leveraged_farm: &'a AccountInfo<'info>,
    pub authority_token_account: &'a AccountInfo<'info>,
    pub vault: &'a AccountInfo<'info>,
    pub vault_program: &'a AccountInfo<'info>,
    pub user_balance_account: &'a AccountInfo<'info>,
    pub user_info_account: &'a AccountInfo<'info>,
    pub user_lp_token_account: &'a AccountInfo<'info>,
    pub user_reward_a_token_account: &'a AccountInfo<'info>,
    pub pool_reward_a_token_account: &'a AccountInfo<'info>,
    pub user_reward_b_token_account: &'a AccountInfo<'info>,
    pub pool_reward_b_token_account: &'a AccountInfo<'info>,
    pub vault_pda_account: &'a AccountInfo<'info>,
    pub pool_lp_token_account: &'a AccountInfo<'info>,
    pub pool_authority: &'a AccountInfo<'info>,
    pub pool_id: &'a AccountInfo<'info>,
    pub stake_program_id: &'a AccountInfo<'info>,
    pub user_balance_meta: &'a AccountInfo<'info>,
    pub lending_market_account: &'a AccountInfo<'info>,
    pub user_farm_obligation: &'a AccountInfo<'info>,
    pub lending_market_authority: &'a AccountInfo<'info>,
    pub lending_program: &'a AccountInfo<'info>,
    pub position_info_account: &'a AccountInfo<'info>,
    pub liquidity_program_id: &'a AccountInfo<'info>,
    pub amm_id: &'a AccountInfo<'info>,
    pub amm_authority: &'a AccountInfo<'info>,
    pub amm_open_orders: &'a AccountInfo<'info>,
    pub amm_quantities_or_target_orders: &'a AccountInfo<'info>,
    pub lp_mint_address: &'a AccountInfo<'info>,
    pub pool_coin_token_account: &'a AccountInfo<'info>,
    pub pool_pc_token_account: &'a AccountInfo<'info>,
    pub pool_withdraw_queue: &'a AccountInfo<'info>,
    pub pool_temp_lp_token_account: &'a AccountInfo<'info>,
    pub serum_program_id: &'a AccountInfo<'info>,
    pub serum_market: &'a AccountInfo<'info>,
    pub serum_coin_vault_account: &'a AccountInfo<'info>,
    pub serum_pc_vault_account: &'a AccountInfo<'info>,
    pub serum_vault_signer: &'a AccountInfo<'info>,
    pub lev_farm_coin_token_account: &'a AccountInfo<'info>,
    pub lev_farm_pc_token_account: &'a AccountInfo<'info>,
    pub user_obligation_account: &'a AccountInfo<'info>,
    pub vault_signer: &'a AccountInfo<'info>,
    pub swap_or_liquidity_program_id: &'a AccountInfo<'info>,
    pub serum_bids: &'a AccountInfo<'info>,
    pub serum_asks: &'a AccountInfo<'info>,
    pub serum_event_queue: &'a AccountInfo<'info>,
    pub coin_wallet: &'a AccountInfo<'info>,
    pub pc_wallet: &'a AccountInfo<'info>,
    pub asset_price_account: &'a AccountInfo<'info>,
    pub base_price_account: &'a AccountInfo<'info>,
    pub quote_price_account: &'a AccountInfo<'info>,
    pub asset_vault: &'a AccountInfo<'info>,
    pub first_reserve: &'a AccountInfo<'info>,
    pub first_reserve_price: &'a AccountInfo<'info>,
    pub second_reserve: &'a AccountInfo<'info>,
    pub second_reserve_price: &'a AccountInfo<'info>,
    pub coin_source_token_account: &'a AccountInfo<'info>,
    pub coin_destination_token_account: &'a AccountInfo<'info>,
    pub pc_source_token_account: &'a AccountInfo<'info>,
    pub pc_destination_token_account: &'a AccountInfo<'info>,
    pub coin_reserve_account: &'a AccountInfo<'info>,
    pub pc_reserve_account: &'a AccountInfo<'info>,
    pub lp_pyth_price_account: &'a AccountInfo<'info>,
    pub coin_deposit_reserve_account: &'a AccountInfo<'info>,
    pub pc_deposit_reserve_account: &'a AccountInfo<'info>,
    pub coin_source_reserve_liquidity_token_account: &'a AccountInfo<'info>,
    pub pc_source_reserve_liquidity_token_account: &'a AccountInfo<'info>,
    pub coin_reserve_liquidity_fee_receiver: &'a AccountInfo<'info>,
    pub pc_reserve_liquidity_fee_receiver: &'a AccountInfo<'info>,
    pub borrow_authorizer: &'a AccountInfo<'info>,
    pub vault_account: &'a AccountInfo<'info>,
    pub pool_coin_tokenaccount: &'a AccountInfo<'info>,
    pub pool_pc_tokenaccount: &'a AccountInfo<'info>,
    pub pyth_price_account: &'a AccountInfo<'info>,
    pub dex_program: &'a AccountInfo<'info>,
    pub lp_token_account: &'a AccountInfo<'info>,
    pub vault_info_account: &'a AccountInfo<'info>,
    pub user_balance_metadata: &'a AccountInfo<'info>,
}

impl<'a, 'info> RebalanceChamberLookupTable<'a, 'info> {
    pub fn from_remaining_accounts(remaining_accounts: &'a [AccountInfo<'info>]) -> Box<Self> {
        Box::new(RebalanceChamberLookupTable {
            user_farm: &remaining_accounts[0],
            obligation_vault_address: &remaining_accounts[1],
            leveraged_farm: &remaining_accounts[2],
            authority_token_account: &remaining_accounts[3],
            vault: &remaining_accounts[4],
            vault_program: &remaining_accounts[5],
            user_balance_account: &remaining_accounts[6],
            user_info_account: &remaining_accounts[7],
            user_lp_token_account: &remaining_accounts[8],
            user_reward_a_token_account: &remaining_accounts[9],
            pool_reward_a_token_account: &remaining_accounts[10],
            user_reward_b_token_account: &remaining_accounts[11],
            pool_reward_b_token_account: &remaining_accounts[12],
            vault_pda_account: &remaining_accounts[13],
            pool_lp_token_account: &remaining_accounts[14],
            pool_authority: &remaining_accounts[15],
            pool_id: &remaining_accounts[16],
            stake_program_id: &remaining_accounts[17],
            user_balance_meta: &remaining_accounts[18],
            lending_market_account: &remaining_accounts[19],
            user_farm_obligation: &remaining_accounts[20],
            lending_market_authority: &remaining_accounts[21],
            lending_program: &remaining_accounts[22],
            position_info_account: &remaining_accounts[23],
            liquidity_program_id: &remaining_accounts[24],
            amm_id: &remaining_accounts[25],
            amm_authority: &remaining_accounts[26],
            amm_open_orders: &remaining_accounts[27],
            amm_quantities_or_target_orders: &remaining_accounts[28],
            lp_mint_address: &remaining_accounts[29],
            pool_coin_token_account: &remaining_accounts[30],
            pool_pc_token_account: &remaining_accounts[31],
            pool_withdraw_queue: &remaining_accounts[32],
            pool_temp_lp_token_account: &remaining_accounts[33],
            serum_program_id: &remaining_accounts[34],
            serum_market: &remaining_accounts[35],
            serum_coin_vault_account: &remaining_accounts[36],
            serum_pc_vault_account: &remaining_accounts[37],
            serum_vault_signer: &remaining_accounts[38],
            lev_farm_coin_token_account: &remaining_accounts[39],
            lev_farm_pc_token_account: &remaining_accounts[40],
            user_obligation_account: &remaining_accounts[41],
            vault_signer: &remaining_accounts[42],
            swap_or_liquidity_program_id: &remaining_accounts[43],
            serum_bids: &remaining_accounts[44],
            serum_asks: &remaining_accounts[45],
            serum_event_queue: &remaining_accounts[46],
            coin_wallet: &remaining_accounts[47],
            pc_wallet: &remaining_accounts[48],
            asset_price_account: &remaining_accounts[49],
            base_price_account: &remaining_accounts[50],
            quote_price_account: &remaining_accounts[51],
            asset_vault: &remaining_accounts[52],
            first_reserve: &remaining_accounts[53],
            first_reserve_price: &remaining_accounts[54],
            second_reserve: &remaining_accounts[55],
            second_reserve_price: &remaining_accounts[56],
            coin_source_token_account: &remaining_accounts[57],
            coin_destination_token_account: &remaining_accounts[58],
            pc_source_token_account: &remaining_accounts[59],
            pc_destination_token_account: &remaining_accounts[60],
            coin_reserve_account: &remaining_accounts[61],
            pc_reserve_account: &remaining_accounts[62],
            lp_pyth_price_account: &remaining_accounts[63],
            coin_deposit_reserve_account: &remaining_accounts[64],
            pc_deposit_reserve_account: &remaining_accounts[65],
            coin_source_reserve_liquidity_token_account: &remaining_accounts[66],
            pc_source_reserve_liquidity_token_account: &remaining_accounts[67],
            coin_reserve_liquidity_fee_receiver: &remaining_accounts[68],
            pc_reserve_liquidity_fee_receiver: &remaining_accounts[69],
            borrow_authorizer: &remaining_accounts[70],
            vault_account: &remaining_accounts[71],
            pool_coin_tokenaccount: &remaining_accounts[72],
            pool_pc_tokenaccount: &remaining_accounts[73],
            pyth_price_account: &remaining_accounts[74],
            dex_program: &remaining_accounts[75],
            lp_token_account: &remaining_accounts[76],
            vault_info_account: &remaining_accounts[77],
            user_balance_metadata: &remaining_accounts[78],
        })
    }
}

#[derive(Accounts)]
pub struct RebalanceChamber<'info> {
    pub chamber: Box<Account<'info, state::Chamber>>,

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

impl<'c, 'info> RebalanceChamber<'info> {
    #[inline(always)]
    pub fn process(&mut self, remaining_accounts: &'c [AccountInfo<'info>]) -> Result<()> {
        // TODO: Create rebalance trigger logic

        // 1. Process market specific logic
        match self.chamber.strategy.market {
            state::ChamberMarket::Tulip => {
                let alt = RebalanceChamberLookupTable::from_remaining_accounts(remaining_accounts);

                // 2. Withdraw lp from vault
                cpi::tulip::leveraged::raydium::withdraw_raydium_vault_close(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::raydium::WithdrawFarm {
                            authority: self.chamber_authority.to_account_info(),
                            user_farm: alt.user_farm.clone(),
                            obligation_vault_address: alt.obligation_vault_address.clone(),
                            leveraged_farm: alt.leveraged_farm.clone(),
                            authority_token_account: alt.authority_token_account.clone(),
                            vault: alt.vault.clone(),
                            vault_program: alt.vault_program.clone(),
                            user_balance_account: alt.user_balance_account.clone(),
                            user_info_account: alt.user_info_account.clone(),
                            user_lp_token_account: alt.user_lp_token_account.clone(),
                            user_reward_a_token_account: alt.user_reward_a_token_account.clone(),
                            pool_reward_a_token_account: alt.pool_reward_a_token_account.clone(),
                            user_reward_b_token_account: alt.user_reward_b_token_account.clone(),
                            pool_reward_b_token_account: alt.pool_reward_b_token_account.clone(),
                            token_program_id: self.token_program.clone(),
                            clock: self.clock_sysvar.clone(),
                            vault_pda_account: alt.vault_pda_account.clone(),
                            pool_lp_token_account: alt.pool_lp_token_account.clone(),
                            pool_authority: alt.pool_authority.clone(),
                            pool_id: alt.pool_id.clone(),
                            stake_program_id: alt.stake_program_id.clone(),
                            user_balance_meta: alt.user_balance_meta.clone(),
                            lending_market_account: alt.lending_market_account.clone(),
                            user_farm_obligation: alt.user_farm_obligation.clone(),
                            lending_market_authority: alt.lending_market_authority.clone(),
                            lending_program: alt.lending_program.clone(),
                            position_info_account: alt.position_info_account.clone(),
                            system_program: self.system_program.clone(),
                            rent: self.rent_sysvar.clone(),
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
                    100,
                    0,
                )?;

                // 3. Remove liquidity from AMM for lp tokens
                cpi::tulip::leveraged::remove_liquidity_new(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::RemoveLiquidityNew {
                            user_farm: alt.user_farm.clone(),
                            obligation_vault_address: alt.obligation_vault_address.clone(),
                            leveraged_farm: alt.leveraged_farm.clone(),
                            liquidity_program_id: alt.liquidity_program_id.clone(),
                            amm_id: alt.amm_id.clone(),
                            amm_authority: alt.amm_authority.clone(),
                            amm_open_orders: alt.amm_open_orders.clone(),
                            amm_quantities_or_target_orders: alt
                                .amm_quantities_or_target_orders
                                .clone(),
                            lp_mint_address: alt.lp_mint_address.clone(),
                            pool_coin_token_account: alt.pool_coin_token_account.clone(),
                            pool_pc_token_account: alt.pool_pc_token_account.clone(),
                            pool_withdraw_queue: alt.pool_withdraw_queue.clone(),
                            pool_temp_lp_token_account: alt.pool_temp_lp_token_account.clone(),
                            serum_program_id: alt.serum_program_id.clone(),
                            serum_market: alt.serum_market.clone(),
                            serum_coin_vault_account: alt.serum_coin_vault_account.clone(),
                            serum_pc_vault_account: alt.serum_pc_vault_account.clone(),
                            serum_vault_signer: alt.serum_vault_signer.clone(),
                            token_program: self.token_program.clone(),
                            lev_farm_coin_token_account: alt.lev_farm_coin_token_account.clone(),
                            lev_farm_pc_token_account: alt.lev_farm_pc_token_account.clone(),
                            user_lp_token_account: alt.user_lp_token_account.clone(),
                            clock_sysvar: self.clock_sysvar.clone(),
                            authority: self.chamber_authority.to_account_info(),
                            lending_market_account: alt.lending_market_account.clone(),
                            user_obligation_account: alt.user_obligation_account.clone(),
                            lending_market_authority: alt.lending_market_authority.clone(),
                            lending_program_id: alt.lending_program.clone(),
                            user_position_info: alt.position_info_account.clone(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    0,
                    0,
                )?;

                // 4. Swap AMM tokens for liquidity
                cpi::tulip::leveraged::raydium::swap_to_repay_raydium(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::raydium::SwapToRepayRaydium {
                            authority: self.chamber_authority.to_account_info(),
                            leveraged_farm: alt.leveraged_farm.clone(),
                            user_farm: alt.user_farm.clone(),
                            user_farm_obligation: alt.user_farm_obligation.clone(),
                            token_program: self.token_program.clone(),
                            vault_signer: alt.vault_signer.clone(),
                            swap_or_liquidity_program_id: alt.swap_or_liquidity_program_id.clone(),
                            amm_id: alt.amm_id.clone(),
                            amm_authority: alt.amm_authority.clone(),
                            amm_open_orders: alt.amm_open_orders.clone(),
                            amm_quantities_or_target_orders: alt
                                .amm_quantities_or_target_orders
                                .clone(),
                            pool_coin_token_account: alt.pool_coin_token_account.clone(),
                            pool_pc_token_account: alt.pool_pc_token_account.clone(),
                            serum_program_id: alt.serum_program_id.clone(),
                            serum_market: alt.serum_market.clone(),
                            serum_bids: alt.serum_bids.clone(),
                            serum_asks: alt.serum_asks.clone(),
                            serum_event_queue: alt.serum_event_queue.clone(),
                            serum_coin_vault_account: alt.serum_coin_vault_account.clone(),
                            serum_pc_vault_account: alt.serum_pc_vault_account.clone(),
                            serum_vault_signer: alt.serum_vault_signer.clone(),
                            coin_wallet: alt.coin_wallet.clone(),
                            pc_wallet: alt.pc_wallet.clone(),
                            lending_market_account: alt.lending_market_account.clone(),
                            lending_market_authority: alt.lending_market_authority.clone(),
                            lending_program_id: alt.lending_program.clone(),
                            asset_price_account: alt.asset_price_account.clone(),
                            base_price_account: alt.base_price_account.clone(),
                            quote_price_account: alt.quote_price_account.clone(),
                            asset_vault: alt.asset_vault.clone(),
                            user_position_info: alt.position_info_account.clone(),
                            first_reserve: alt.first_reserve.clone(),
                            first_reserve_price: alt.first_reserve_price.clone(),
                            second_reserve: alt.second_reserve.clone(),
                            second_reserve_price: alt.second_reserve_price.clone(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    0,
                )?;

                // 5. Repay lending obligation
                cpi::tulip::leveraged::repay_obligation_liquidity_external(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::RepayObligationLiquidityExternal {
                            authority: self.chamber_authority.to_account_info(),
                            user_farm: alt.user_farm.clone(),
                            user_farm_obligation: alt.user_farm_obligation.clone(),
                            leveraged_farm: alt.leveraged_farm.clone(),
                            coin_source_token_account: alt.coin_source_token_account.clone(),
                            coin_destination_token_account: alt
                                .coin_destination_token_account
                                .clone(),
                            pc_source_token_account: alt.pc_source_token_account.clone(),
                            pc_destination_token_account: alt.pc_destination_token_account.clone(),
                            coin_reserve_account: alt.coin_reserve_account.clone(),
                            pc_reserve_account: alt.pc_reserve_account.clone(),
                            lending_market_account: alt.lending_market_account.clone(),
                            lending_market_authority: alt.lending_market_authority.clone(),
                            clock_sysvar: self.clock_sysvar.clone(),
                            token_program: self.token_program.clone(),
                            lending_program: alt.lending_program.clone(),
                            lp_pyth_price_account: alt.lp_pyth_price_account.clone(),
                            coin_price_account: alt.base_price_account.clone(),
                            pc_price_account: alt.quote_price_account.clone(),
                            vault_account: alt.vault.clone(),
                            user_coin_token_account: self.chamber_base_token.to_account_info(),
                            user_pc_token_account: self.chamber_quote_token.to_account_info(),
                            position_info_account: alt.position_info_account.clone(),
                            first_reserve: alt.first_reserve.clone(),
                            first_reserve_price: alt.first_reserve_price.clone(),
                            second_reserve: alt.second_reserve.clone(),
                            second_reserve_price: alt.second_reserve_price.clone(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    &vec![alt.first_reserve.key(), alt.second_reserve.key()],
                    0,
                )?;

                self.chamber_base_token.reload()?;
                self.chamber_quote_token.reload()?;

                let base_amount = self.chamber_base_token.amount;
                let quote_amount = self.chamber_quote_token.amount;

                // 6. Get base token price and decimals
                let base_price = tulipv2_sdk_common::pyth::load_pyth_price(
                    &self.chamber_base_oracle.data.as_ref().borrow(),
                )?;

                // 7. Calculate chamber base token value in `base_price`
                let chamber_base_value = base_price
                    .try_mul(base_amount)?
                    .try_div(self.chamber.vault.base_decimals)?;

                // 8. Get quote token price and decimals
                let quote_price = tulipv2_sdk_common::pyth::load_pyth_price(
                    &self.chamber_quote_oracle.data.as_ref().borrow(),
                )?;

                // 9. Calculate chamber quote token value in `quote_price`
                let chamber_quote_value = quote_price
                    .try_mul(quote_amount)?
                    .try_div(self.chamber.vault.quote_decimals)?;

                // 10. Calculate total chamber deposit value
                let chamber_total_value = chamber_base_value.try_add(chamber_quote_value)?;

                // 11. Calculate base and quote borrow amount
                let (chamber_base_borrow_amount, chamber_quote_borrow_amount) = {
                    let (volatile_price, underlying_price) =
                        if self.chamber.strategy.is_base_volatile {
                            (base_price, quote_price)
                        } else {
                            (quote_price, base_price)
                        };

                    let volatile_borrow_amount = chamber_total_value
                        .try_mul(3)?
                        .try_div(4)?
                        .try_mul(self.chamber.strategy.leverage - 1)?
                        .try_div(volatile_price)?
                        .try_floor_u64()?;

                    let underlying_borrow_amount = chamber_total_value
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
                            user_farm: alt.user_farm.to_account_info(),
                            leveraged_farm: alt.leveraged_farm.to_account_info(),
                            user_farm_obligation: alt.user_farm_obligation.to_account_info(),
                            coin_source_token_account: self.chamber_base_token.to_account_info(),
                            coin_destination_token_account: alt
                                .coin_destination_token_account
                                .to_account_info(),
                            pc_source_token_account: self.chamber_quote_token.to_account_info(),
                            pc_destination_token_account: alt
                                .pc_destination_token_account
                                .to_account_info(),
                            coin_deposit_reserve_account: alt
                                .coin_deposit_reserve_account
                                .to_account_info(),
                            pc_deposit_reserve_account: alt
                                .pc_deposit_reserve_account
                                .to_account_info(),
                            coin_reserve_liquidity_oracle: self
                                .chamber_base_oracle
                                .to_account_info()
                                .to_account_info(),
                            pc_reserve_liquidity_oracle: self
                                .chamber_quote_oracle
                                .to_account_info()
                                .to_account_info(),
                            lending_market_account: alt.lending_market_account.to_account_info(),
                            derived_lending_market_authority: alt
                                .lending_market_authority
                                .to_account_info(),
                            lending_program: alt.lending_program.to_account_info(),
                            coin_source_reserve_liquidity_token_account: alt
                                .coin_source_reserve_liquidity_token_account
                                .to_account_info(),
                            pc_source_reserve_liquidity_token_account: alt
                                .pc_source_reserve_liquidity_token_account
                                .to_account_info(),
                            coin_reserve_liquidity_fee_receiver: alt
                                .coin_reserve_liquidity_fee_receiver
                                .to_account_info(),
                            pc_reserve_liquidity_fee_receiver: alt
                                .pc_reserve_liquidity_fee_receiver
                                .to_account_info(),
                            borrow_authorizer: alt.borrow_authorizer.to_account_info(),
                            lp_pyth_price_account: alt.lp_pyth_price_account.to_account_info(),
                            vault_account: alt.vault_account.to_account_info(),
                            position_info_account: alt.position_info_account.to_account_info(),
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
                    chamber_base_borrow_amount,
                    chamber_quote_borrow_amount,
                    0,
                )?;

                // 13. Swap tokens via AMM
                cpi::tulip::leveraged::raydium::swap_tokens_raydium_stats(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::raydium::RaydiumSwap {
                            authority: self.chamber_authority.to_account_info(),
                            leveraged_farm: alt.leveraged_farm.to_account_info(),
                            user_farm: alt.user_farm.to_account_info(),
                            user_farm_obligation: alt.user_farm_obligation.to_account_info(),
                            token_program: self.token_program.clone(),
                            vault_signer: alt.vault_signer.to_account_info(),
                            swap_or_liquidity_program_id: alt
                                .swap_or_liquidity_program_id
                                .to_account_info(),
                            amm_id: alt.amm_id.to_account_info(),
                            amm_authority: alt.amm_authority.to_account_info(),
                            amm_open_orders: alt.amm_open_orders.to_account_info(),
                            amm_quantities_or_target_orders: alt
                                .amm_quantities_or_target_orders
                                .to_account_info(),
                            pool_coin_tokenaccount: alt.pool_coin_tokenaccount.to_account_info(),
                            pool_pc_tokenaccount: alt.pool_pc_tokenaccount.to_account_info(),
                            serum_program_id: alt.serum_program_id.to_account_info(),
                            serum_market: alt.serum_market.to_account_info(),
                            serum_bids: alt.serum_bids.to_account_info(),
                            serum_asks: alt.serum_asks.to_account_info(),
                            serum_event_queue: alt.serum_event_queue.to_account_info(),
                            serum_coin_vault_account: alt
                                .serum_coin_vault_account
                                .to_account_info(),
                            serum_pc_vault_account: alt.serum_pc_vault_account.to_account_info(),
                            serum_vault_signer: alt.serum_vault_signer.to_account_info(),
                            coin_wallet: alt.coin_wallet.to_account_info(),
                            pc_wallet: alt.pc_wallet.to_account_info(),
                            lending_market_account: alt.lending_market_account.to_account_info(),
                            lending_market_authority: alt
                                .lending_market_authority
                                .to_account_info(),
                            lending_program: alt.lending_program.to_account_info(),
                            position_info_account: alt.position_info_account.to_account_info(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    0,
                )?;

                // 14. Deposit tokens into lp
                cpi::tulip::leveraged::add_liquidity_stats(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::AddLiquidity {
                            authority: self.chamber_authority.to_account_info(),
                            user_farm: alt.user_farm.to_account_info(),
                            leveraged_farm: alt.leveraged_farm.to_account_info(),
                            liquidity_program_id: alt.liquidity_program_id.to_account_info(),
                            amm_id: alt.amm_id.to_account_info(),
                            amm_authority: alt.amm_authority.to_account_info(),
                            amm_open_orders: alt.amm_open_orders.to_account_info(),
                            amm_quantities_or_target_orders: alt
                                .amm_quantities_or_target_orders
                                .to_account_info(),
                            lp_mint_address: alt.lp_mint_address.to_account_info(),
                            pool_coin_token_account: alt.pool_coin_tokenaccount.to_account_info(),
                            pool_pc_token_account: alt.pool_pc_tokenaccount.to_account_info(),
                            serum_market: alt.serum_market.to_account_info(),
                            token_program: self.token_program.clone(),
                            lev_farm_coin_token_account: alt
                                .lev_farm_coin_token_account
                                .to_account_info(),
                            lev_farm_pc_token_account: alt
                                .lev_farm_pc_token_account
                                .to_account_info(),
                            user_lp_token_account: alt.user_lp_token_account.to_account_info(),
                            pyth_price_account: alt.pyth_price_account.to_account_info(),
                            lending_market_account: alt.lending_market_account.to_account_info(),
                            user_farm_obligation: alt.user_farm_obligation.to_account_info(),
                            derived_lending_market_authority: alt
                                .lending_market_authority
                                .to_account_info(),
                            lending_program: alt.lending_program.to_account_info(),
                            clock: self.clock_sysvar.clone(),
                            dex_program: alt.dex_program.to_account_info(),
                            position_info_account: alt.position_info_account.to_account_info(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    0,
                )?;

                // 15. Deposit lp tokens into tulip vault
                cpi::tulip::leveraged::raydium::deposit_raydium_vault(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::raydium::DepositFarm {
                            authority: self.chamber_authority.to_account_info(),
                            user_farm: alt.user_farm.to_account_info(),
                            obligation_vault_address: alt
                                .obligation_vault_address
                                .to_account_info(),
                            leveraged_farm: alt.leveraged_farm.to_account_info(),
                            vault_program: alt.vault_program.to_account_info(),
                            authority_token_account: alt.authority_token_account.to_account_info(),
                            vault_pda_account: alt.vault_pda_account.to_account_info(),
                            vault: alt.vault.to_account_info(),
                            lp_token_account: alt.lp_token_account.to_account_info(),
                            user_balance_account: alt.user_balance_account.to_account_info(),
                            system_program: self.system_program.clone(),
                            stake_program_id: alt.stake_program_id.to_account_info(),
                            pool_id: alt.pool_id.to_account_info(),
                            pool_authority: alt.pool_authority.to_account_info(),
                            vault_info_account: alt.vault_info_account.to_account_info(),
                            pool_lp_token_account: alt.pool_lp_token_account.to_account_info(),
                            user_reward_a_token_account: alt
                                .user_reward_a_token_account
                                .to_account_info(),
                            pool_reward_a_token_account: alt
                                .pool_reward_a_token_account
                                .to_account_info(),
                            user_reward_b_token_account: alt
                                .user_reward_b_token_account
                                .to_account_info(),
                            pool_reward_b_token_account: alt
                                .pool_reward_b_token_account
                                .to_account_info(),
                            clock: self.clock_sysvar.clone(),
                            rent: self.rent_sysvar.clone(),
                            token_program_id: self.token_program.clone(),
                            user_balance_metadata: alt.user_balance_metadata.to_account_info(),
                            lending_market_account: alt.lending_market_account.to_account_info(),
                            user_farm_obligation: alt.user_farm_obligation.to_account_info(),
                            lending_market_authority: alt
                                .lending_market_authority
                                .to_account_info(),
                            lending_program: alt.lending_program.to_account_info(),
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
            }
        };

        Ok(())
    }
}
