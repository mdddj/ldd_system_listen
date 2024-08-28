// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'entitys.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$LddRawEvent {
  Object get field0 => throw _privateConstructorUsedError;
  Object get field1 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            BigInt field0, LddKeyId field1, LddState field2, LddKeyboard field3)
        lddKeyboardEvent,
    required TResult Function(List<LddKeyId> field0, LddKeyboard field1)
        scanGunEvent,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt field0, LddKeyId field1, LddState field2,
            LddKeyboard field3)?
        lddKeyboardEvent,
    TResult? Function(List<LddKeyId> field0, LddKeyboard field1)? scanGunEvent,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt field0, LddKeyId field1, LddState field2,
            LddKeyboard field3)?
        lddKeyboardEvent,
    TResult Function(List<LddKeyId> field0, LddKeyboard field1)? scanGunEvent,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LddRawEvent_LddKeyboardEvent value)
        lddKeyboardEvent,
    required TResult Function(LddRawEvent_ScanGunEvent value) scanGunEvent,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LddRawEvent_LddKeyboardEvent value)? lddKeyboardEvent,
    TResult? Function(LddRawEvent_ScanGunEvent value)? scanGunEvent,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LddRawEvent_LddKeyboardEvent value)? lddKeyboardEvent,
    TResult Function(LddRawEvent_ScanGunEvent value)? scanGunEvent,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LddRawEventCopyWith<$Res> {
  factory $LddRawEventCopyWith(
          LddRawEvent value, $Res Function(LddRawEvent) then) =
      _$LddRawEventCopyWithImpl<$Res, LddRawEvent>;
}

/// @nodoc
class _$LddRawEventCopyWithImpl<$Res, $Val extends LddRawEvent>
    implements $LddRawEventCopyWith<$Res> {
  _$LddRawEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of LddRawEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$LddRawEvent_LddKeyboardEventImplCopyWith<$Res> {
  factory _$$LddRawEvent_LddKeyboardEventImplCopyWith(
          _$LddRawEvent_LddKeyboardEventImpl value,
          $Res Function(_$LddRawEvent_LddKeyboardEventImpl) then) =
      __$$LddRawEvent_LddKeyboardEventImplCopyWithImpl<$Res>;
  @useResult
  $Res call(
      {BigInt field0, LddKeyId field1, LddState field2, LddKeyboard field3});
}

/// @nodoc
class __$$LddRawEvent_LddKeyboardEventImplCopyWithImpl<$Res>
    extends _$LddRawEventCopyWithImpl<$Res, _$LddRawEvent_LddKeyboardEventImpl>
    implements _$$LddRawEvent_LddKeyboardEventImplCopyWith<$Res> {
  __$$LddRawEvent_LddKeyboardEventImplCopyWithImpl(
      _$LddRawEvent_LddKeyboardEventImpl _value,
      $Res Function(_$LddRawEvent_LddKeyboardEventImpl) _then)
      : super(_value, _then);

  /// Create a copy of LddRawEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
    Object? field2 = null,
    Object? field3 = null,
  }) {
    return _then(_$LddRawEvent_LddKeyboardEventImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BigInt,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as LddKeyId,
      null == field2
          ? _value.field2
          : field2 // ignore: cast_nullable_to_non_nullable
              as LddState,
      null == field3
          ? _value.field3
          : field3 // ignore: cast_nullable_to_non_nullable
              as LddKeyboard,
    ));
  }
}

/// @nodoc

class _$LddRawEvent_LddKeyboardEventImpl extends LddRawEvent_LddKeyboardEvent {
  const _$LddRawEvent_LddKeyboardEventImpl(
      this.field0, this.field1, this.field2, this.field3)
      : super._();

  @override
  final BigInt field0;
  @override
  final LddKeyId field1;
  @override
  final LddState field2;
  @override
  final LddKeyboard field3;

  @override
  String toString() {
    return 'LddRawEvent.lddKeyboardEvent(field0: $field0, field1: $field1, field2: $field2, field3: $field3)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LddRawEvent_LddKeyboardEventImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1) &&
            (identical(other.field2, field2) || other.field2 == field2) &&
            (identical(other.field3, field3) || other.field3 == field3));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1, field2, field3);

  /// Create a copy of LddRawEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LddRawEvent_LddKeyboardEventImplCopyWith<
          _$LddRawEvent_LddKeyboardEventImpl>
      get copyWith => __$$LddRawEvent_LddKeyboardEventImplCopyWithImpl<
          _$LddRawEvent_LddKeyboardEventImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            BigInt field0, LddKeyId field1, LddState field2, LddKeyboard field3)
        lddKeyboardEvent,
    required TResult Function(List<LddKeyId> field0, LddKeyboard field1)
        scanGunEvent,
  }) {
    return lddKeyboardEvent(field0, field1, field2, field3);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt field0, LddKeyId field1, LddState field2,
            LddKeyboard field3)?
        lddKeyboardEvent,
    TResult? Function(List<LddKeyId> field0, LddKeyboard field1)? scanGunEvent,
  }) {
    return lddKeyboardEvent?.call(field0, field1, field2, field3);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt field0, LddKeyId field1, LddState field2,
            LddKeyboard field3)?
        lddKeyboardEvent,
    TResult Function(List<LddKeyId> field0, LddKeyboard field1)? scanGunEvent,
    required TResult orElse(),
  }) {
    if (lddKeyboardEvent != null) {
      return lddKeyboardEvent(field0, field1, field2, field3);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LddRawEvent_LddKeyboardEvent value)
        lddKeyboardEvent,
    required TResult Function(LddRawEvent_ScanGunEvent value) scanGunEvent,
  }) {
    return lddKeyboardEvent(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LddRawEvent_LddKeyboardEvent value)? lddKeyboardEvent,
    TResult? Function(LddRawEvent_ScanGunEvent value)? scanGunEvent,
  }) {
    return lddKeyboardEvent?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LddRawEvent_LddKeyboardEvent value)? lddKeyboardEvent,
    TResult Function(LddRawEvent_ScanGunEvent value)? scanGunEvent,
    required TResult orElse(),
  }) {
    if (lddKeyboardEvent != null) {
      return lddKeyboardEvent(this);
    }
    return orElse();
  }
}

