use cosmos_grpc_client::Decimal;
use cosmos_grpc_client::GrpcClient;
use cosmos_grpc_client::Wallet;
use cosmos_grpc_client::CoinType;

use std::str::FromStr;
use serde_json::to_string;

mod init;
mod types;
mod mint;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() {
    let data = types::readConfig();
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

    let msg = init::instantiate_cw721(
        client,
        types::CollectionInfo {
            name: data.contract.collection_name.clone(), 
            description: data.contract.collection_description.clone(),
            symbol: data.contract.collection_symbol.clone(), 
            max_supply: data.contract.max_supply.clone(),
            minter: wallet.account_address()
        },
        wallet
    ).await;

    println!("msg: {:?}", msg);
}

// fn main() {
//     println!("{:?}", to_string(&mint::construct_mint_msg_self(
//         "https://github.com/Nebula-Marketplace".to_string(),
//         types::readConfig(),
//         "inj1a0cu20e2dupn8ja9xvuye8l2n6czxzp688zl30".to_string(), 
//     )).unwrap());
// }