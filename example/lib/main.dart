import 'package:flutter/material.dart';
import 'package:ark_wallet/ark_wallet.dart';

Future<void> main() async {
  await LibArk.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  bool _isLoading = false;
  String _arkAddress = '';
  String _btcAddress = '';
  int _balancePending = 0;
  int _balanceConfirmed = 0;
  int _balanceTotal = 0;
  List<Transaction> _txs = [];

  @override
  void initState() {
    super.initState();
    test();
  }

  void test() async {
    try {
      setState(() => _isLoading = true);

      final client = await ArkClient.init(
        config: ArkClientConfig(
          nsec:
              "nsec1cz2uryzw7j20wdwgh94xua8nt3ch9xcuvlkg85zalg2c336ahjdq6nu2qx",
          network: "signet",
          esplora: "https://mutinynet.com/api",
          server: "https://mutinynet.arkade.sh",
        ),
      );

      _txs = await client.fetchTransactions();

      final balance = await client.balance();
      _balancePending = balance.pending.toInt();
      _balanceConfirmed = balance.confirmed.toInt();
      _balanceTotal = balance.total.toInt();

      final addresses = await client.addresses();
      _arkAddress = addresses.boarding;
      _btcAddress = addresses.offchain;

      setState(() => _isLoading = false);
    } catch (e) {
      print(e);
    }
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('ark wallet')),
        body: Column(
          children: [
            if (_isLoading) const Center(child: CircularProgressIndicator()),
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                ListTile(
                  title: const Text('Ark Address'),
                  subtitle: Text(_arkAddress),
                ),
                ListTile(
                  title: const Text('BTC Address'),
                  subtitle: Text(_btcAddress),
                ),
                ListTile(
                  title: const Text('Balance Pending'),
                  subtitle: Text('$_balancePending sats'),
                ),
                ListTile(
                  title: const Text('Balance Confirmed'),
                  subtitle: Text('$_balanceConfirmed sats'),
                ),
                ListTile(
                  title: const Text('Balance Total'),
                  subtitle: Text('$_balanceTotal sats'),
                ),
              ],
            ),

            Expanded(
              child: ListView.builder(
                itemCount: _txs.length,
                itemBuilder: (context, index) {
                  final tx = _txs[index];

                  return ListTile(
                    title: Text('${tx.amountSats} sats'),
                    subtitle: Text(tx.txid),
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}
