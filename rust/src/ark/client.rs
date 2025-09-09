
use crate::ark::storage::InMemoryDb;
use crate::ark::esplora::EsploraClient;

use anyhow::{Result, anyhow};
use ark_client::OfflineClient;
use bitcoin::key::Secp256k1;
use bitcoin::Network;
use nostr::Keys;
use std::str::FromStr;
use std::sync::Arc;

// Re-export types that flutter_rust_bridge needs
pub use ark_client::Client;
pub use ark_bdk_wallet::Wallet;


pub struct ArkClientConfig {
    pub nsec: String,
    pub network: String,
    pub esplora: String,
    pub server: String,
}


#[derive(Clone)]
pub struct ArkClient {
    pub inner: Arc<Client<EsploraClient, Wallet<InMemoryDb>>>,
}

impl ArkClient {
    pub async fn init(config: ArkClientConfig) -> Result<ArkClient> {
        let client = setup_client(config).await?;
        Ok(client)
    }
}


pub async fn setup_client(
    config: ArkClientConfig,
) -> Result<ArkClient> {    
    let network = Network::from_str(config.network.as_str())?;
    let secp = Secp256k1::new();
    let keys = Keys::parse(config.nsec.as_str())?;
    let kp = *keys.key_pair(&secp);
    let esplora = config.esplora;
    let server = config.server;

    let db = InMemoryDb::default();
    let wallet = ark_bdk_wallet::Wallet::new(kp, secp, network, esplora.as_str(), db)?;
    let wallet = Arc::new(wallet);

    let esplora = EsploraClient::new(esplora.as_str())?;
    esplora.check_connection().await?;

    let client = OfflineClient::new(
        "winston".to_string(),
        kp,
        Arc::new(esplora),
        wallet.clone(),
        server,
    )
    .connect()
    .await
    .map_err(|err| anyhow!(err))?;

    Ok(ArkClient { inner: Arc::new(client) })
}