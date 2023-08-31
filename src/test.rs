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

    let request = MsgInstantiateContract {
            sender: data.auth.address, 
            admin: data.contract.admin,
            code_id: 49,
            label: "Init Nebula cw721".to_string(),
            msg: to_vec(&collection).expect("Serialization failed."),
            funds: vec![]
        }.to_any().unwrap();
    

    let sim = wallet.simulate_tx(&mut client, vec![request]).await.unwrap();
}