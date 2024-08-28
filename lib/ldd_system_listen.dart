import 'package:ldd_system_listen/api/entitys.dart';
import 'package:ldd_system_listen/frb_generated.dart';

///初始化lib
Future<void> initLddSystenListen() async {
  await RustLib.init();
}

extension LddRawEventEx on LddRawEvent {
  void printInfo() {
    when(
      lddKeyboardEvent: (field0, field1, field2, field3) {
        print("普通键盘事件:${field1.keyIdToString()},类型:${field2.formatString()}");
      },
      scanGunEvent: (field0, field1) {
        print("扫码枪事件:${field0.map((item) => item.keyIdToString())}");
      },
    );
  }
}
