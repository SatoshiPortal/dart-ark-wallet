pub mod address;
pub mod balance;
pub mod client;
pub mod esplora;
pub mod send;
pub mod server_info;
pub mod settle;
pub mod storage;
pub mod transactions;
pub mod unilateral_exit;
pub mod utils;

// #[cfg(test)]
// mod tests {
//     use crate::ark::{self, client::ArkWallet};

//     fn derive_key_from_mnemonic(mnemonic_phrase: &str) -> Vec<u8> {
//         use bip39::{Language, Mnemonic};

//         // Parse the mnemonic phrase
//         let mnemonic = Mnemonic::parse_in(Language::English, mnemonic_phrase)
//             .expect("Invalid mnemonic phrase");

//         // Generate seed from mnemonic (no passphrase)
//         let seed = mnemonic.to_seed("");

//         // Use the first 32 bytes of the seed as the private key
//         // This is a simplified approach - in production you'd want proper BIP32 derivation
//         seed[..32].to_vec()
//     }

//     #[tokio::test]
//     async fn test_ark_wallet_balance_with_mnemonic() {
//         // This test requires a real network connection, so we'll skip it in CI
//         if std::env::var("CI").is_ok() {
//             return;
//         }

//         // Replace with your actual mnemonic phrase
//         let mnemonic_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

//         let secret_key = derive_key_from_mnemonic(mnemonic_phrase);

//         let wallet = ArkWallet::init(
//             secret_key,
//             "bitcoin".to_string(),
//             "https://mempool.space/api".to_string(),
//             "https://arkade.computer".to_string(),
//             "https://api.boltz.exchange".to_string(),
//         )
//         .await
//         .expect("Failed to create wallet");

//         let balance = wallet.balance().await.expect("Failed to get balance");

//         println!("=== ARK Wallet Balance (from mnemonic) ===");
//         println!("Preconfirmed: {} sats", balance.preconfirmed);
//         println!("Settled: {} sats", balance.settled);
//         println!("Available: {} sats", balance.available);
//         println!("Recoverable: {} sats", balance.recoverable);
//         println!("Total: {} sats", balance.total);
//         println!("");
//         println!("Boarding Balance:");
//         println!("  Unconfirmed: {} sats", balance.boarding.unconfirmed);
//         println!("  Confirmed: {} sats", balance.boarding.confirmed);
//         println!("  Total: {} sats", balance.boarding.total);
//         println!("=========================================");

//         let boarding_address = wallet
//             .boarding_address()
//             .expect("Failed to get boarding address");
//         println!("Boarding Address: {}", boarding_address);

//         // Check boarding settlement status
//         let boarding_status = wallet
//             .get_boarding_status()
//             .await
//             .expect("Failed to get boarding status");

//         println!("=== Boarding Settlement Status ===");
//         println!("Pending transactions: {}", boarding_status.pending_count);
//         println!(
//             "Confirmed transactions: {}",
//             boarding_status.confirmed_count
//         );
//         println!("Total pending sats: {}", boarding_status.total_pending_sats);
//         println!(
//             "Total confirmed sats: {}",
//             boarding_status.total_confirmed_sats
//         );

//         // Settle pending boarding transactions if any exist
//         if boarding_status.confirmed_count > 0 {
//             println!(
//                 "Settling {} confirmed boarding transactions...",
//                 boarding_status.confirmed_count
//             );
//             let settlement_result = wallet
//                 .settle_boarding_transactions(true)
//                 .await
//                 .expect("Failed to settle boarding transactions");

//             println!("Settlement completed:");
//             println!("  Pending after: {}", settlement_result.pending_count);
//             println!("  Confirmed after: {}", settlement_result.confirmed_count);
//             println!(
//                 "  Confirmed amount: {}",
//                 settlement_result.total_confirmed_sats
//             );
//             println!("  Pending amount: {}", settlement_result.total_pending_sats);
//         } else {
//             println!("No confirmed boarding transactions to settle");
//         }
//         println!("==================================");

//         // Test unilateral exit functionality
//         println!("=== Unilateral Exit Test ===");
//         let vtxos = wallet.get_vtxos().await.expect("Failed to get VTXOs");

//         println!("Found {} VTXOs", vtxos.len());
//         for vtxo in &vtxos {
//             println!("  VTXO: {} - {} sats", vtxo.outpoint, vtxo.amount);
//         }

//         // Get VTXO tree information
//         // let vtxo_trees = wallet
//         //     .get_vtxo_tree_info()
//         //     .await
//         //     .expect("Failed to get VTXO tree info");

//         // println!("VTXO Tree Information:");
//         // for tree in &vtxo_trees {
//         //     println!("  VTXO: {}", tree.vtxo_outpoint);
//         //     println!("    Tree Height: {}", tree.tree_height);
//         //     println!("    Total VTXOs: {}", tree.total_vtxos);
//         //     println!("    Spendable VTXOs: {}", tree.spendable_vtxos);
//         //     println!("    Spent VTXOs: {}", tree.spent_vtxos);
//         //     println!("    Tree Structure:");
//         //     for level in &tree.tree_structure {
//         //         println!("      {}", level);
//         //     }
//         // }

//         // let max_height = wallet
//         //     .get_max_vtxo_tree_height()
//         //     .await
//         //     .expect("Failed to get max VTXO tree height");
//         // println!("Maximum VTXO Tree Height: {}", max_height);

