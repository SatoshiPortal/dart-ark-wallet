import 'package:convert/convert.dart';
import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ark_wallet/ark_wallet.dart' as ark;

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('ARK Package Data Types Tests', () {
    late ark.ArkWallet client;

    setUpAll(() async {
      await ark.LibArk.init();

      client = await ark.ArkWallet.init(
        secretKey: hex.decode(
          "c095c1904ef494f735c8b96a6e74f35c71729b1c67ec83d05dfa1588c75dbc9a",
        ),
        network: "signet",
        esplora: "https://mutinynet.com/api",
        server: "https://mutinynet.arkade.sh",
        boltz: "https://api.boltz.exchange",
      );
    });

    test('Balance object has correct properties', () async {
      final balance = await client.balance();

      // Check that balance object has all expected properties
      expect(balance.pending, isA<int>());
      expect(balance.confirmed, isA<int>());
      expect(balance.total, isA<int>());

      // Check that properties are accessible
      final pending = balance.pending;
      final confirmed = balance.confirmed;
      final total = balance.total;

      expect(pending, isNotNull);
      expect(confirmed, isNotNull);
      expect(total, isNotNull);
    });

    test('Transaction objects have correct structure', () async {
      final transactions = await client.transactionHistory();

      expect(transactions, isA<List<ark.Transaction>>());

      for (final tx in transactions) {
        switch (tx) {
          case ark.Transaction_Boarding tx:
            expect(tx.sats, isA<int>());
            expect(tx.confirmedAt, isA<int>());
            expect(tx.txid, isA<String>());
            break;
          case ark.Transaction_Commitment tx:
            expect(tx.sats, isA<int>());
            expect(tx.createdAt, isA<int>());
            expect(tx.txid, isA<String>());
            break;
          case ark.Transaction_Redeem tx:
            expect(tx.sats, isA<int>());
            expect(tx.isSettled, isA<bool>());
            expect(tx.createdAt, isA<int>());
            expect(tx.txid, isA<String>());
            break;
        }
      }
    });

    test('ServerInfo has all required string fields', () {
      final serverInfo = client.serverInfo();

      // Test string fields
      expect(serverInfo.network, isA<String>());
      expect(serverInfo.version, isA<String>());
      expect(serverInfo.signerPubkey, isA<String>());
      expect(serverInfo.forfeitPubkey, isA<String>());
      expect(serverInfo.forfeitAddress, isA<String>());
      expect(serverInfo.checkpointTapscript, isA<String>());
      expect(serverInfo.digest, isA<String>());

      // Test that string fields are not empty
      expect(serverInfo.network, isNotEmpty);
      expect(serverInfo.signerPubkey, isNotEmpty);
      expect(serverInfo.forfeitPubkey, isNotEmpty);
      expect(serverInfo.forfeitAddress, isNotEmpty);
      expect(serverInfo.checkpointTapscript, isNotEmpty);
      expect(serverInfo.digest, isNotEmpty);

      // Test numeric fields
      expect(serverInfo.dust, isA<int>());
      expect(serverInfo.sessionDuration, isA<int>());
      expect(serverInfo.boardingExitDelay, isA<int>());
      expect(serverInfo.unilateralExitDelay, isA<int>());
      expect(serverInfo.utxoMinAmount, isA<int?>());
      expect(serverInfo.utxoMaxAmount, isA<int?>());
      expect(serverInfo.vtxoMinAmount, isA<int?>());
      expect(serverInfo.vtxoMaxAmount, isA<int?>());
    });

    test('ARK addresses have expected format', () {
      final arkAddress = client.offchainAddress();

      expect(arkAddress, isA<String>());
      expect(arkAddress, isNotEmpty);
      expect(ark.Utils.isArk(address: arkAddress), true);
    });

    test('BTC addresses have expected format', () {
      final btcAddress = client.boardingAddress();

      expect(btcAddress, isA<String>());
      expect(btcAddress, isNotEmpty);
      expect(ark.Utils.isBtc(address: btcAddress), true);
    });
  });
}
