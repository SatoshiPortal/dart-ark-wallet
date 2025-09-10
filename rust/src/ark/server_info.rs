use anyhow::{Result};
use flutter_rust_bridge::frb;

use crate::ark::client::ArkClient;



pub struct ServerInfo {
    pub pk: String,
    pub vtxo_tree_expiry: u32,
    pub unilateral_exit_delay: u32,
    pub boarding_exit_delay: u32,
    pub round_interval: i64,
    pub network: String,
    pub dust: u64,
    pub forfeit_address: String,
    pub version: String,
    pub utxo_min_amount: Option<u64>,
    pub utxo_max_amount: Option<u64>,
    pub vtxo_min_amount: Option<u64>,
    pub vtxo_max_amount: Option<u64>,
}

impl ArkClient {
    #[frb(sync)]
    pub fn server_info(&self) -> Result<ServerInfo> {
        let info = self.inner.server_info.clone();
        Ok(ServerInfo {
            pk: info.pk.to_string(),
            vtxo_tree_expiry: info.vtxo_tree_expiry.0,
            unilateral_exit_delay: info.unilateral_exit_delay.0,
            boarding_exit_delay: info.boarding_exit_delay.0,
            round_interval: info.round_interval,
            network: info.network.to_string(),
            dust: info.dust.to_sat(),
            forfeit_address: info.forfeit_address.to_string(),
            version: info.version,
            utxo_min_amount: info.utxo_min_amount.map(|amount| amount.to_sat()),
            utxo_max_amount: info.utxo_max_amount.map(|amount| amount.to_sat()),
            vtxo_min_amount: info.vtxo_min_amount.map(|amount| amount.to_sat()),
            vtxo_max_amount: info.vtxo_max_amount.map(|amount| amount.to_sat()),
        })
    }
}