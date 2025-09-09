use anyhow::{Result, anyhow};
use crate::ark::client::ArkClient;

pub use bitcoin::Address;
pub use ark_core::ArkAddress;

pub struct Addresses {
    pub boarding: String,
    pub offchain: String,
}

impl ArkClient {
    pub async fn addresses(&self) -> Result<Addresses> {
        let boarding_address = self.inner
        .get_boarding_address()
        .map_err(|error| anyhow!("Could not get boarding address {error:#}"))?;

    let (offchain_address, _vtxo) = self.inner
        .get_offchain_address()
        .map_err(|error| anyhow!("Could not get offchain address {error:#}"))?;

    Ok(Addresses {
        boarding: boarding_address.to_string(),
        offchain: offchain_address.encode(),
    })
    }
}