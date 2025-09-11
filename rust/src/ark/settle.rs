use anyhow::{Result, anyhow};

use crate::ark::client::ArkWallet;

pub use rand::rngs::StdRng;
pub use rand::SeedableRng;

impl ArkWallet {
    pub async fn settle(&self, select_recoverable_vtxos: bool) -> Result<()> {
        let mut rng = StdRng::from_entropy();
        self.inner
            .settle(&mut rng,select_recoverable_vtxos)
            .await
            .map_err(|e| anyhow!("Failed to settle: {e:#}"))?;
        Ok(())
    }
}
