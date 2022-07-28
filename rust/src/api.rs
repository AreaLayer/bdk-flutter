use crate::ffi::{
    generate_extended_key, restore_extended_key, AddressIndex, ExtendedKeyInfo,
    PartiallySignedBitcoinTransaction, Transaction, TxBuilder, Wallet,
};
use anyhow::{anyhow, Result};
use bdk::bitcoin::consensus::deserialize;
use bdk::bitcoin::hashes::serde_macros::serde_details::SerdeHash;
use bdk::bitcoin::{base64, Network};
use bdk::blockchain::{Blockchain, ElectrumBlockchain, GetTx};
use bdk::electrum_client::Client;
use bdk::wallet::tx_builder;
use bdk::Error;
use flutter_rust_bridge::*;
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::sync::Arc;
use std::sync::RwLock;


fn config_network(network: String) -> Network {
    return match network.as_str() {
        "SIGNET" => Network::Signet,
        "TESTNET" => Network::Testnet,
        "REGTEST" => Network::Regtest,
        "BITCOIN" => Network::Bitcoin,
        _ => Network::Testnet,
    };
}
lazy_static! {
    static ref WALLET: RwLock<Wallet> = RwLock::new(Wallet::default());
    // static ref BLOCKCHAIN: RwLock<ElectrumBlockchain> = RwLock::new(blockchain_init());
}
fn blockchain_init() -> ElectrumBlockchain {
    let blockchain =
        ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002").unwrap());
    return blockchain;
}
pub fn wallet_init(descriptor:String,change_descriptor:String,network:String ) {
    let node_network = config_network(network.clone());
    let blockchain_obj = blockchain_init();
    let wallet = Wallet::new(
        descriptor.clone(),
        Some(change_descriptor.clone()),
        node_network,
    )
        .unwrap();
    wallet.sync(blockchain_obj);
    let mut new_wallet = WALLET.write().unwrap();
    *new_wallet = wallet
}
pub fn generate_key(node_network: String) -> ExtendedKeyInfo {
    let node_network = config_network(node_network);
    let response = generate_extended_key(node_network);
    return response;
}
pub fn restore_key(node_network: String, mnemonic: String) -> ExtendedKeyInfo {
    let node_network = config_network(node_network);
    let response = restore_extended_key(node_network, mnemonic);
    return response;
}

pub fn sync_wallet() {
    let res = WALLET.read().unwrap();
    let blockchain = blockchain_init();
    res.sync(blockchain);
}
pub fn get_balance() -> u64 {
    let res = WALLET.read().unwrap();
    let balance = res.get_balance().unwrap();
    balance
}
pub fn get_new_address() -> String {
    let res = WALLET.read().unwrap();
    let address = res.get_address(AddressIndex::New).unwrap().address;
    address
}
pub fn get_last_unused_address() -> String {
    let res = WALLET.read().unwrap();
    res.get_address(AddressIndex::New).unwrap().address
}
pub fn get_transactions() -> Vec<Transaction> {
    let res = WALLET.read().unwrap();
    let response: Vec<Transaction> = res.get_transactions().unwrap();
    return  response;
}
pub fn  create_transaction(recipient: String, amount: u64, fee_rate: f32) -> String {
    let res = WALLET.read().unwrap();
    let tx_builder = TxBuilder::new();
    let x =  tx_builder.
        add_recipient(recipient,amount).fee_rate(fee_rate).finish(&res);
   x.unwrap().serialize()
}
pub fn sign_and_broadcast( psbt_str:String) -> String {
    let wallet = WALLET.read().unwrap();
    let blockchain= blockchain_init();
    let  psbt = PartiallySignedBitcoinTransaction::new(psbt_str).unwrap();
    wallet.sign(&psbt);
    let tx = psbt.internal.lock().unwrap().clone().extract_tx();
    blockchain.broadcast(&tx).unwrap();
    tx.txid().to_string()
}



#[cfg(test)]
mod tests {
    use crate::api::{wallet_init, WalletObj, WALLET};
    #[test]
    fn init_wallet() {
        wallet_init(
             "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)".to_string(),
             "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)".to_string(),
            "TESTNET".to_string()
        );
        let res = WALLET.read().unwrap();
        let balance = res.get_balance().unwrap();
        assert_eq!(balance, 4);
    }
}

//
// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// pub struct BridgeResult {
//     result: String,
//     data: Vec<String>,
// }
//
// impl Default for BridgeResult {
//     fn default() -> BridgeResult {
//         BridgeResult {
//             result: "".to_string(),
//             data: vec!["".to_string()],
//         }
//     }
// }
//
// impl BridgeResult {
//     fn err<E: std::fmt::Debug>(desc: &'static str, err: E) -> BridgeResult {
//         //this should write a log of every error
//         let mut file = OpenOptions::new()
//             .write(true)
//             .append(true)
//             .create(true)
//             .open("log.txt")
//             .expect("failed to open log");
//         let local: DateTime<Local> = Local::now();
//         file.write(format!("{} ///{}: {:?}\n", local.date(), desc, err).as_bytes())
//             .expect("failed to write log");
//         BridgeResult {
//             result: "Err()".to_string(),
//             data: vec![format!("{}: {:?}", desc, err)],
//         }
//     }
//
//     fn ok<D: std::string::ToString>(data: D) -> BridgeResult {
//         BridgeResult {
//             result: "Ok()".to_string(),
//             data: vec![data.to_string()],
//         }
//     }
// }

// fn generate_mnemonic(data: &String, wallet_obj: Wallet) -> BridgeResult {
//     #[derive(Deserialize)]
//     struct Arguments {
//         network: String,
//     }
//     let arguments: Arguments = match serde_json::from_str(&data) {
//         Ok(data) => data,
//         Err(err) => return BridgeResult::err("failed to parse arguments\n, {}", err),
//     };
//     let node_network = self::config_network(arguments.network.to_string());
//     let response = generate_extended_key(node_network);
//     let response_str = serde_json::to_string(&response.mnemonic.clone().to_string()).unwrap();
//     return BridgeResult::ok(response_str);
// }
// fn config_blockchain(blockchain: &str, url: String, socks5: Option<String>) -> BlockchainConfig {
//     return match blockchain {
//         "ELECTRUM" => BlockchainConfig::Electrum {
//             config: ElectrumConfig {
//                 url,
//                 socks5,
//                 retry: 0,
//                 timeout: None,
//                 stop_gap: 0,
//             },
//         },
//         "ESPLORA" => BlockchainConfig::Esplora {
//             config: EsploraConfig {
//                 concurrency: None,
//                 stop_gap: 10,
//                 timeout: None,
//                 base_url: url,
//                 proxy: None,
//             },
//         },
//         _ => BlockchainConfig::Electrum {
//             config: ElectrumConfig {
//                 url,
//                 socks5,
//                 retry: 0,
//                 timeout: None,
//                 stop_gap: 0,
//             },
//         },
//     };
// }