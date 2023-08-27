use cosm_tome_wasm_deploy_fork::clients::{
    client::CosmTome,
    cosmos_grpc::CosmosgRPC
};
use cosm_tome_wasm_deploy_fork::chain::{
    coin::{
        Coin,
        Denom
    },
    fee::{
        Fee,
        Gas
    },
    request::TxOptions
};
use cosm_tome_wasm_deploy_fork::signing_key::{
    key::Key::Mnemonic,
    key::SigningKey,
};
use cosm_tome_wasm_deploy_fork::modules::{
    auth::model::Address,
    cosmwasm::model::InstantiateRequest
};
use cosm_tome_wasm_deploy_fork::config::cfg::ChainConfig;

use std::fs;
use std::process::exit;
use toml;
use serde::Serialize;
use core::str::FromStr;

extern crate serde;
extern crate serde_json;

use crate::types::{CollectionInfo, Data};

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

    if data.network.address == "" || data.network.mnemonic == "" || data.network.chain_id == "" || data.network.grpc == "" || data.network.rpc == "" {
        println!("Error: Config not finished in {}", filename);
        exit(1);
    }

    return data;
}

#[derive(Serialize)]
struct Msg {
    admin: String,
    sender: String,
    code_id: u8,
    label: String,
    name: String,
    symbol: String,
    description: String,
    minter: String,
    max_supply: u16,
    fundslist: Vec<Coin>
}

pub async fn instantiate(
    #[allow(non_snake_case)]
    collectionInfo: CollectionInfo
) -> String {
    let config = readConfig();
    let m = Msg {
        admin: config.network.address.clone(),
        sender: config.network.address.clone(),
        code_id: 49,
        label: "Init Nebula Cw721".to_string(),
        name: collectionInfo.name,
        symbol: collectionInfo.symbol,
        description: collectionInfo.description,
        minter: config.network.address.clone(),
        max_supply: collectionInfo.max_supply,
        fundslist: Vec::new()
    };
    let grpc = config.network.grpc;
    let client = CosmTome::new(ChainConfig {
        denom: "inj".to_string(),
        prefix: "inj".to_string(),
        chain_id: config.network.chain_id,
        derivation_path: "m/44'/60'/0'/0/0".to_string(),
        rpc_endpoint: Some(config.network.rpc),
        grpc_endpoint: Some(grpc.clone()),
        gas_prices: 0.025,
        gas_adjustment: 1.3
    }, CosmosgRPC::new(grpc.clone()));
    let init = client.wasm_instantiate(
        InstantiateRequest {
            code_id: 49,
            msg: serde_json::to_string(&m).unwrap(),
            funds: Vec::new(),
            label: m.label,
            admin: Some(Address::from_str(&config.network.address.clone()).unwrap()),
        },
        &SigningKey {
            name: config.network.address.clone(),
            key: Mnemonic(config.network.mnemonic.clone()),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
        },
        &TxOptions {
            fee: Some(Fee {
                amount: vec![Coin {
                    denom: Denom::from_str("inj").unwrap(),
                    amount: 500000000
                }],
                gas_limit: Gas::default(),
                granter: Some(Address::from_str(&config.network.address.clone()).unwrap()),
                payer: Some(Address::from_str(&config.network.address.clone()).unwrap()),
            }),
            memo: "Init Nebula Cw721".to_string(),
            timeout_height: Some(9999)
        }
    ).await;
    format!("{:?}", init)
}