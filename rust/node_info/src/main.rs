// Goal: Recieve node infos from testnet via api call and then send part of that node info as a message to the Tangle 
// run application via: cargo run

use iota_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {

// Example Node API for the IOTA Testnet: let node_url: String = "https://api.lb-0.h.chrysalis-devnet.iota.cafe".to_string();
let node_url: String = "https://api.lb-0.h.chrysalis-devnet.iota.cafe".to_string();

    // Initialise the IOTA client
    let iota = Client::builder()
    .with_node(&node_url)?
    .finish()
    .await?;

    // Get node info
    let info = iota.get_info().await.unwrap();
    println!("{:?}", info);


    // Get messages_per_second from node info
    let messages_per_second = info.nodeinfo.messages_per_second.to_string();
    
    let index: &str = "Node Status";
    let mut content: String = "Messages per second: ".to_string();
    content.push_str(&messages_per_second);

    // Send payload with index and data to the tangle
        let message = iota
        .message()
        .with_index(index)
        .with_data(content.as_bytes().to_vec())
        .finish()
        .await?;

    println!(
        "Message sent https://explorer.iota.org/devnet/message/{}\n",
        message.id().0
    );

Ok(())
}
