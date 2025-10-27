// use crate::ark::client::ArkWallet;
// use anyhow::{anyhow, Result};
// use bitcoin::OutPoint;

// pub struct VtxoInfo {
//     pub outpoint: OutPoint,
//     pub amount: i64,
//     pub is_spent: bool,
//     pub confirmation_time: Option<i64>,
//     pub created_at: i64,
//     pub expires_at: i64,
//     pub script: String,
//     pub is_preconfirmed: bool,
//     pub is_swept: bool,
//     pub is_unrolled: bool,
// }

// impl ArkWallet {
//     pub async fn send_unilateral_exit(&self, _address: String, amount_sats: u64) -> Result<String> {
//         use bitcoin::{Address, Amount};
//         use std::str::FromStr;

//         let destination_address = Address::from_str(&_address).unwrap().assume_checked();

//         let txid = self
//             .inner
//             .send_on_chain(destination_address, Amount::from_sat(amount_sats))
//             .await
//             .map_err(|e| anyhow!("Failed to send coins to wallet: {e:#}"))?;

//         println!("Coins finally in your wallet! TXID: {}", txid);
//         Ok(txid.to_string())
//     }

//     pub async fn get_vtxos(&self) -> Result<Vec<VtxoInfo>> {
//         let mut vtxos = Vec::new();

//         // Get spendable VTXOs from the ark-client
//         let spendable_vtxos = self
//             .inner
//             .spendable_vtxos(false) // select_recoverable_vtxos = false
//             .await
//             .map_err(|e| anyhow!("Failed to get spendable VTXOs: {e:#}"))?;

//         for (virtual_tx_outpoints, _) in spendable_vtxos {
//             for virtual_tx_outpoint in virtual_tx_outpoints {
//                 vtxos.push(VtxoInfo {
//                     outpoint: virtual_tx_outpoint.outpoint,
//                     amount: virtual_tx_outpoint.amount.to_sat() as i64,
//                     is_spent: virtual_tx_outpoint.is_spent,
//                     confirmation_time: None,
//                     created_at: virtual_tx_outpoint.created_at,
//                     expires_at: virtual_tx_outpoint.expires_at,
//                     script: virtual_tx_outpoint.script.to_hex_string(),
//                     is_preconfirmed: virtual_tx_outpoint.is_preconfirmed,
//                     is_swept: virtual_tx_outpoint.is_swept,
//                     is_unrolled: virtual_tx_outpoint.is_unrolled,
//                 });
//             }
//         }

//         Ok(vtxos)
//     }
// }
