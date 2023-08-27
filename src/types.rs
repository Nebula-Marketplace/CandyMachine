use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Network {
    pub rpc: String,
    pub grpc: String,
    pub chain_id: String,
    pub address: String,
    pub mnemonic: String
}

#[derive(Deserialize)]
pub struct Data {
    pub network: Network,
}

#[allow(non_snake_case)]
pub struct CollectionInfo {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub max_supply: u16,
}