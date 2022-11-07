use anchor_lang::prelude::*;

pub const SHARES_DECIMALS: u8 = 6;
pub const CHAMBER_PREFIX: &str = "chamber";
pub const CHAMBER_AUTHORITY_PREFIX: &str = "chamber_authority";
pub const USER_ACCOUNT_PREFIX: &str = "user_account";

pub fn derive_chamber_address(
    farm: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
    nonce: u8,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            CHAMBER_PREFIX.as_bytes(),
            farm.as_ref(),
            base_mint.as_ref(),
            quote_mint.as_ref(),
            nonce.to_le_bytes().as_ref(),
        ],
        &crate::id(),
    )
}

pub fn derive_chamber_authority_address(chamber: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[CHAMBER_AUTHORITY_PREFIX.as_bytes(), chamber.as_ref()],
        &crate::id(),
    )
}

pub fn derive_user_account_address(chamber: &Pubkey, user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            USER_ACCOUNT_PREFIX.as_bytes(),
            chamber.as_ref(),
            user.as_ref(),
        ],
        &crate::id(),
    )
}
