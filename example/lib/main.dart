import 'package:ark_wallet_example/send_page.dart';
import 'package:flutter/material.dart';
import 'package:ark_wallet/ark_wallet.dart' as ark;

Future<void> main() async {
  await ark.LibArk.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(home: HomePage());
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  late ark.ArkClient _client;
  bool _isLoading = false;
  String _arkAddress = '';
  String _btcAddress = '';
  int _balancePending = 0;
  int _balanceConfirmed = 0;
  int _balanceTotal = 0;
  List<ark.Transaction> _txs = [];

  @override
  void initState() {
    super.initState();
    test();
  }

  void test() async {
    try {
      setState(() => _isLoading = true);

      _client = await ark.ArkClient.init(
        config: ark.ArkClientConfig(
          nsec:
              "nsec1cz2uryzw7j20wdwgh94xua8nt3ch9xcuvlkg85zalg2c336ahjdq6nu2qx",
          network: "signet",
          esplora: "https://mutinynet.com/api",
          server: "https://mutinynet.arkade.sh",
        ),
      );

      _txs = await _client.fetchTransactions();

      final balance = await _client.balance();
      _balancePending = balance.pending.toInt();
      _balanceConfirmed = balance.confirmed.toInt();
      _balanceTotal = balance.total.toInt();

      final addresses = _client.addresses();
      _arkAddress = addresses.offchain;
      _btcAddress = addresses.boarding;

      setState(() => _isLoading = false);

      print(
        'is ark address: ${await ark.Utils.isArkAddress(address: _arkAddress)}',
      );
      print(
        'is btc address: ${await ark.Utils.isBtcAddress(address: _btcAddress)}',
      );
    } catch (e) {
      print(e);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('ark wallet')),
      body: CustomScrollView(
        slivers: [
          if (_isLoading)
            const SliverToBoxAdapter(
              child: Center(child: CircularProgressIndicator()),
            ),
          SliverList(
            delegate: SliverChildListDelegate([
              ListTile(
                title: const Text('Ark Address'),
                subtitle: SelectableText(_arkAddress),
              ),
              ListTile(
                title: const Text('BTC Address'),
                subtitle: SelectableText(_btcAddress),
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
              const ListTile(title: Text('Transactions')),
            ]),
          ),
          SliverList(
            delegate: SliverChildBuilderDelegate((context, index) {
              final tx = _txs[index];
              return ListTile(
                title: Text('${tx.amountSats} sats'),
                subtitle: Text(tx.txid),
                trailing: Text(tx.runtimeType.toString()),
              );
            }, childCount: _txs.length),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          Navigator.push(
            context,
            MaterialPageRoute(builder: (context) => SendPage(client: _client)),
          );
        },
        child: const Icon(Icons.send),
      ),
    );
  }
}
