use crate::{cpi, state, utils};
use anchor_lang::prelude::*;
use anchor_spl::token;

#[derive(Accounts)]
pub struct ProcessDepositChamber<'info> {
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

impl<'c, 'info> ProcessDepositChamber<'info> {
    pub fn process(&mut self, remaining_accounts: &'c [AccountInfo<'info>]) -> Result<()> {
        // 1. Ensure, that `UserAccount` in correct state
        self.user_account
            .assert_status(state::UserAccountStatus::BeginDeposit)?;

        // 2. Process market specific logic
        match self.chamber.strategy.market {
            state::ChamberMarket::Tulip => {
                let user_farm = &remaining_accounts[0];
                let leveraged_farm = &remaining_accounts[1];
                let user_farm_obligation = &remaining_accounts[2];
                let lending_market_account = &remaining_accounts[3];
                let lending_program = &remaining_accounts[4];
                let position_info_account = &remaining_accounts[5];
                let vault_signer = &remaining_accounts[6];
                let swap_or_liquidity_program_id = &remaining_accounts[7];
                let amm_id = &remaining_accounts[8];
                let amm_authority = &remaining_accounts[9];
                let amm_open_orders = &remaining_accounts[10];
                let amm_quantities_or_target_orders = &remaining_accounts[11];
                let pool_coin_tokenaccount = &remaining_accounts[12];
                let pool_pc_tokenaccount = &remaining_accounts[13];
                let serum_program_id = &remaining_accounts[14];
                let serum_market = &remaining_accounts[15];
                let serum_bids = &remaining_accounts[16];
                let serum_asks = &remaining_accounts[17];
                let serum_event_queue = &remaining_accounts[18];
                let serum_coin_vault_account = &remaining_accounts[19];
                let serum_pc_vault_account = &remaining_accounts[20];
                let serum_vault_signer = &remaining_accounts[21];
                let coin_wallet = &remaining_accounts[22];
                let pc_wallet = &remaining_accounts[23];
                let lending_market_authority = &remaining_accounts[24];
                let derived_lending_market_authority = &remaining_accounts[25];
                let liquidity_program_id = &remaining_accounts[26];
                let lp_mint_address = &remaining_accounts[27];
                let lev_farm_coin_token_account = &remaining_accounts[28];
                let lev_farm_pc_token_account = &remaining_accounts[29];
                let user_lp_token_account = &remaining_accounts[30];
                let pyth_price_account = &remaining_accounts[31];
                let dex_program = &remaining_accounts[32];

                // 3. Swap tokens via AMM
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

                // 4. Deposit tokens into lp
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
                            derived_lending_market_authority: derived_lending_market_authority
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

                // 5. Update `UserAccount` state
                self.user_account.process_deposit();
            }
        };

        Ok(())
    }
}
