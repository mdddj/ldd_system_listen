import 'dart:async';

import 'package:flutter/material.dart';
import 'package:ldd_system_listen/api/entitys.dart';
import 'package:ldd_system_listen/api/multiinput.dart';
import 'package:ldd_system_listen/ldd_system_listen.dart';

Future<void> main() async {
  await initLddSystenListen();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final keyboardManager =
      LddKeyboardManager(printDebug: true, gunAddEndReturnKey: true); //初始化键盘管理器
  List<LddKeyboard> keyboards = []; //电脑连接的设备列表
  StreamSubscription<LddRawEvent>? stream; //监听键盘事件流
  @override
  void initState() {
    super.initState();
    Future.microtask(init);
  }

  ///初始化键盘管理器
  void init() {
    keyboardManager.register(); //注册键盘监听事件
    keyboards = keyboardManager.getLddKeyboardList(); //获取键盘设备列表，包括传统键盘，扫码枪等特殊键盘
    setState(() {}); //刷新ui

    stream ??= keyboardManager
        .listenLddKeyboardEvent(gunDevice: keyboards.last)
        .listen(onKeyboardListen);
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text("键盘测试"),
        ),
        body: SingleChildScrollView(
          child: Column(
            children: [
              if (keyboards.isEmpty) const Text('没有检测到键盘设备'),
              ...keyboards.map((item) {
                return Card(
                  child: ListTile(
                    title: Text(item.name),
                    subtitle: Text(item.serial ?? "-"),
                  ),
                );
              })
            ],
          ),
        ),
      ),
    );
  }

  ///监听原始键盘事件
  void onKeyboardListen(LddRawEvent event) {
    event.printInfo();
  }

  @override
  void dispose() {
    //销毁监听
    stream?.cancel();
    //释放内存
    keyboardManager.dispose();
    super.dispose();
  }
}
