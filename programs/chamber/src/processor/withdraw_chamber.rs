use crate::{cpi, error, state, utils};
use anchor_lang::prelude::*;
use anchor_spl::token;

#[derive(Accounts)]
#[instruction(base_amount: u64, quote_amount: u64)]
pub struct WithdrawChamber<'info> {
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

impl<'c, 'info> WithdrawChamber<'info> {
    pub fn process(
        &mut self,
        remaining_accounts: &'c [AccountInfo<'info>],
        base_amount: u64,
        quote_amount: u64,
    ) -> Result<()> {
        // 1. Ensure, that `UserAccount` in correct state
        self.user_account
            .assert_status(state::UserAccountStatus::Ready)?;

        // 2. Ensure, that `User` is withdrawing something valuable
        if base_amount == 0 && quote_amount == 0 {
            return Err(error::ChamberError::InsufficientFunds.into());
        }

        // 3. Process market specific logic
        match self.chamber.strategy.market {
            state::ChamberMarket::Tulip => {
                let user_farm = &remaining_accounts[0];
                let obligation_vault_address = &remaining_accounts[1];
                let leveraged_farm = &remaining_accounts[2];
                let authority_token_account = &remaining_accounts[3];
                let vault = &remaining_accounts[4];
                let vault_program = &remaining_accounts[5];
                let user_balance_account = &remaining_accounts[6];
                let user_info_account = &remaining_accounts[7];
                let user_lp_token_account = &remaining_accounts[8];
                let user_reward_a_token_account = &remaining_accounts[9];
                let pool_reward_a_token_account = &remaining_accounts[10];
                let user_reward_b_token_account = &remaining_accounts[11];
                let pool_reward_b_token_account = &remaining_accounts[12];
                let vault_pda_account = &remaining_accounts[13];
                let pool_lp_token_account = &remaining_accounts[14];
                let pool_authority = &remaining_accounts[15];
                let pool_id = &remaining_accounts[16];
                let stake_program_id = &remaining_accounts[17];
                let user_balance_meta = &remaining_accounts[18];
                let lending_market_account = &remaining_accounts[19];
                let user_farm_obligation = &remaining_accounts[20];
                let lending_market_authority = &remaining_accounts[21];
                let lending_program = &remaining_accounts[22];
                let position_info_account = &remaining_accounts[23];
                let liquidity_program_id = &remaining_accounts[24];
                let amm_id = &remaining_accounts[25];
                let amm_authority = &remaining_accounts[26];
                let amm_open_orders = &remaining_accounts[27];
                let amm_quantities_or_target_orders = &remaining_accounts[28];
                let lp_mint_address = &remaining_accounts[29];
                let pool_coin_token_account = &remaining_accounts[30];
                let pool_pc_token_account = &remaining_accounts[31];
                let pool_withdraw_queue = &remaining_accounts[32];
                let pool_temp_lp_token_account = &remaining_accounts[33];
                let serum_program_id = &remaining_accounts[34];
                let serum_market = &remaining_accounts[35];
                let serum_coin_vault_account = &remaining_accounts[36];
                let serum_pc_vault_account = &remaining_accounts[37];
                let serum_vault_signer = &remaining_accounts[38];
                let lev_farm_coin_token_account = &remaining_accounts[39];
                let lev_farm_pc_token_account = &remaining_accounts[40];
                let user_obligation_account = &remaining_accounts[41];
                let vault_signer = &remaining_accounts[42];
                let swap_or_liquidity_program_id = &remaining_accounts[43];
                let serum_bids = &remaining_accounts[44];
                let serum_asks = &remaining_accounts[45];
                let serum_event_queue = &remaining_accounts[46];
                let coin_wallet = &remaining_accounts[47];
                let pc_wallet = &remaining_accounts[48];
                let asset_price_account = &remaining_accounts[49];
                let base_price_account = &remaining_accounts[50];
                let quote_price_account = &remaining_accounts[51];
                let asset_vault = &remaining_accounts[52];
                let first_reserve = &remaining_accounts[53];
                let first_reserve_price = &remaining_accounts[54];
                let second_reserve = &remaining_accounts[55];
                let second_reserve_price = &remaining_accounts[56];
                let coin_source_token_account = &remaining_accounts[57];
                let coin_destination_token_account = &remaining_accounts[58];
                let pc_source_token_account = &remaining_accounts[59];
                let pc_destination_token_account = &remaining_accounts[60];
                let coin_reserve_account = &remaining_accounts[61];
                let pc_reserve_account = &remaining_accounts[62];
                let lp_pyth_price_account = &remaining_accounts[63];

                // 4. Withdraw lp from vault
                // TODO: Calculate percentage
                cpi::tulip::leveraged::raydium::withdraw_raydium_vault_close(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::raydium::WithdrawFarm {
                            authority: self.chamber_authority.to_account_info(),
                            user_farm: user_farm.clone(),
                            obligation_vault_address: obligation_vault_address.clone(),
                            leveraged_farm: leveraged_farm.clone(),
                            authority_token_account: authority_token_account.clone(),
                            vault: vault.clone(),
                            vault_program: vault_program.clone(),
                            user_balance_account: user_balance_account.clone(),
                            user_info_account: user_info_account.clone(),
                            user_lp_token_account: user_lp_token_account.clone(),
                            user_reward_a_token_account: user_reward_a_token_account.clone(),
                            pool_reward_a_token_account: pool_reward_a_token_account.clone(),
                            user_reward_b_token_account: user_reward_b_token_account.clone(),
                            pool_reward_b_token_account: pool_reward_b_token_account.clone(),
                            token_program_id: self.token_program.clone(),
                            clock: self.clock_sysvar.clone(),
                            vault_pda_account: vault_pda_account.clone(),
                            pool_lp_token_account: pool_lp_token_account.clone(),
                            pool_authority: pool_authority.clone(),
                            pool_id: pool_id.clone(),
                            stake_program_id: stake_program_id.clone(),
                            user_balance_meta: user_balance_meta.clone(),
                            lending_market_account: lending_market_account.clone(),
                            user_farm_obligation: user_farm_obligation.clone(),
                            lending_market_authority: lending_market_authority.clone(),
                            lending_program: lending_program.clone(),
                            position_info_account: position_info_account.clone(),
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
                    0,
                    0,
                )?;

                // 5. Remove liquidity from AMM for lp tokens
                cpi::tulip::leveraged::remove_liquidity_new(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::RemoveLiquidityNew {
                            user_farm: user_farm.clone(),
                            obligation_vault_address: obligation_vault_address.clone(),
                            leveraged_farm: leveraged_farm.clone(),
                            liquidity_program_id: liquidity_program_id.clone(),
                            amm_id: amm_id.clone(),
                            amm_authority: amm_authority.clone(),
                            amm_open_orders: amm_open_orders.clone(),
                            amm_quantities_or_target_orders: amm_quantities_or_target_orders
                                .clone(),
                            lp_mint_address: lp_mint_address.clone(),
                            pool_coin_token_account: pool_coin_token_account.clone(),
                            pool_pc_token_account: pool_pc_token_account.clone(),
                            pool_withdraw_queue: pool_withdraw_queue.clone(),
                            pool_temp_lp_token_account: pool_temp_lp_token_account.clone(),
                            serum_program_id: serum_program_id.clone(),
                            serum_market: serum_market.clone(),
                            serum_coin_vault_account: serum_coin_vault_account.clone(),
                            serum_pc_vault_account: serum_pc_vault_account.clone(),
                            serum_vault_signer: serum_vault_signer.clone(),
                            token_program: self.token_program.clone(),
                            lev_farm_coin_token_account: lev_farm_coin_token_account.clone(),
                            lev_farm_pc_token_account: lev_farm_pc_token_account.clone(),
                            user_lp_token_account: user_lp_token_account.clone(),
                            clock_sysvar: self.clock_sysvar.clone(),
                            authority: self.chamber_authority.to_account_info(),
                            lending_market_account: lending_market_account.clone(),
                            user_obligation_account: user_obligation_account.clone(),
                            lending_market_authority: lending_market_authority.clone(),
                            lending_program_id: lending_program.clone(),
                            user_position_info: position_info_account.clone(),
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

                // 6. Swap AMM tokens for liquidity
                cpi::tulip::leveraged::raydium::swap_to_repay_raydium(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::raydium::SwapToRepayRaydium {
                            authority: self.chamber_authority.to_account_info(),
                            leveraged_farm: leveraged_farm.clone(),
                            user_farm: user_farm.clone(),
                            user_farm_obligation: user_farm_obligation.clone(),
                            token_program: self.token_program.clone(),
                            vault_signer: vault_signer.clone(),
                            swap_or_liquidity_program_id: swap_or_liquidity_program_id.clone(),
                            amm_id: amm_id.clone(),
                            amm_authority: amm_authority.clone(),
                            amm_open_orders: amm_open_orders.clone(),
                            amm_quantities_or_target_orders: amm_quantities_or_target_orders
                                .clone(),
                            pool_coin_token_account: pool_coin_token_account.clone(),
                            pool_pc_token_account: pool_pc_token_account.clone(),
                            serum_program_id: serum_program_id.clone(),
                            serum_market: serum_market.clone(),
                            serum_bids: serum_bids.clone(),
                            serum_asks: serum_asks.clone(),
                            serum_event_queue: serum_event_queue.clone(),
                            serum_coin_vault_account: serum_coin_vault_account.clone(),
                            serum_pc_vault_account: serum_pc_vault_account.clone(),
                            serum_vault_signer: serum_vault_signer.clone(),
                            coin_wallet: coin_wallet.clone(),
                            pc_wallet: pc_wallet.clone(),
                            lending_market_account: lending_market_account.clone(),
                            lending_market_authority: lending_market_authority.clone(),
                            lending_program_id: lending_program.clone(),
                            asset_price_account: asset_price_account.clone(),
                            base_price_account: base_price_account.clone(),
                            quote_price_account: quote_price_account.clone(),
                            asset_vault: asset_vault.clone(),
                            user_position_info: position_info_account.clone(),
                            first_reserve: first_reserve.clone(),
                            first_reserve_price: first_reserve_price.clone(),
                            second_reserve: second_reserve.clone(),
                            second_reserve_price: second_reserve_price.clone(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    0,
                )?;

                // 7. Repay lending obligation
                cpi::tulip::leveraged::repay_obligation_liquidity_external(
                    CpiContext::new_with_signer(
                        self.chamber_farm_program.to_account_info(),
                        Box::new(cpi::tulip::leveraged::RepayObligationLiquidityExternal {
                            authority: self.chamber_authority.to_account_info(),
                            user_farm: user_farm.clone(),
                            user_farm_obligation: user_farm_obligation.clone(),
                            leveraged_farm: leveraged_farm.clone(),
                            coin_source_token_account: coin_source_token_account.clone(),
                            coin_destination_token_account: coin_destination_token_account.clone(),
                            pc_source_token_account: pc_source_token_account.clone(),
                            pc_destination_token_account: pc_destination_token_account.clone(),
                            coin_reserve_account: coin_reserve_account.clone(),
                            pc_reserve_account: pc_reserve_account.clone(),
                            lending_market_account: lending_market_account.clone(),
                            lending_market_authority: lending_market_authority.clone(),
                            clock_sysvar: self.clock_sysvar.clone(),
                            token_program: self.token_program.clone(),
                            lending_program: lending_program.clone(),
                            lp_pyth_price_account: lp_pyth_price_account.clone(),
                            coin_price_account: base_price_account.clone(),
                            pc_price_account: quote_price_account.clone(),
                            vault_account: vault.clone(),
                            user_coin_token_account: self.chamber_base_token.to_account_info(),
                            user_pc_token_account: self.chamber_quote_token.to_account_info(),
                            position_info_account: position_info_account.clone(),
                            first_reserve: first_reserve.clone(),
                            first_reserve_price: first_reserve_price.clone(),
                            second_reserve: second_reserve.clone(),
                            second_reserve_price: second_reserve_price.clone(),
                        }),
                        &[&[
                            utils::CHAMBER_AUTHORITY_PREFIX.as_bytes(),
                            self.chamber.key().as_ref(),
                            &[self.chamber.config.authority_bump],
                        ]],
                    ),
                    &vec![first_reserve.key(), second_reserve.key()],
                    0,
                )?;
            }
        };

        Ok(())
    }
}
