// use crate::ark::client::ArkWallet;
// use anyhow::{anyhow, Result};
// use bitcoin::{OutPoint, Transaction};

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

// pub struct VtxoTreeInfo {
//     pub vtxo_outpoint: String,
//     pub tree_height: usize,
//     pub total_vtxos: usize,
//     pub spendable_vtxos: usize,
//     pub spent_vtxos: usize,
//     pub tree_structure: Vec<String>, // Tree path as string representation
// }

// pub struct PresignedTransaction {
//     pub transaction_hex: String,
//     pub txid: String,
//     pub vtxo_outpoints: Vec<String>,
//     pub total_amount: i64,
//     pub fee_amount: i64,
// }

// pub struct UnilateralExitInfo {
//     pub vtxos: Vec<VtxoInfo>,
//     pub presigned_tx: Option<PresignedTransaction>,
//     pub can_exit: bool,
//     pub exit_delay: u32,
// }

// pub struct TransactionSignatureInfo {
//     pub branch_index: usize,
//     pub tx_index: usize,
//     pub txid: String,
//     pub is_signed: bool,
//     pub input_count: usize,
//     pub output_count: usize,
//     pub total_size: usize,
// }

// pub struct TransactionInfo {
//     pub txid: String,
//     pub is_signed: bool,
//     pub input_count: usize,
//     pub output_count: usize,
//     pub total_size: usize,
// }

// pub struct VtxoChainInfo {
//     pub outpoint: OutPoint,
//     pub chain_height: usize,
//     pub transaction_count: usize,
//     pub transactions: Vec<TransactionInfo>,
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

//     pub async fn get_presigned_transactions(&self) -> Result<Vec<Vec<Transaction>>> {
//         let presigned_transactions = self
//             .inner
//             .build_unilateral_exit_trees()
//             .await
//             .map_err(|e| anyhow!("Failed to build unilateral exit trees: {e:#}"))?;
//         Ok(presigned_transactions)
//     }
// }
