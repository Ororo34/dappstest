use sui_sdk::SuiClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    let sui = SuiClientBuilder::default()
        .build("http://127.0.0.1:9000") // provide the Sui network URL
        .await?;
    println!("Sui local network version: {:?}", sui.api_version());

    // local Sui network, same result as above except using the dedicated function
    let sui_local = SuiClientBuilder::default().build_localnet().await?;
    println!("Sui local network version: {:?}", sui_local.api_version());

    // Sui devnet running at `https://fullnode.devnet.io:443`
    let sui_devnet = SuiClientBuilder::default().build_devnet().await?;
    println!("Sui devnet version: {:?}", sui_devnet.api_version());

    // Sui testnet running at `https://testnet.devnet.io:443`
    let sui_testnet = SuiClientBuilder::default().build_testnet().await?;
    println!("Sui testnet version: {:?}", sui_testnet.api_version());
    Ok(())

}
