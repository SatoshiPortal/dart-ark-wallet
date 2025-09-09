import 'package:flutter/material.dart';
import 'package:ark_wallet/ark_wallet.dart';

Future<void> main() async {
  await LibArk.init();
  runApp(const MyApp());
}

void test() async {
  try {
    final client = await ArkClient.init(
      config: ArkClientConfig(
        nsec: "nsec1cz2uryzw7j20wdwgh94xua8nt3ch9xcuvlkg85zalg2c336ahjdq6nu2qx",
        network: "signet",
        esplora: "https://mutinynet.com/api",
        server: "https://mutinynet.arkade.sh",
      ),
    );

    final txs = await client.fetchTransactions();
    print("Tx history: $txs");

    final balance = await client.balance();
    print("Balance pending  : ${balance.pending}");
    print("Balance confirmed: ${balance.confirmed}");
    print("Balance total    : ${balance.total}");

    final addresses = await client.addresses();
    print("Addresses: ${addresses.boarding} ${addresses.offchain}");
  } catch (e) {
    print(e);
  }
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    test();

    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(child: Text('Action: Call Rust `greet("Tom")`\nResult:')),
      ),
    );
  }
}
