// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.25.

// Section: imports

use super::*;
use crate::api::types::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::for_generated::wasm_bindgen;
use flutter_rust_bridge::for_generated::wasm_bindgen::prelude::*;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_web!();

// Section: dart2rust

impl CstDecode<String> for String {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        self
    }
}
impl CstDecode<crate::api::types::AddressBase>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::AddressBase {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::types::AddressBase(self_.get(0).cst_decode())
    }
}
impl CstDecode<crate::util::error::BdkError>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::util::error::BdkError {
        let self_ = self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Array>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::util::error::BdkError::HexError(self_.get(1).cst_decode()),
            1 => crate::util::error::BdkError::ConsensusError(self_.get(1).cst_decode()),
            2 => crate::util::error::BdkError::AddressError(self_.get(1).cst_decode()),
            _ => unreachable!(),
        }
    }
}
impl CstDecode<crate::util::error::BitcoinAddressError>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::util::error::BitcoinAddressError {
        let self_ = self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Array>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::util::error::BitcoinAddressError::Base58(self_.get(1).cst_decode()),
            1 => crate::util::error::BitcoinAddressError::Bech32(self_.get(1).cst_decode()),
            2 => crate::util::error::BitcoinAddressError::EmptyBech32Payload,
            3 => crate::util::error::BitcoinAddressError::InvalidBech32Variant {
                expected: self_.get(1).cst_decode(),
                found: self_.get(2).cst_decode(),
            },
            4 => crate::util::error::BitcoinAddressError::InvalidWitnessVersion(
                self_.get(1).cst_decode(),
            ),
            5 => crate::util::error::BitcoinAddressError::UnparsableWitnessVersion(
                self_.get(1).cst_decode(),
            ),
            6 => crate::util::error::BitcoinAddressError::MalformedWitnessVersion,
            7 => crate::util::error::BitcoinAddressError::InvalidWitnessProgramLength(
                self_.get(1).cst_decode(),
            ),
            8 => crate::util::error::BitcoinAddressError::InvalidSegwitV0ProgramLength(
                self_.get(1).cst_decode(),
            ),
            9 => crate::util::error::BitcoinAddressError::UncompressedPubkey,
            10 => crate::util::error::BitcoinAddressError::ExcessiveScriptSize,
            11 => crate::util::error::BitcoinAddressError::UnrecognizedScript,
            12 => crate::util::error::BitcoinAddressError::UnknownAddressType(
                self_.get(1).cst_decode(),
            ),
            13 => crate::util::error::BitcoinAddressError::NetworkValidation {
                network_required: self_.get(1).cst_decode(),
                network_found: self_.get(2).cst_decode(),
                address: self_.get(3).cst_decode(),
            },
            _ => unreachable!(),
        }
    }
}
impl CstDecode<crate::util::error::BitcoinConsensusError>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::util::error::BitcoinConsensusError {
        let self_ = self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Array>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::util::error::BitcoinConsensusError::Io(self_.get(1).cst_decode()),
            1 => crate::util::error::BitcoinConsensusError::OversizedVectorAllocation {
                requested: self_.get(1).cst_decode(),
                max: self_.get(2).cst_decode(),
            },
            2 => crate::util::error::BitcoinConsensusError::InvalidChecksum {
                expected: self_.get(1).cst_decode(),
                actual: self_.get(2).cst_decode(),
            },
            3 => crate::util::error::BitcoinConsensusError::NonMinimalVarInt,
            4 => crate::util::error::BitcoinConsensusError::ParseFailed(self_.get(1).cst_decode()),
            5 => crate::util::error::BitcoinConsensusError::UnsupportedSegwitFlag(
                self_.get(1).cst_decode(),
            ),
            _ => unreachable!(),
        }
    }
}
impl CstDecode<crate::util::error::BitcoinHexError>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::util::error::BitcoinHexError {
        let self_ = self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Array>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::util::error::BitcoinHexError::InvalidChar(self_.get(1).cst_decode()),
            1 => crate::util::error::BitcoinHexError::OddLengthString(self_.get(1).cst_decode()),
            2 => crate::util::error::BitcoinHexError::InvalidLength(
                self_.get(1).cst_decode(),
                self_.get(2).cst_decode(),
            ),
            _ => unreachable!(),
        }
    }
}
impl CstDecode<Vec<Vec<u8>>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<Vec<u8>> {
        self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap()
            .iter()
            .map(CstDecode::cst_decode)
            .collect()
    }
}
impl CstDecode<Vec<u8>> for Box<[u8]> {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl CstDecode<Vec<crate::api::types::TxIn>>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<crate::api::types::TxIn> {
        self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap()
            .iter()
            .map(CstDecode::cst_decode)
            .collect()
    }
}
impl CstDecode<Vec<crate::api::types::TxOut>>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<crate::api::types::TxOut> {
        self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap()
            .iter()
            .map(CstDecode::cst_decode)
            .collect()
    }
}
impl CstDecode<crate::api::types::OutPoint>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::OutPoint {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::types::OutPoint {
            txid: self_.get(0).cst_decode(),
            vout: self_.get(1).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::Payload>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::Payload {
        let self_ = self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Array>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::types::Payload::PubkeyHash {
                pubkey_hash: self_.get(1).cst_decode(),
            },
            1 => crate::api::types::Payload::ScriptHash {
                script_hash: self_.get(1).cst_decode(),
            },
            2 => crate::api::types::Payload::WitnessProgram {
                version: self_.get(1).cst_decode(),
                program: self_.get(2).cst_decode(),
            },
            _ => unreachable!(),
        }
    }
}
impl CstDecode<crate::api::types::ScriptBufBase>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::ScriptBufBase {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::types::ScriptBufBase {
            bytes: self_.get(0).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::TransactionBase>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::TransactionBase {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::types::TransactionBase {
            inner: self_.get(0).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::TxIn>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::TxIn {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::types::TxIn {
            previous_output: self_.get(0).cst_decode(),
            script_sig: self_.get(1).cst_decode(),
            sequence: self_.get(2).cst_decode(),
            witness: self_.get(3).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::TxOut>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::TxOut {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::types::TxOut {
            value: self_.get(0).cst_decode(),
            script_pubkey: self_.get(1).cst_decode(),
        }
    }
}
impl CstDecode<[u8; 4]> for Box<[u8]> {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> [u8; 4] {
        let vec: Vec<u8> = self.cst_decode();
        flutter_rust_bridge::for_generated::from_vec_to_array(vec)
    }
}
impl CstDecode<bdk::bitcoin::Address>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> bdk::bitcoin::Address {
        CstDecode::<
            RustOpaqueNom<
                flutter_rust_bridge::for_generated::rust_async::RwLock<bdk::bitcoin::Address>,
            >,
        >::cst_decode(self)
        .rust_auto_opaque_decode_owned()
    }
}
impl
    CstDecode<
        RustOpaqueNom<
            flutter_rust_bridge::for_generated::rust_async::RwLock<bdk::bitcoin::Address>,
        >,
    > for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(
        self,
    ) -> RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<bdk::bitcoin::Address>>
    {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }
        unsafe { decode_rust_opaque_nom((self.as_f64().unwrap() as usize) as _) }
    }
}
impl CstDecode<String> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl CstDecode<bool> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> bool {
        self.is_truthy()
    }
}
impl CstDecode<i32> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<Vec<u8>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Uint8Array>()
            .to_vec()
            .into()
    }
}
impl CstDecode<crate::api::types::Network>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::Network {
        (self.unchecked_into_f64() as i32).cst_decode()
    }
}
impl CstDecode<u32> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<u64> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u64 {
        ::std::convert::TryInto::try_into(
            self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::BigInt>()
                .unwrap(),
        )
        .unwrap()
    }
}
impl CstDecode<u8> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<[u8; 4]> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> [u8; 4] {
        let vec: Vec<u8> = self.cst_decode();
        flutter_rust_bridge::for_generated::from_vec_to_array(vec)
    }
}
impl CstDecode<usize> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> usize {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<crate::api::types::Variant>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::Variant {
        (self.unchecked_into_f64() as i32).cst_decode()
    }
}
impl CstDecode<crate::api::types::WitnessVersion>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::WitnessVersion {
        (self.unchecked_into_f64() as i32).cst_decode()
    }
}

