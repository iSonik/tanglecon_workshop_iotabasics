// Goal: Create wallet from predefined mnemonic
// run application via: cargo run

use iota_wallet::{account_manager::AccountManager, client::ClientOptionsBuilder, signing::SignerType};
use iota_client::crypto::signatures::ed25519::SecretKey;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> iota_wallet::Result<()> {

// Select the path where you want to create your Stronghold
let storage_folder: PathBuf = "./my-db".into();

// Example Node API for the IOTA Testnet: let node_url: String = "https://api.lb-0.h.chrysalis-devnet.iota.cafe".to_string();
let node_url: String = "https://api.lb-0.h.chrysalis-devnet.iota.cafe".to_string();
let password: String = "password".to_string();

let manager = AccountManager::builder()
    .with_storage(&storage_folder, None)?
    .finish()
    .await?;

// Set the password for your Stronghold file
manager.set_stronghold_password(&password).await?;

let mnemonic = "park remain person kitchen mule spell knee armed position rail grid ankle".to_string();

// Stores your mnemonic in Stronghold file
manager.store_mnemonic(SignerType::Stronghold, Some(mnemonic)).await?;

let client_options = ClientOptionsBuilder::new()
    .with_node(&node_url)?
    .build()?;

let account = manager
    .create_account(client_options)?
    .signer_type(SignerType::Stronghold)
    .initialise()
    .await?;

let address = account.generate_address().await?;
println!("Address: {}", address.address().to_bech32());

Ok(())
}
