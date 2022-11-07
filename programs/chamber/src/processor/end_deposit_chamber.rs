use crate::{cpi, state, utils};
use anchor_lang::prelude::*;
use anchor_spl::token;

#[derive(Accounts)]
pub struct EndDepositChamber<'info> {
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
        constraint = user_account.shares == user_shares.key()
    )]
    pub user_account: Box<Account<'info, state::UserAccount>>,

    #[account(mut, constraint = chamber_shares_mint.key() == chamber.config.shares_mint)]
    pub chamber_shares_mint: Box<Account<'info, token::Mint>>,

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

    #[account(mut, constraint = user_shares.key() == user_account.shares)]
    pub user_shares: Box<Account<'info, token::TokenAccount>>,

    pub user: Signer<'info>,

    pub clock_sysvar: Sysvar<'info, Clock>,
    pub rent_sysvar: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}

impl<'c, 'info> EndDepositChamber<'info> {
    pub fn process(&mut self, remaining_accounts: &'c [AccountInfo<'info>]) -> Result<()> {
        // 1. Ensure, that `UserAccount` in correct state
        self.user_account
            .assert_status(state::UserAccountStatus::ProcessDeposit)?;

        // 2. Process market specific logic
        match self.chamber.strategy.market {
            state::ChamberMarket::Tulip => {
                let user_farm = &remaining_accounts[0];
                let obligation_vault_address = &remaining_accounts[1];
                let leveraged_farm = &remaining_accounts[2];
                let vault_program = &remaining_accounts[3];
                let authority_token_account = &remaining_accounts[4];
                let vault_pda_account = &remaining_accounts[5];
                let vault = &remaining_accounts[6];
                let lp_token_account = &remaining_accounts[7];
                let user_balance_account = &remaining_accounts[8];
                let stake_program_id = &remaining_accounts[9];
                let pool_id = &remaining_accounts[10];
                let pool_authority = &remaining_accounts[11];
                let vault_info_account = &remaining_accounts[12];
                let pool_lp_token_account = &remaining_accounts[13];
                let user_reward_a_token_account = &remaining_accounts[14];
                let pool_reward_a_token_account = &remaining_accounts[15];
                let user_reward_b_token_account = &remaining_accounts[16];
                let pool_reward_b_token_account = &remaining_accounts[17];
                let user_balance_metadata = &remaining_accounts[18];
                let lending_market_account = &remaining_accounts[19];
                let user_farm_obligation = &remaining_accounts[20];
                let lending_market_authority = &remaining_accounts[21];
                let lending_program = &remaining_accounts[22];

                // 3. Deposit lp tokens into tulip vault
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

                // 4. Mint shares to user
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
                    self.user_account.locked_shares_amount,
                )?;

                // 5. Update `Chamber` state
                self.chamber.vault.deposit(
                    self.user_account.locked_base_amount,
                    self.user_account.locked_quote_amount,
                )?;

                // 6. Update `UserAccount` state
                self.user_account.end_deposit();
            }
        };

        Ok(())
    }
}
