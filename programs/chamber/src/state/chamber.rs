use super::{ChamberConfig, ChamberStrategy, ChamberVault};
use anchor_lang::prelude::*;

/// Protocol vault, which represents pool position.
#[account]
#[derive(Debug)]
pub struct Chamber {
    /// Internal strategy details and config.
    pub strategy: ChamberStrategy,

    /// Stores `Chamber` tokens config and related data.
    pub vault: ChamberVault,

    /// `Chamber` config.
    pub config: ChamberConfig,
}

impl Chamber {
    pub const LEN: usize = 8 + ChamberStrategy::LEN + ChamberVault::LEN + ChamberConfig::LEN;

    pub fn init(
        &mut self,
        strategy: &ChamberStrategy,
        vault: &ChamberVault,
        config: &ChamberConfig,
    ) {
        self.strategy = strategy.clone();
        self.vault = vault.clone();
        self.config = config.clone();
    }
}
