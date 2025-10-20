# ARK Wallet

A Flutter bindings from [ArkLabsHQ/ark-flutter-example](https://github.com/ArkLabsHQ/ark-flutter-example) using `flutter_rust_bridge`

## Quick Start

```dart
import 'package:ark_wallet/ark_wallet.dart' as ark;

// Initialize the library
await ark.LibArk.init();

// Create wallet instance
final client = await ark.ArkWallet.init(
  secretKey: [], // 32-byte Uint8List
  network: "signet",        // or "mainnet"
  esplora: "https://mutinynet.com/api",
  server: "https://mutinynet.arkade.sh",
  boltz: "https://api.boltz.exchange",
);

// Get addresses
final arkAddress = client.offchainAddress();  // ARK off-chain address
final btcAddress = client.boardingAddress();  // Bitcoin boarding address

// Check balance
final balance = await client.balance();
print('Total: ${balance.total} sats');
print('Confirmed: ${balance.confirmed} sats');
print('Pending: ${balance.pending} sats');

// Send off-chain transaction
client.sendOffChain(
  address: recipientArkAddress,
  sats: amountInSatoshis,
);

// Collaborative redeem (exit to Bitcoin with server cooperation)
await client.collaborativeRedeem(
  address: bitcoinAddress,
  sats: amountInSatoshis,
  selectRecoverableVtxos: true,
);

// Get transaction history
final transactions = await client.transactionHistory();
// Returns List<Transaction> with Boarding, Commitment, and Redeem variants

// Settle transactions
client.settle(selectRecoverableVtxos: true);

// Address validation
ark.Utils.isArk(address: arkAddress);  // Validate ARK address
ark.Utils.isBtc(address: btcAddress);  // Validate Bitcoin address
```

## Rust Functions

This plugin binds the following Rust functions from ARK ecosystem crates:

| Dart Method | Rust Function | Origin Crate | Description |
|-------------|---------------|--------------|-------------|
| `ArkWallet.init()` | `OfflineClient::new().connect()` | `ark-client` | Initialize wallet client |
| `balance()` | `offchain_balance()` | `ark-client` | Get wallet balance |
| `transactionHistory()` | `transaction_history()` | `ark-client` | Fetch transaction history |
| `sendOffChain()` | `send_vtxo()` | `ark-client` | Send off-chain transaction |
| `sendOnChain()` | `send_on_chain()` | `ark-client` | Send on-chain Bitcoin transaction |
| `collaborativeRedeem()` | `collaborative_redeem()` | `ark-client` | Exit to Bitcoin with server cooperation |
| `settle()` | `settle()` | `ark-client` | Settle off-chain transactions |
| `offchainAddress()` | `get_offchain_address()` | `ark-client` | Get ARK off-chain address |
| `boardingAddress()` | `get_boarding_address()` | `ark-client` | Get Bitcoin boarding address |
| `serverInfo()` | `server_info` | `ark-client` | Get ARK server information |
| `Utils.isArk()` | `ArkAddress::decode()` | `ark-core` | Validate ARK address |
| `Utils.isBtc()` | `Address::from_str()` | `bitcoin` | Validate Bitcoin address |

**Core Dependencies:**
- `ark-client` - Main ARK protocol client
- `ark-core` - Core ARK types and utilities  
- `ark-bdk-wallet` - Bitcoin wallet functionality
- `bitcoin` - Bitcoin primitives and address handling

### Example App

The `example/` directory contains a complete Flutter app demonstrating all features:
- Wallet initialization and address display
- Balance monitoring
- Transaction history
- Send functionality with validation
- Settlement operations
