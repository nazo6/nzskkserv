import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import "ffi.dart";

void main() {
  runApp(const App());
}

class App extends StatelessWidget {
  const App({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return FluentApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        accentColor: Colors.red,
      ),
      home: HomePage(),
    );
  }
}

class HomePage extends HookConsumerWidget {
  const HomePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final t = useState("");
    final paneIndex = useState(0);

    return NavigationView(
      appBar: const NavigationAppBar(
        title: Text("title"),
        automaticallyImplyLeading: true,
      ),
      pane: NavigationPane(
        selected: paneIndex.value,
        onChanged: (i) => paneIndex.value = i,
        displayMode: PaneDisplayMode.auto,
        items: [
          PaneItem(
            icon: const Icon(FluentIcons.bar_chart_vertical),
          ),
        ],
      ),
      content: NavigationBody(
        index: paneIndex.value,
        children: [
          Container(
            alignment: Alignment.center,
            child: Column(
              children: [
                Button(
                  child: const Text("Get"),
                  onPressed: () async {
                    final a = await api.greet();
                    t.value = a;
                  },
                ),
                Text(t.value)
              ],
            ),
          ),
        ],
      ),
    );
  }
}
