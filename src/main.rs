use iota_client::{Client};

#[tokio::main]
async fn main() {
    let balance = Balance{ };
    balance.balance().await;
}
struct Balance {}
impl Balance {
    async fn balance(&self)
    {
        let iota = Client::builder()
            .with_node("https://chrysalis-nodes.iota.org")
            .unwrap()
            .with_node_sync_disabled()
            .finish()
            .await
            .unwrap();
        let address = "iota1qrepert94y80wyx6j07n2zgldfy2509hpugkx3apvdga0uux780xz3xtcyf";
        let response = iota.get_address().balance(&address).await.unwrap();
        let balance: u64 = response.balance;
        #[derive(Debug)]
        struct Information {
            balance: u64,
            address: String
        }
        let balance_address = Information { balance, address: response.address };
        println!("{:?}", balance_address)
    }
}