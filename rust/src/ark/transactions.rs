use crate::ark::client::ArkWallet;
use anyhow::{Result, anyhow};

pub enum Transaction {
    Boarding {
         txid: String,
         sats: i64,
         confirmed_at: Option<i64>,
     },
      Commitment {
         txid: String,
         sats: i64,
         created_at: i64,
     },
      Redeem {
         txid: String,
         sats: i64,
         is_settled: bool,
         created_at: i64,
     },
}


impl ArkWallet {
    pub async fn transaction_history(
        &self,
    ) -> Result<Vec<Transaction>> {
        let mut txs = self.inner
            .transaction_history()
            .await
            .map_err(|error| anyhow!("Failed getting transaction history {error:#}"))?;
    
        // sort desc, i.e. newest transactions first
        txs.sort_by_key(|b| std::cmp::Reverse(b.created_at()));
    
    
        let result = txs
            .into_iter()
            .map(|tx| match tx {
                ark_core::history::Transaction::Boarding {
                    txid,
                    amount,
                    confirmed_at,
                } => Transaction::Boarding{
                    txid: txid.to_string(),
                    sats: amount.to_sat() as i64,
                    confirmed_at: confirmed_at.map(|ts| ts),
                },
                ark_core::history::Transaction::Commitment {
                    txid,
                    amount,
                    created_at,
                } => Transaction::Commitment{
                    txid: txid.to_string(),
                    sats: amount.to_sat() as i64,
created_at,
                },
                ark_core::history::Transaction::Ark {
                    txid,
                    amount,
                    is_settled,
                    created_at,
                } => Transaction::Redeem{
                    txid: txid.to_string(),
                    sats: amount.to_sat(),
                    is_settled,
                    created_at,
                },
            })
            .collect();
    
        Ok(result)
    }
}

