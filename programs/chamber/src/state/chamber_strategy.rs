use super::ChamberMarket;
use anchor_lang::prelude::*;

/// Provide internal strategy configuration for `state::Chamber`.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ChamberStrategy {
    pub market: ChamberMarket,
    pub farm: Pubkey,
    pub farm_program: Pubkey,

    /// Provide underlying LYF leverage config.
    pub leverage: u64,

    /// Indicates, that `ChamberVault::base` is volatile token.
    /// Primarly used in PDN(Pseudo-Delta Neutral) position calculation.
    pub is_base_volatile: bool,
}

impl ChamberStrategy {
    pub const LEN: usize = 1 + 32 + 32 + 8 + 1;

    pub fn new(
        market: ChamberMarket,
        farm: &Pubkey,
        farm_program: &Pubkey,
        leverage: u64,
        is_base_volatile: bool,
    ) -> Self {
        ChamberStrategy {
            market,
            farm: *farm,
            farm_program: *farm_program,
            leverage,
            is_base_volatile,
        }
    }
}
