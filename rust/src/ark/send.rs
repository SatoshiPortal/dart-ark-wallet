use anyhow::{Result, anyhow};
use crate::ark::client::ArkClient;
use std::str::FromStr;

pub use bitcoin::Address;
pub use bitcoin::Amount;
pub use bitcoin::Txid;

impl ArkClient {
    pub async fn send_on_chain(&self, address: String, sats: i64) -> Result<Txid> {
        let address = Address::from_str(address.as_str())?;
        let txid = self.inner
            .send_on_chain(address.assume_checked(), Amount::from_sat(sats as u64))
            .await
            .map_err(|e| anyhow!("Failed sending onchain {e:#}"))?;
        Ok(txid)
    }

    pub async fn send_off_chain(&self, address: String, sats: i64) -> Result<Txid> {
        let address = ark_core::ArkAddress::decode(address.as_str())?;
        let txid = self.inner
            .send_vtxo(address, Amount::from_sat(sats as u64))
            .await
            .map_err(|e| anyhow!("Failed sending offchain {e:#}"))?;
        Ok(txid)
    }
}