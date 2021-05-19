use iota_client::Client;

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("https://api.lb-0.testnet.chrysalis2.com")
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Nodeinfo: {:?}", info);
}