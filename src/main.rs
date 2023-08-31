use cosmos_grpc_client::Decimal;
use cosmos_grpc_client::GrpcClient;
use cosmos_grpc_client::Wallet;
use cosmos_grpc_client::CoinType;

use std::str::FromStr;

mod init;
mod types;

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

    init::instantiate_cw721(
        client,
        types::CollectionInfo {
            name: &data.contract.name, 
            description: &data.contract.description,, 
            symbol: &data.contract.symbol, 
            max_supply: &data.contract.max_supply,
            minter: wallet.account_address()
        },
        wallet
    ).await
}