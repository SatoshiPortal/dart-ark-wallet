use crate::ark::client::ArkWallet;
use anyhow::{anyhow, Result};

pub use ark_client::OffChainBalance;

pub struct Boarding {
    pub unconfirmed: i64,
    pub confirmed: i64,
    pub total: i64,
}

pub struct Balance {
    pub preconfirmed: i64,
    pub settled: i64,
    pub available: i64,
    pub recoverable: i64,
    pub total: i64,
    pub boarding: Boarding,
}

impl ArkWallet {
    pub async fn balance(&self) -> Result<Balance> {
        let offchain_balance = self
            .inner
            .offchain_balance()
            .await
            .map_err(|error| anyhow!("Could not fetch balance {error}"))?;

        let preconfirmed = 0;
        let settled = offchain_balance.confirmed().to_sat() as i64;
        let available = settled + preconfirmed;
        let recoverable = offchain_balance.pending().to_sat() as i64;
        let total = available + recoverable;

        // Calculate boarding balance from transaction history
        let boarding_balance = self.calculate_boarding_balance().await?;

        Ok(Balance {
            preconfirmed,
            settled,
            available,
            recoverable,
            total,
            boarding: boarding_balance,
        })
    }

    async fn calculate_boarding_balance(&self) -> Result<Boarding> {
        let txs = self
            .inner
            .transaction_history()
            .await
            .map_err(|error| anyhow!("Could not fetch transaction history {error}"))?;

        let mut unconfirmed = 0i64;
        let mut confirmed = 0i64;

        for tx in txs {
            if let ark_core::history::Transaction::Boarding {
                amount,
                confirmed_at,
                ..
            } = tx
            {
                let sats = amount.to_sat() as i64;
                if confirmed_at.is_some() {
                    confirmed += sats;
                } else {
                    unconfirmed += sats;
                }
            }
        }

        Ok(Boarding {
            unconfirmed,
            confirmed,
            total: unconfirmed + confirmed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_balance_structure() {
        // This test requires a real network connection, so we'll skip it in CI
        // but it can be run manually for integration testing
        if std::env::var("CI").is_ok() {
            return;
        }

        let secret_key =
            hex::decode("66366882c0fd4f2591e46d37ffe232ff1620b1f7444fc22518be28aef9f63a59")
                .expect("Invalid hex secret key");

        let wallet = ArkWallet::init(
            secret_key,
            "signet".to_string(),
            "https://mempool.space/api".to_string(),
            "https://bitcoin-beta.arkade.sh".to_string(),
            "https://api.boltz.exchange".to_string(),
        )
        .await
        .expect("Failed to create wallet");

        let balance = wallet.balance().await.expect("Failed to get balance");

        println!("=== ARK Wallet Balance ===");
        println!("Preconfirmed: {} sats", balance.preconfirmed);
        println!("Settled: {} sats", balance.settled);
        println!("Available: {} sats", balance.available);
        println!("Recoverable: {} sats", balance.recoverable);
        println!("Total: {} sats", balance.total);
        println!("");
        println!("Boarding Balance:");
        println!("  Unconfirmed: {} sats", balance.boarding.unconfirmed);
        println!("  Confirmed: {} sats", balance.boarding.confirmed);
        println!("  Total: {} sats", balance.boarding.total);
        println!("========================");
    }
}