#[wasm_bindgen]
pub fn wire_AddressBase_as_string(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_AddressBase_as_string_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_AddressBase_from_script(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    script: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    network: i32,
) {
    wire_AddressBase_from_script_impl(port_, script, network)
}

#[wasm_bindgen]
pub fn wire_AddressBase_from_string(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    address: String,
    network: i32,
) {
    wire_AddressBase_from_string_impl(port_, address, network)
}

#[wasm_bindgen]
pub fn wire_AddressBase_is_valid_for_network(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    network: i32,
) {
    wire_AddressBase_is_valid_for_network_impl(port_, that, network)
}

#[wasm_bindgen]
pub fn wire_AddressBase_network(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_AddressBase_network_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_AddressBase_payload(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_AddressBase_payload_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_AddressBase_script_pubkey(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_AddressBase_script_pubkey_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_AddressBase_to_qr_uri(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_AddressBase_to_qr_uri_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_ScriptBufBase_empty(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_ScriptBufBase_empty_impl(port_)
}

#[wasm_bindgen]
pub fn wire_ScriptBufBase_from_hex(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    s: String,
) {
    wire_ScriptBufBase_from_hex_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_ScriptBufBase_with_capacity(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    capacity: usize,
) {
    wire_ScriptBufBase_with_capacity_impl(port_, capacity)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_input(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_input_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_is_coin_base(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_is_coin_base_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_is_explicitly_rbf(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_is_explicitly_rbf_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_is_lock_time_enabled(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_is_lock_time_enabled_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_lock_time(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_lock_time_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_new(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    transaction_bytes: Box<[u8]>,
) {
    wire_TransactionBase_new_impl(port_, transaction_bytes)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_output(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_output_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_serialize(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_serialize_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_size(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_size_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_txid(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_txid_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_version(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_version_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_vsize(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_vsize_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_TransactionBase_weight(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_TransactionBase_weight_impl(port_, that)
}

#[wasm_bindgen]
pub fn rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockbdkbitcoinAddress(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<bdk :: bitcoin :: Address>>::increment_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockbdkbitcoinAddress(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<bdk :: bitcoin :: Address>>::decrement_strong_count(ptr as _);
    }
}
