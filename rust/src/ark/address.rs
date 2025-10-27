use anyhow::{anyhow, Result};
use flutter_rust_bridge::frb;

pub use ark_core::ArkAddress;
pub use bitcoin::Address;

use crate::ark::client::ArkWallet;

impl ArkWallet {
    #[frb(sync)]
    pub fn offchain_address(&self) -> Result<String> {
        let (offchain_address, _vtxo) = self
            .inner
            .get_offchain_address()
            .map_err(|error| anyhow!("Could not get offchain address {error:#}"))?;

        Ok(offchain_address.encode())
    }

    #[frb(sync)]
    pub fn boarding_address(&self) -> Result<String> {
        let boarding_address = self
            .inner
            .get_boarding_address()
            .map_err(|error| anyhow!("Could not get boarding address {error:#}"))?;

        Ok(boarding_address.to_string())
    }

    #[frb(sync)]
    pub fn onchain_address(&self) -> Result<String> {
        // Get the general onchain Bitcoin address for the wallet
        let onchain_address = self
            .inner
            .get_onchain_address()
            .map_err(|error| anyhow!("Could not get onchain address {error:#}"))?;
        Ok(onchain_address.to_string())
    }
}
