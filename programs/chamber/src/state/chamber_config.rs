use anchor_lang::prelude::*;

/// Provide internal configuration for `state::Chamber`.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ChamberConfig {
    pub authority: Pubkey,
    pub owner: Pubkey,
    pub fee_manager: Pubkey,
    pub shares_mint: Pubkey,
    pub authority_bump: u8,

    /// Chamber nonce(index).
    pub nonce: u8,
}

impl ChamberConfig {
    pub const LEN: usize = 32 * 4 + 1 + 1;

    pub fn new(
        authority: &Pubkey,
        owner: &Pubkey,
        fee_manager: &Pubkey,
        shares_mint: &Pubkey,
        authority_bump: u8,
        nonce: u8,
    ) -> Self {
        ChamberConfig {
            authority: *authority,
            owner: *owner,
            fee_manager: *fee_manager,
            shares_mint: *shares_mint,
            authority_bump,
            nonce,
        }
    }
}
