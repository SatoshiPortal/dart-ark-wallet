use crate::ark::client::ArkWallet;
use anyhow::{anyhow, Result};

pub use ark_client::OffChainBalance;

pub struct Balance {
    pub pending: i64,
    pub confirmed: i64,
    pub total: i64,
}

impl ArkWallet {
    pub async fn balance(&self) -> Result<Balance> {
        let offchain_balance = self
            .inner
            .offchain_balance()
            .await
            .map_err(|error| anyhow!("Could not fetch balance {error}"))?;

        Ok(Balance {
            pending: offchain_balance.pending().to_sat() as i64,
            confirmed: offchain_balance.confirmed().to_sat() as i64,
            total: offchain_balance.total().to_sat() as i64,
        })
    }
}
