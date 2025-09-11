
use crate::ark::storage::InMemoryDb;
use crate::ark::esplora::EsploraClient;
use bitcoin::secp256k1::{Secp256k1, Keypair};
use anyhow::{Result, anyhow};
use ark_client::OfflineClient;
use bitcoin::Network;
use std::str::FromStr;
use std::sync::Arc;

// Re-export types that flutter_rust_bridge needs
pub use ark_client::Client;
pub use ark_bdk_wallet::Wallet;

#[derive(Clone)]
pub struct ArkWallet {
    pub inner: Arc<Client<EsploraClient, Wallet<InMemoryDb>>>,
}


impl ArkWallet {
    pub async fn init(
        secret_key: Vec<u8>,
        network: String,
        esplora: String,
        server: String,
    ) -> Result<ArkWallet> {    
        let network = Network::from_str(network.as_str())?;
        let secp = Secp256k1::new();
        let kp = Keypair::from_seckey_slice(&secp, secret_key.as_slice())?;

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
    
        Ok(ArkWallet { inner: Arc::new(client) })
    }
}


