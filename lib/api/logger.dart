// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.1.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'entitys.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `record_to_entry`
// These types are ignored because they are not used by any `pub` functions: `SEND_TO_DART_LOGGER_STREAM_SINK`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `as_log`, `as_log`, `config`, `config`, `deref`, `enabled`, `enabled`, `flush`, `flush`, `initialize`, `level`, `level`, `log`, `log`

Stream<LogEntry> createLogStream() =>
    RustLib.instance.api.crateApiLoggerCreateLogStream();

Future<void> rustSetUp() => RustLib.instance.api.crateApiLoggerRustSetUp();

Future<void> initLogger() => RustLib.instance.api.crateApiLoggerInitLogger();

class MyMobileLogger {
  const MyMobileLogger();

  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.
  static Future<MyMobileLogger> newInstance() =>
      RustLib.instance.api.crateApiLoggerMyMobileLoggerNew();

  @override
  int get hashCode => 0;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MyMobileLogger && runtimeType == other.runtimeType;
}

class SendToDartLogger {
  const SendToDartLogger();

  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.
  static Future<SendToDartLogger> newInstance() =>
      RustLib.instance.api.crateApiLoggerSendToDartLoggerNew();

  static Stream<LogEntry> setStreamSink() =>
      RustLib.instance.api.crateApiLoggerSendToDartLoggerSetStreamSink();

  @override
  int get hashCode => 0;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SendToDartLogger && runtimeType == other.runtimeType;
}
