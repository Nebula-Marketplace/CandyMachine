use serde_derive::Deserialize;
// use cosm_tome_wasm_deploy_fork::chain::response::ChainTxResponse;
// use std::fs;
// use std::process::exit;
// use toml;
use serde::Serialize;
use core::str::FromStr;
// use cosmrs::AccountId;
// use cosmrs::crypto::secp256k1;

// use cosmos_sdk_proto::cosmos::auth::v1beta1::{QueryAccountRequest, QueryAccountResponse};
// use cosmos_sdk_proto::cosmos::bank::v1beta1::QueryBalanceRequest;
// use cosmos_sdk_proto::cosmos::tx::v1beta1::QueryTxRequest;
// use cosmos_sdk_proto::cosmos::tx::v1beta1::QueryTxResponse;
// use cosmrs::http::HttpClient;
// use prost::Message;

extern crate serde;
extern crate serde_json;

#[allow(non_snake_case)]
pub fn readConfig() -> Data {
    let filename = "Nebula.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            println!("Error reading file {}", filename);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            println!("Error parsing file {}", filename);
            exit(1);
        }
    };

    if  data.network.address == "" || 
        data.network.private_key == "" || 
        data.network.chain_id == "" || 
        data.network.grpc == "" || 
        data.network.rpc == "" {
        println!("Error: Config not finished in {}", filename);
        exit(1);
    }

    return data;
}

#[derive(Deserialize, Clone)]
pub struct Network {
    pub rpc: String,
    pub grpc: String,
    pub chain_id: String,
}

#[derive(Deserialize, Clone)]
pub struct CollectionCreator {
    pub address: String,
    pub share: u8,
}

#[derive(Deserialize, Clone)]
pub struct Auth {
    mnemonic: String,
    address: String
}

#[derive(Deserialize, Clone)]
pub struct ContractInfo {
    admin: String,
    max_supply: u16,
    collection_name: String,
    collection_symbol: String,
    collection_description: String,
    collection_banner_uri: String,
    collection_pfp_uri: String,
    collection_ext_uri: String,
    royalty_bps: u8,
    creators: Vec<CollectionCreator>,
}

#[derive(Deserialize, Clone)]
pub struct Data {
    pub network: Network,
    pub contract: ContractInfo,
    pub auth: Auth
}

#[derive(Debug, Clone)]
pub struct InstantiateResponse {
    pub code_id: u8,
    pub contract: String,
    pub result: String,
    pub collection: CollectionInfo
}

#[derive(Debug, Clone)]
pub struct MintResponse {

}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct CollectionInfo {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub max_supply: u16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MsgMint {
    pub sender: String,
    pub contract: String,
    pub funds: u8,
    pub msg: Mint
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mint {
    pub owner: String,
    pub metadata_uri: String,
    pub royalty: Royalties
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Royalties {
    pub seller_fee_basis_points: u16,
    pub creators: Vec<CollectionCreator>,
    pub primary_sell_happened: bool
}