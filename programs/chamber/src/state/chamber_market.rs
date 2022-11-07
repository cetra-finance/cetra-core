use anchor_lang::prelude::*;

/// Represent `state::Chamber` position market.
/// Indicates all supported(integrated) protocols.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub enum ChamberMarket {
    Tulip,
}