//         if !vtxos.is_empty() {
//             // Test unilateral exit using build_unilateral_exit_trees approach
//             let exit_address = "bc1qptfvuqc6wrzj3yskv8dqjkue6924t5fxxlgq2s";

//             // Get the total available balance for unilateral exit
//             let balance = wallet.balance().await.expect("Failed to get balance");

//             println!("=== Unilateral Exit Test ===");
//             println!(
//                 "Exit address: {} (note: address not used in this approach)",
//                 exit_address
//             );
//             println!("Available balance: {} sats", balance.available);
//             println!("Settled balance: {} sats", balance.settled);
//             println!("Preconfirmed balance: {} sats", balance.preconfirmed);
//             println!("VTXOs found: {}", vtxos.len());

//             if balance.available > 0 {
//                 // Check if we have any confirmed boarding transactions (Bitcoin UTXOs) for fees
//                 if boarding_status.confirmed_count == 0 {
//                     println!("WARNING: No confirmed boarding transactions available.");
//                     println!("Unilateral exit requires Bitcoin UTXOs to pay for anchor transaction fees.");
//                     println!("You need to wait for boarding transactions to confirm first.");
//                     println!("Skipping unilateral exit test.");
//                 } else {
//                     println!("Initiating unilateral exit for all available VTXOs...");
//                     let txid = wallet
//                         .send_unilateral_exit(exit_address.to_string(), balance.available as u64)
//                         .await
//                         .expect("Failed to initiate unilateral exit");
//                     println!("Unilateral exit completed. Last transaction: {}", txid);
//                 }
//             } else {
//                 println!("No available balance for unilateral exit");
//             }
//         }

//         println!("============================");

//         /*
//         get vtxo outpoints
//         get vtxo chain(outpoints)
//         get virtual transactions
//          */
//         println!("===========TRANSACTION HISTORY=================");

//         let txs = wallet
//             .transaction_history()
//             .await
//             .expect("Failed to get transaction history");
//         for tx in txs {
//             match tx {
//                 ark::transactions::Transaction::Boarding {
//                     txid,
//                     sats,
//                     confirmed_at,
//                 } => {
//                     println!(
//                         "Boarding Transaction: {txid} {sats} sats - Confirmed: {:?}",
//                         confirmed_at
//                     );
//                 }
//                 ark::transactions::Transaction::Commitment {
//                     txid,
//                     sats,
//                     created_at,
//                 } => {
//                     println!("Commitment Transaction: {txid} {sats} sats - Created: {created_at}");
//                 }
//                 ark::transactions::Transaction::Redeem {
//                     txid,
//                     sats,
//                     is_settled,
//                     created_at,
//                 } => {
//                     println!("Redeem Transaction: {txid} {sats} sats - Settled: {is_settled} - Created: {created_at}");
//                 }
//             }
//         }
//         // println!("Transaction History: {:#?}", txs);
//     }

//     #[tokio::test]
//     async fn test_unilateral_exit_to_address() {
//         // This test requires a real network connection, so we'll skip it in CI
//         if std::env::var("CI").is_ok() {
//             return;
//         }

//         // Replace with your actual mnemonic phrase
//         let mnemonic_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

//         let secret_key = derive_key_from_mnemonic(mnemonic_phrase);

//         let wallet = ArkWallet::init(
//             secret_key,
//             "signet".to_string(),
//             "https://mempool.space/api".to_string(),
//             "https://arkade.computer".to_string(),
//             "https://api.boltz.exchange".to_string(),
//         )
//         .await
//         .expect("Failed to create wallet");

//         let exit_address = "bc1qptfvuqc6wrzj3yskv8dqjkue6924t5fxxlgq2s";

//         println!("=== Unilateral Exit to {} ===", exit_address);

//         // Get VTXOs
//         let vtxos = wallet.get_vtxos().await.expect("Failed to get VTXOs");

//         println!("Found {} VTXOs", vtxos.len());
//         for vtxo in &vtxos {
//             println!("  VTXO: {} - {} sats", vtxo.outpoint, vtxo.amount);
//         }

//         if vtxos.is_empty() {
//             println!("No VTXOs available for unilateral exit");
//             return;
//         }

//         // Create unilateral exit
//         // let exit_info = wallet
//         //     .create_unilateral_exit(exit_address.to_string())
//         //     .await
//         //     .expect("Failed to create unilateral exit info");

//         // println!("Unilateral exit info:");
//         // println!("  Can exit: {}", exit_info.can_exit);
//         // println!("  Exit delay: {} blocks", exit_info.exit_delay);

//         // if let Some(presigned_tx) = exit_info.presigned_tx {
//         //     println!("  Presigned transaction:");
//         //     println!("    TXID: {}", presigned_tx.txid);
//         //     println!("    Total amount: {} sats", presigned_tx.total_amount);
//         //     println!("    Fee amount: {} sats", presigned_tx.fee_amount);
//         //     println!("    VTXO outpoints: {:?}", presigned_tx.vtxo_outpoints);
//         //     println!("    Transaction hex: {}", presigned_tx.transaction_hex);

//         // Uncomment to actually broadcast (WARNING: Real transaction!)
//         /*
//         println!("Broadcasting unilateral exit...");
//         let txid = wallet
//             .broadcast_unilateral_exit(exit_address.to_string())
//             .await
//             .expect("Failed to broadcast unilateral exit");
//         println!("Exit transaction broadcasted with TXID: {}", txid);
//         */
//         // }

//         println!("=====================================");
//     }
// }
