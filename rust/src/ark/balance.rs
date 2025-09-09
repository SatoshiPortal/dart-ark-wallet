use anyhow::{Result, anyhow};
use crate::ark::client::ArkClient;

pub use ark_client::OffChainBalance;

pub struct Balance {
    pub pending: u64,
    pub confirmed: u64,
    pub total: u64,
}

impl ArkClient {
    pub async fn balance(&self) -> Result<Balance> {
        let offchain_balance = self.inner
        .offchain_balance()
        .await
        .map_err(|error| anyhow!("Could not fetch balance {error}"))?;

         Ok(Balance {pending: offchain_balance.pending().to_sat(), confirmed: offchain_balance.confirmed().to_sat(), total: offchain_balance.total().to_sat()})
    }
}