abstract class LddRawEvent_LddKeyboardEvent extends LddRawEvent {
  const factory LddRawEvent_LddKeyboardEvent(
      final BigInt field0,
      final LddKeyId field1,
      final LddState field2,
      final LddKeyboard field3) = _$LddRawEvent_LddKeyboardEventImpl;
  const LddRawEvent_LddKeyboardEvent._() : super._();

  @override
  BigInt get field0;
  @override
  LddKeyId get field1;
  LddState get field2;
  LddKeyboard get field3;

  /// Create a copy of LddRawEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LddRawEvent_LddKeyboardEventImplCopyWith<
          _$LddRawEvent_LddKeyboardEventImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LddRawEvent_ScanGunEventImplCopyWith<$Res> {
  factory _$$LddRawEvent_ScanGunEventImplCopyWith(
          _$LddRawEvent_ScanGunEventImpl value,
          $Res Function(_$LddRawEvent_ScanGunEventImpl) then) =
      __$$LddRawEvent_ScanGunEventImplCopyWithImpl<$Res>;
  @useResult
  $Res call({List<LddKeyId> field0, LddKeyboard field1});
}

/// @nodoc
class __$$LddRawEvent_ScanGunEventImplCopyWithImpl<$Res>
    extends _$LddRawEventCopyWithImpl<$Res, _$LddRawEvent_ScanGunEventImpl>
    implements _$$LddRawEvent_ScanGunEventImplCopyWith<$Res> {
  __$$LddRawEvent_ScanGunEventImplCopyWithImpl(
      _$LddRawEvent_ScanGunEventImpl _value,
      $Res Function(_$LddRawEvent_ScanGunEventImpl) _then)
      : super(_value, _then);

  /// Create a copy of LddRawEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
    Object? field1 = null,
  }) {
    return _then(_$LddRawEvent_ScanGunEventImpl(
      null == field0
          ? _value._field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as List<LddKeyId>,
      null == field1
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as LddKeyboard,
    ));
  }
}

/// @nodoc

class _$LddRawEvent_ScanGunEventImpl extends LddRawEvent_ScanGunEvent {
  const _$LddRawEvent_ScanGunEventImpl(final List<LddKeyId> field0, this.field1)
      : _field0 = field0,
        super._();

  final List<LddKeyId> _field0;
  @override
  List<LddKeyId> get field0 {
    if (_field0 is EqualUnmodifiableListView) return _field0;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_field0);
  }

  @override
  final LddKeyboard field1;

  @override
  String toString() {
    return 'LddRawEvent.scanGunEvent(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LddRawEvent_ScanGunEventImpl &&
            const DeepCollectionEquality().equals(other._field0, _field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(_field0), field1);

  /// Create a copy of LddRawEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$LddRawEvent_ScanGunEventImplCopyWith<_$LddRawEvent_ScanGunEventImpl>
      get copyWith => __$$LddRawEvent_ScanGunEventImplCopyWithImpl<
          _$LddRawEvent_ScanGunEventImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(
            BigInt field0, LddKeyId field1, LddState field2, LddKeyboard field3)
        lddKeyboardEvent,
    required TResult Function(List<LddKeyId> field0, LddKeyboard field1)
        scanGunEvent,
  }) {
    return scanGunEvent(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt field0, LddKeyId field1, LddState field2,
            LddKeyboard field3)?
        lddKeyboardEvent,
    TResult? Function(List<LddKeyId> field0, LddKeyboard field1)? scanGunEvent,
  }) {
    return scanGunEvent?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt field0, LddKeyId field1, LddState field2,
            LddKeyboard field3)?
        lddKeyboardEvent,
    TResult Function(List<LddKeyId> field0, LddKeyboard field1)? scanGunEvent,
    required TResult orElse(),
  }) {
    if (scanGunEvent != null) {
      return scanGunEvent(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(LddRawEvent_LddKeyboardEvent value)
        lddKeyboardEvent,
    required TResult Function(LddRawEvent_ScanGunEvent value) scanGunEvent,
  }) {
    return scanGunEvent(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(LddRawEvent_LddKeyboardEvent value)? lddKeyboardEvent,
    TResult? Function(LddRawEvent_ScanGunEvent value)? scanGunEvent,
  }) {
    return scanGunEvent?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(LddRawEvent_LddKeyboardEvent value)? lddKeyboardEvent,
    TResult Function(LddRawEvent_ScanGunEvent value)? scanGunEvent,
    required TResult orElse(),
  }) {
    if (scanGunEvent != null) {
      return scanGunEvent(this);
    }
    return orElse();
  }
}

abstract class LddRawEvent_ScanGunEvent extends LddRawEvent {
  const factory LddRawEvent_ScanGunEvent(
          final List<LddKeyId> field0, final LddKeyboard field1) =
      _$LddRawEvent_ScanGunEventImpl;
  const LddRawEvent_ScanGunEvent._() : super._();

  @override
  List<LddKeyId> get field0;
  @override
  LddKeyboard get field1;

  /// Create a copy of LddRawEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$LddRawEvent_ScanGunEventImplCopyWith<_$LddRawEvent_ScanGunEventImpl>
      get copyWith => throw _privateConstructorUsedError;
}
