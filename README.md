


# init

```dart
void main(){
    initLib();
}
```

# Monitor keyboard events


```dart
startListenSystemEvent(); 
```

# full example 

```dart
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:ldd_system_listen/api/syste.dart';

import 'package:ldd_system_listen/ldd_system_listen.dart';

void main() {
  initLib();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  void initState() {
    super.initState();
    Future.delayed(const Duration(seconds: 1), _startListen);
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('example'),
        ),
        body: const SingleChildScrollView(
          child: Column(
            children: [],
          ),
        ),
      ),
    );
  }

  FutureOr _startListen() async {
    startListenSystemEvent().listen(onData);
  }

  void onData(LddEvent event) {
    print("name:${event.name} event ${event.eventType}");
  }
}

```