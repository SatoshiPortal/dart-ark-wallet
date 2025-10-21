pub mod address;
pub mod balance;
pub mod client;
pub mod esplora;
pub mod send;
pub mod server_info;
pub mod settle;
pub mod storage;
pub mod transactions;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::ark::client::ArkWallet;

    fn derive_key_from_mnemonic(mnemonic_phrase: &str) -> Vec<u8> {
        use bip39::{Language, Mnemonic};

        // Parse the mnemonic phrase
        let mnemonic = Mnemonic::parse_in(Language::English, mnemonic_phrase)
            .expect("Invalid mnemonic phrase");

        // Generate seed from mnemonic (no passphrase)
        let seed = mnemonic.to_seed("");

        // Use the first 32 bytes of the seed as the private key
        // This is a simplified approach - in production you'd want proper BIP32 derivation
        seed[..32].to_vec()
    }

    #[tokio::test]
    async fn test_ark_wallet_balance_with_mnemonic() {
        // This test requires a real network connection, so we'll skip it in CI
        if std::env::var("CI").is_ok() {
            return;
        }

        // Replace with your actual mnemonic phrase
        let mnemonic_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

        let secret_key = derive_key_from_mnemonic(mnemonic_phrase);

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

        println!("=== ARK Wallet Balance (from mnemonic) ===");
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
        println!("=========================================");
    }
}
