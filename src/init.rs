use crate::types::{
    CollectionInfo,
    readConfig,
    InstantiateResponse
};

#[allow(unused_imports)]
use std::str::FromStr;

use cosmos_grpc_client::{
    GrpcClient, Wallet, BroadcastMode,
    cosmos_sdk_proto::cosmwasm::wasm::v1::MsgInstantiateContract,
    cosmrs::tx::MessageExt,
};
use serde_json::to_vec;

pub async fn instantiate_cw721(client: &mut GrpcClient, collection: CollectionInfo, wallet: &Wallet) -> Result<InstantiateResponse, Box<dyn std::error::Error>> {
    let data = readConfig();
    assert_eq!(&data.auth.address, &wallet.account_address()); // confirm wallet was properly derived from mnemonic

    let request = MsgInstantiateContract {
            sender: data.auth.address, 
            admin: data.contract.admin,
            code_id: 49,
            label: "Init Nebula cw721".to_string(),
            msg: to_vec(&collection).expect("Serialization failed."),
            funds: vec![]
        }.to_any().unwrap();

    // let sim = wallet.simulate_tx(&mut client, vec![request]).await.unwrap();

    // println!("simulated transaction: \n {:?}", sim);

    let response = wallet.broadcast_tx(client, vec![request], None, None, BroadcastMode::Sync).await.unwrap();

    Ok(InstantiateResponse {
        code_id: 49,
        result: response.tx_response,
        collection: collection
    })
    // Instantiate response should contain codeid, contract address, tx hash, block height at confirmation
}

#[allow(dead_code)]
pub async fn simulate_cw721(client: &mut GrpcClient, collection: CollectionInfo, wallet: Wallet, admin: String)  {
    let request = MsgInstantiateContract {
            sender: wallet.account_address(), 
            admin: admin,
            code_id: 49,
            label: "Init Nebula cw721".to_string(),
            msg: to_vec(&collection).expect("Serialization failed."),
            funds: vec![]
        }.to_any().unwrap();

    wallet.simulate_tx(client, vec![request]).await.unwrap();
}