use crate::types::{
    MsgMint,
    Mint, 
    Royalties, 
    MintResponse,
    Data,
    M,
    readConfig
};

#[allow(unused_imports)]
use std::str::FromStr;

use cosmos_grpc_client::{
    GrpcClient, Wallet, BroadcastMode,
    cosmos_sdk_proto::cosmwasm::wasm::v1::MsgExecuteContract,
    cosmrs::tx::MessageExt,
};
use serde_json::to_vec;

pub fn construct_mint_msg_self(
        url: String, 
        d: Data,
        contract: String
    ) -> MsgMint {
    /*
    Mints an nft to self 
    */
    return MsgMint {
        sender: d.clone().auth.address,
        contract: contract,
        funds: 0,
        msg: M {
                mint: Mint {
                owner: d.clone().auth.address,
                metadata_uri: url,
                royalty: Royalties {
                    seller_fee_basis_points: d.clone().contract.royalty_bps,
                    creators: d.clone().contract.creators,
                    primary_sell_happened: true
                }
            }
        }
    }
}

pub fn construct_mint_msg_ext(
    owner: String,
    url: String,
    d: Data,
    contract: String
) -> MsgMint {
    return MsgMint {
        sender: owner.clone(),
        contract: contract,
        funds: 0,
        msg: M {
                mint: Mint {
                owner: owner.clone(),
                metadata_uri: url,
                royalty: Royalties {
                    seller_fee_basis_points: d.clone().contract.royalty_bps,
                    creators: d.clone().contract.creators,
                    primary_sell_happened: true
                }
            }
        }
    }
}

pub async fn mint(client: &mut GrpcClient, message: MsgMint, wallet: &Wallet) -> Result<MintResponse, Box<dyn std::error::Error>> {
    let data = readConfig();
    assert_eq!(&data.auth.address, &wallet.account_address()); // confirm wallet was properly derived from mnemonic

    let request = MsgExecuteContract {
            sender: data.auth.address, 
            contract: message.contract.clone(),
            msg: to_vec(&message.msg).expect("Serialization failed."),
            funds: vec![]
        }.to_any().unwrap();

    let _response = wallet.broadcast_tx(client, vec![request], None, None, BroadcastMode::Async).await.unwrap();

    Ok(MintResponse {})
    // Instantiate response should contain codeid, contract address, tx hash, block height at confirmation
}

#[allow(dead_code)]
pub async fn simulate_mint(client: &mut GrpcClient, message: MsgMint, wallet: Wallet) -> Result<MintResponse, Box<dyn std::error::Error>> {
    let request = MsgExecuteContract {
            sender: wallet.account_address(), 
            contract: message.contract.clone(),
            msg: to_vec(&message.msg).expect("Serialization failed."),
            funds: vec![]
        }.to_any().unwrap();

    let _sim = wallet.simulate_tx(client, vec![request]).await.unwrap();

    Ok(MintResponse {})
}