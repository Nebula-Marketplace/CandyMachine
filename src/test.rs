use crate::types::{
    CollectionInfo,
    readConfig,
    InstantiateResponse,
    MsgMint,
    M,
    Mint,
    Royalties,
    CollectionCreator
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
    let mut client = GrpcClient::new("http://injective-grpc.polkachu.com:14390").await.unwrap();
    let wallet = Wallet::new(
        &mut client,
        "arrange cactus jewel fuel fantasy vote picture kitchen stand talk jelly foot".to_string(),
        "inj",
        CoinType::Injective,
        0,
        Decimal::from_str("1000000000").unwrap(), // Gas_price
        Decimal::from_str("1.5").unwrap(), // Gas_adjustment
        "inj",
    ).await.unwrap();

    assert_eq!("inj1s3hfffwfvehcmwxdltcmt5a4fntj4ytaqstxnr", &wallet.account_address()); // confirm wallet was properly derived from mnemonic

    let collection = CollectionInfo {
        name: "Nebula".to_string(), 
        description: "This is a test collection".to_string(),
        symbol: "TEST".to_string(), 
        max_supply: 1,
        minter: wallet.account_address()
    };
    simulate_cw721(client, collection, wallet, "inj1s3hfffwfvehcmwxdltcmt5a4fntj4ytaqstxnr".to_string()).await;
}

#[tokio::test]
async fn test_mint() {
    let address = "inj1xcy30kk2v5hyhk06wx6v4cn392amxwzx3smer8"; // Test contract address
    let mut client = GrpcClient::new("http://injective-grpc.polkachu.com:14390").await.unwrap();
    let wallet = Wallet::new(
        &mut client,
        "arrange cactus jewel fuel fantasy vote picture kitchen stand talk jelly foot".to_string(),
        "inj",
        CoinType::Injective,
        0,
        Decimal::from_str("1000000000").unwrap(), // Gas_price
        Decimal::from_str("1.5").unwrap(), // Gas_adjustment
        "inj",
    ).await.unwrap();

    assert_eq!("inj1s3hfffwfvehcmwxdltcmt5a4fntj4ytaqstxnr", &wallet.account_address()); // confirm wallet was properly derived from mnemonic

    let _ = simulate_mint(
        client,
        MsgMint {
            sender: "inj1s3hfffwfvehcmwxdltcmt5a4fntj4ytaqstxnr".to_string(),
            contract: address.to_string(),
            funds: 0,
            msg: M {
                    mint: Mint {
                    owner: "inj1s3hfffwfvehcmwxdltcmt5a4fntj4ytaqstxnr".to_string(),
                    metadata_uri: "https://github.com/Nebula-Marketplace/CandyMachine".to_string(),
                    royalty: Royalties {
                        seller_fee_basis_points: 0,
                        creators: vec![CollectionCreator {
                            address: "inj1s3hfffwfvehcmwxdltcmt5a4fntj4ytaqstxnr".to_string(),
                            share: 100
                        }],
                        primary_sell_happened: true
                    }
                }
            }
        },
        wallet
    ).await.unwrap();
}