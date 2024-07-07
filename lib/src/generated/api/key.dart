// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import '../lib.dart';
import 'error.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'types.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `fmt`, `from`, `from`, `from`, `from`

class BdkDerivationPath {
  final DerivationPath ptr;

  const BdkDerivationPath({
    required this.ptr,
  });

  String asString() => core.instance.api.crateApiKeyBdkDerivationPathAsString(
        that: this,
      );

  static Future<BdkDerivationPath> fromString({required String path}) =>
      core.instance.api.crateApiKeyBdkDerivationPathFromString(path: path);

  @override
  int get hashCode => ptr.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BdkDerivationPath &&
          runtimeType == other.runtimeType &&
          ptr == other.ptr;
}

class BdkDescriptorPublicKey {
  final DescriptorPublicKey ptr;

  const BdkDescriptorPublicKey({
    required this.ptr,
  });

  String asString() =>
      core.instance.api.crateApiKeyBdkDescriptorPublicKeyAsString(
        that: this,
      );

  static Future<BdkDescriptorPublicKey> derive(
          {required BdkDescriptorPublicKey ptr,
          required BdkDerivationPath path}) =>
      core.instance.api
          .crateApiKeyBdkDescriptorPublicKeyDerive(ptr: ptr, path: path);

  static Future<BdkDescriptorPublicKey> extend(
          {required BdkDescriptorPublicKey ptr,
          required BdkDerivationPath path}) =>
      core.instance.api
          .crateApiKeyBdkDescriptorPublicKeyExtend(ptr: ptr, path: path);

  static Future<BdkDescriptorPublicKey> fromString(
          {required String publicKey}) =>
      core.instance.api
          .crateApiKeyBdkDescriptorPublicKeyFromString(publicKey: publicKey);

  @override
  int get hashCode => ptr.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BdkDescriptorPublicKey &&
          runtimeType == other.runtimeType &&
          ptr == other.ptr;
}

class BdkDescriptorSecretKey {
  final DescriptorSecretKey ptr;

  const BdkDescriptorSecretKey({
    required this.ptr,
  });

  static Future<BdkDescriptorPublicKey> asPublic(
          {required BdkDescriptorSecretKey ptr}) =>
      core.instance.api.crateApiKeyBdkDescriptorSecretKeyAsPublic(ptr: ptr);

  String asString() =>
      core.instance.api.crateApiKeyBdkDescriptorSecretKeyAsString(
        that: this,
      );

  static Future<BdkDescriptorSecretKey> create(
          {required Network network,
          required BdkMnemonic mnemonic,
          String? password}) =>
      core.instance.api.crateApiKeyBdkDescriptorSecretKeyCreate(
          network: network, mnemonic: mnemonic, password: password);

  static Future<BdkDescriptorSecretKey> derive(
          {required BdkDescriptorSecretKey ptr,
          required BdkDerivationPath path}) =>
      core.instance.api
          .crateApiKeyBdkDescriptorSecretKeyDerive(ptr: ptr, path: path);

  static Future<BdkDescriptorSecretKey> extend(
          {required BdkDescriptorSecretKey ptr,
          required BdkDerivationPath path}) =>
      core.instance.api
          .crateApiKeyBdkDescriptorSecretKeyExtend(ptr: ptr, path: path);

  static Future<BdkDescriptorSecretKey> fromString(
          {required String secretKey}) =>
      core.instance.api
          .crateApiKeyBdkDescriptorSecretKeyFromString(secretKey: secretKey);

  /// Get the private key as bytes.
  Future<Uint8List> secretBytes() =>
      core.instance.api.crateApiKeyBdkDescriptorSecretKeySecretBytes(
        that: this,
      );

  @override
  int get hashCode => ptr.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BdkDescriptorSecretKey &&
          runtimeType == other.runtimeType &&
          ptr == other.ptr;
}

class BdkMnemonic {
  final Mnemonic ptr;

  const BdkMnemonic({
    required this.ptr,
  });

  String asString() => core.instance.api.crateApiKeyBdkMnemonicAsString(
        that: this,
      );

  /// Create a new Mnemonic in the specified language from the given entropy.
  /// Entropy must be a multiple of 32 bits (4 bytes) and 128-256 bits in length.
  static Future<BdkMnemonic> fromEntropy({required List<int> entropy}) =>
      core.instance.api.crateApiKeyBdkMnemonicFromEntropy(entropy: entropy);

  /// Parse a Mnemonic with given string
  static Future<BdkMnemonic> fromString({required String mnemonic}) =>
      core.instance.api.crateApiKeyBdkMnemonicFromString(mnemonic: mnemonic);

  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.
  /// Generates Mnemonic with a random entropy
  static Future<BdkMnemonic> newInstance({required WordCount wordCount}) =>
      core.instance.api.crateApiKeyBdkMnemonicNew(wordCount: wordCount);

  @override
  int get hashCode => ptr.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BdkMnemonic &&
          runtimeType == other.runtimeType &&
          ptr == other.ptr;
}
