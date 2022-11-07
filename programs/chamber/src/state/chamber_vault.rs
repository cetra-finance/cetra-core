use crate::error;
use anchor_lang::prelude::*;
use tulipv2_sdk_common::math::{
    common::{TryAdd, TryDiv, TryMul},
    decimal::Decimal,
};

/// Provide token-related data for `state::Chamber`.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ChamberVault {
    /// Base ata(associated spl token account).
    ///
    /// Important! - chosen based on underlying strategy.
    pub base: Pubkey,

    /// Quote ata(associated spl token account).
    ///
    /// Important! - chosen based on underlying strategy.
    pub quote: Pubkey,

    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,

    pub base_oracle: Pubkey,
    pub quote_oracle: Pubkey,

    pub base_decimals: u64,
    pub quote_decimals: u64,

    pub base_amount: u128,
    pub quote_amount: u128,
}

impl ChamberVault {
    pub const LEN: usize = 32 * 6 + 8 * 2 + 16 * 2;

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base: &Pubkey,
        quote: &Pubkey,
        base_mint: &Pubkey,
        quote_mint: &Pubkey,
        base_oracle: &Pubkey,
        quote_oracle: &Pubkey,
        base_decimals: u64,
        quote_decimals: u64,
    ) -> Self {
        ChamberVault {
            base: *base,
            quote: *quote,
            base_mint: *base_mint,
            quote_mint: *quote_mint,
            base_oracle: *base_oracle,
            quote_oracle: *quote_oracle,
            base_decimals,
            quote_decimals,
            base_amount: 0,
            quote_amount: 0,
        }
    }

    pub fn deposit(&mut self, deposit_base_amount: u64, deposit_quote_amount: u64) -> Result<()> {
        let deposit_base_amount: u128 = deposit_base_amount
            .try_into()
            .map_err(|_| error::ChamberError::MathOverflow)?;
        let deposit_quote_amount: u128 = deposit_quote_amount
            .try_into()
            .map_err(|_| error::ChamberError::MathOverflow)?;

        self.base_amount = self
            .base_amount
            .checked_add(deposit_base_amount)
            .ok_or(error::ChamberError::MathOverflow)?;
        self.quote_amount = self
            .quote_amount
            .checked_add(deposit_quote_amount)
            .ok_or(error::ChamberError::MathOverflow)?;

        Ok(())
    }

    /// TODO: Deprecate. Instead, calculate value from internal position.
    pub fn get_total_value(&self, base_price: &Decimal, quote_price: &Decimal) -> Result<u64> {
        let base_amount = Decimal::from(self.base_amount);
        let quote_amount = Decimal::from(self.quote_amount);

        let base_value = base_price
            .try_mul(base_amount)?
            .try_div(self.base_decimals)?;
        let quote_value = quote_price
            .try_mul(quote_amount)?
            .try_div(self.quote_decimals)?;

        Ok(base_value.try_add(quote_value)?.try_floor_u64()?)
    }
}
