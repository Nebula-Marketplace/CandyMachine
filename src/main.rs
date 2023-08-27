

mod init;
mod types;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    let i = init::instantiate(
        types::CollectionInfo {
            name: "Nebula".to_string(),
            symbol: "NBLA".to_string(),
            description: "Nebula is a collection of 10,000 unique NFTs living on the Injective Chain.".to_string(),
            max_supply: 10000
        }
    ).await;
    println!("{}", i);
}