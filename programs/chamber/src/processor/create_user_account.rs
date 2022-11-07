use crate::{state, utils};
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    /// Chamber will be associated with `UserAccount`.
    pub chamber: Box<Account<'info, state::Chamber>>,

    #[account(
        init,
        payer = user,
        space = state::UserAccount::LEN,
        seeds = [
            utils::USER_ACCOUNT_PREFIX.as_bytes(),
            chamber.key().as_ref(),
            user.key().as_ref(),
        ],
        bump
    )]
    pub user_account: Box<Account<'info, state::UserAccount>>,

    /// CHECK: Uninitialized shares ata for `user`.
    #[account(mut)]
    pub user_shares: UncheckedAccount<'info>,

    #[account(constraint = shares_mint.key() == chamber.config.shares_mint)]
    pub shares_mint: Box<Account<'info, token::Mint>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateUserAccount<'info> {
    pub fn process(&mut self) -> Result<()> {
        // 1. Create user shares token ata
        associated_token::create(CpiContext::new(
            self.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: self.user.to_account_info(),
                associated_token: self.user_shares.to_account_info(),
                authority: self.user.to_account_info(),
                mint: self.shares_mint.to_account_info(),
                rent: self.rent_sysvar.to_account_info(),
                token_program: self.token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
            },
        ))?;

        // 2. Initialize `UserAccount` position for provided `Chamber`
        self.user_account
            .init(&self.chamber.key(), self.user.key, self.user_shares.key);

        Ok(())
    }
}
