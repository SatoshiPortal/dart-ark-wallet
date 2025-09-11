use anyhow::{Result, anyhow};
use crate::ark::client::ArkWallet;
use std::str::FromStr;

pub use bitcoin::Address;
pub use bitcoin::Amount;

impl ArkWallet {
    pub async fn send_on_chain(&self, address: String, sats: i64) -> Result<String> {
        let address = Address::from_str(address.as_str())?;
        let txid = self.inner
            .send_on_chain(address.assume_checked(), Amount::from_sat(sats as u64))
            .await
            .map_err(|e| anyhow!("Failed sending onchain {e:#}"))?;
        Ok(txid.to_string())
    }

    pub async fn send_off_chain(&self, address: String, sats: i64) -> Result<String> {
        let address = ark_core::ArkAddress::decode(address.as_str())?;
        let txid = self.inner
            .send_vtxo(address, Amount::from_sat(sats as u64))
            .await
            .map_err(|e| anyhow!("Failed sending offchain {e:#}"))?;
        Ok(txid.to_string())    
    }
}