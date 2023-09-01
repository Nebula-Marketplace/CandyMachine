use crate::types::{
    CollectionInfo,
    readConfig,
    InstantiateResponse
};

use std::str::FromStr;

use cosmos_grpc_client::{
    GrpcClient, Wallet, BroadcastMode,
    cosmos_sdk_proto::cosmwasm::wasm::v1::MsgInstantiateContract,
    cosmrs::tx::MessageExt,
    Decimal, CoinType, 
};
use serde_json::to_vec;

use crate::mint::{
    construct_mint_msg_ext,
    construct_mint_msg_self,
    mint,
    simulate_mint
};
use crate::init::{
    instantiate_cw721,
    simulate_cw721
};

#[tokio::test]
async fn test_init() {
    let data = readConfig();
    let mut client = GrpcClient::new("http://injective-grpc.polkachu.com:14390").await.unwrap();
    let wallet = Wallet::new(
        &mut client,
        &data.auth.mnemonic,
        "inj",
        CoinType::Injective,
        0,
        Decimal::from_str("1000000000").unwrap(), // Gas_price
        Decimal::from_str("1.5").unwrap(), // Gas_adjustment
        "inj",
    ).await.unwrap();

    assert_eq!(&data.auth.address, &wallet.account_address()); // confirm wallet was properly derived from mnemonic

    let collection = CollectionInfo {
        name: "Nebula".to_string(), 
        description: "This is a test collection".to_string(),
        symbol: "TEST".to_string(), 
        max_supply: 1,
        minter: wallet.account_address()
    };
    simulate_cw721(client, collection, wallet).await;
}

#[tokio::test]
async fn test_mint() {
    let address = "inj1xcy30kk2v5hyhk06wx6v4cn392amxwzx3smer8"; // Test contract address
    let data = readConfig();
    let mut client = GrpcClient::new("http://injective-grpc.polkachu.com:14390").await.unwrap();
    let wallet = Wallet::new(
        &mut client,
        &data.auth.mnemonic,
        "inj",
        CoinType::Injective,
        0,
        Decimal::from_str("1000000000").unwrap(), // Gas_price
        Decimal::from_str("1.5").unwrap(), // Gas_adjustment
        "inj",
    ).await.unwrap();
    let _ = simulate_mint(
        client,
        construct_mint_msg_self(
            "ipfs://QmNtkD8y4i9xDQtzNFVsiu1kcnoYbMVshwEJLtZNMwNcxa/1807.json".to_string(),
            data.clone(),
            address.to_string(), 
        ),
        wallet
    ).await.unwrap();
}