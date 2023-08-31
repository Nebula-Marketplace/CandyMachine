use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;
use serde::Serialize;

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
            println!("Error opening file {}", filename);
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

    if  data.auth.address == "" || 
        data.auth.mnemonic == "" || 
        data.network.chain_id == "" || 
        data.network.grpc == "" || 
        data.network.rpc == "" {
        println!("Error: Config not finished in {}", filename);
        exit(1);
    }

    return data;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Network {
    pub rpc: String,
    pub grpc: String,
    pub chain_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionCreator {
    pub address: String,
    pub share: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub mnemonic: String,
    pub address: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContractInfo {
    pub admin: String,
    pub max_supply: u16,
    pub collection_name: String,
    pub collection_symbol: String,
    pub collection_description: String,
    pub collection_banner_uri: String,
    pub collection_pfp_uri: String,
    pub collection_ext_uri: String,
    pub royalty_bps: u8,
    pub creators: Vec<CollectionCreator>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub network: Network,
    pub contract: ContractInfo,
    pub auth: Auth
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstantiateResponse {
    pub code_id: u8,
    pub contract: String,
    pub result: String,
    pub collection: CollectionInfo
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MintResponse {

}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct CollectionInfo {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub max_supply: u128,
    pub minter: String
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