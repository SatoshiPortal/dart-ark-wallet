use crate::ark::client::ArkWallet;
use anyhow::{anyhow, Result};

pub use rand::rngs::StdRng;
pub use rand::SeedableRng;

pub struct BoardingSettlement {
    pub pending_count: i32,
    pub confirmed_count: i32,
    pub total_pending_sats: i64,
    pub total_confirmed_sats: i64,
}

impl ArkWallet {
    pub async fn settle(&self, select_recoverable_vtxos: bool) -> Result<()> {
        let mut rng = StdRng::from_entropy();
        self.inner
            .settle(&mut rng, select_recoverable_vtxos)
            .await
            .map_err(|e| anyhow!("Failed to settle: {e:#}"))?;
        Ok(())
    }

    pub async fn get_boarding_status(&self) -> Result<BoardingSettlement> {
        let txs = self
            .inner
            .transaction_history()
            .await
            .map_err(|error| anyhow!("Failed getting transaction history {error:#}"))?;

        let mut pending_count = 0i32;
        let mut confirmed_count = 0i32;
        let mut total_pending_sats = 0i64;
        let mut total_confirmed_sats = 0i64;

        for tx in txs {
            if let ark_core::history::Transaction::Boarding {
                amount,
                confirmed_at,
                ..
            } = tx
            {
                let sats = amount.to_sat() as i64;
                if confirmed_at.is_some() {
                    confirmed_count += 1;
                    total_confirmed_sats += sats;
                } else {
                    pending_count += 1;
                    total_pending_sats += sats;
                }
            }
        }

        Ok(BoardingSettlement {
            pending_count,
            confirmed_count,
            total_pending_sats,
            total_confirmed_sats,
        })
    }

    pub async fn settle_boarding_transactions(
        &self,
        select_recoverable_vtxos: bool,
    ) -> Result<BoardingSettlement> {
        let status_before = self.get_boarding_status().await?;

        if status_before.confirmed_count == 0 {
            return Ok(status_before);
        }

        self.settle(select_recoverable_vtxos).await?;

        let status_after = self.get_boarding_status().await?;
        Ok(status_after)
    }

    pub async fn can_settle_boarding(&self) -> Result<bool> {
        let status = self.get_boarding_status().await?;
        Ok(status.confirmed_count > 0)
    }
}
