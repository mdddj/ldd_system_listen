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
  StreamSubscription<LddKeyboardValue>? stream;

  @override
  void initState() {
    super.initState();
    Future.delayed(const Duration(seconds: 1), _startListen);
  }

  FutureOr _startListen() async {
    stream = startListenSystenEventByLdd().listen(onScanCode);
  }

  void disponseListen() {
    stream?.cancel();
  }

  void onScanCode(LddKeyboardValue event) {}

  @override
  void dispose() {
    disponseListen();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Native Packages'),
        ),
        body: const SingleChildScrollView(
          child: Column(
            children: [],
          ),
        ),
      ),
    );
  }
}
