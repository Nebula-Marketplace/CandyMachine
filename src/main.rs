use cosmos_grpc_client::Decimal;
use cosmos_grpc_client::GrpcClient;
use cosmos_grpc_client::Wallet;
use cosmos_grpc_client::CoinType;
use serde::Deserialize;

#[allow(unused_imports)]
use std::str::FromStr;
#[allow(unused_imports)]
use serde_json::to_string;

use std::thread;
use std::io::stdin;
use std::time::Duration;

#[allow(unused_imports)]
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::copy;
use std::io::stdout;

extern crate serde;
extern crate serde_json;

mod init;
mod types;
mod mint;

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize)]
struct TokenInfo {
    owner: String,
    uri: String
}

#[tokio::main]
async fn main() {
    let data = types::readConfig();
    let static_addr: &'static str = Box::leak(data.network.grpc.clone().into_boxed_str());
    let mut client = GrpcClient::new(static_addr).await.unwrap();
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
        &mut client,
        types::CollectionInfo {
            name: data.contract.collection_name.clone(), 
            description: data.contract.collection_description.clone(),
            symbol: data.contract.collection_symbol.clone(), 
            max_supply: data.contract.max_supply.clone(),
            minter: wallet.account_address()
        },
        &wallet
    ).await;

    let hash = msg
                .unwrap() // unwrap from Result
                .result // get result object
                .unwrap() // unwrap from Option
                .txhash; // get txhash as string

    println!("waiting for transaction to be confirmed...");
    let delay = Duration::from_secs(5);
    thread::sleep(delay);
    println!("Tx Hash: {}", hash);
    thread::sleep(delay);
    println!("Please enter the contract address: ");
    let mut contract_address = String::new();
    stdin().read_line(&mut contract_address).expect("Failed");
    
    // read json file
    let mut file = File::open("metadata.json").unwrap();
    let mut stdout = stdout();
    let s = &copy(&mut file, &mut stdout).unwrap().to_string();
    let json: Vec<TokenInfo> = serde_json::from_str(s).expect("JSON was not correctly formatted");

    println!("{:?}", json);

    for token in json {
        let owner = token.owner;
        let uri = token.uri;

        #[allow(unused_must_use)]
        let _ = mint::mint(
            &mut client,
            mint::construct_mint_msg_ext(
                owner,
                uri,
                &data.clone(),
                contract_address.clone()
            ),
            &wallet
        ).await;
    }

    println!("All done!")
}