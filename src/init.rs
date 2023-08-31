use crate::types::{
    CollectionInfo,
    readConfig
};

use std::str::FromStr;

use cosmos_grpc_client::{
    GrpcClient, Wallet, BroadcastMode,
    cosmos_sdk_proto::cosmwasm::wasm::v1::MsgInstantiateContract,
    cosmrs::tx::MessageExt,
};
use serde_json::to_vec;

pub async fn instantiate_cw721(mut client: GrpcClient, collection: CollectionInfo, wallet: Wallet) {
    let data = readConfig();
    let request = MsgInstantiateContract {
            sender: &data.auth.address, 
            admin: &data.contract.admin,
            code_id: 49,
            label: "Init Nebula cw721".to_string(),
            msg: to_vec(&collection).expect("Serialization failed."),
            funds: vec![]
        }.to_any().unwrap();
    
    assert_eq!(&data.auth.address, wallet.account_address()); // confirm wallet was properly derived from mnemonic

    // let sim = wallet.simulate_tx(&mut client, vec![request]).await.unwrap();

    // println!("simulated transaction: \n {:?}", sim);

    let response = wallet.broadcast_tx(&mut client, vec![request], None, None, BroadcastMode::Sync).await.unwrap();

    println!("result: \n {:?}", response);

    // Instantiate response should contain codeid, contract address, tx hash, block height at confirmation
}