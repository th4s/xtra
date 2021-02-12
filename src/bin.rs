use ethane::rpc::eth_block_number;
use ethane::{Connector, Credentials};
use libblockbender::working;

fn main() {
    dotenv::from_filename("./secret.env");
    let address = std::env::var("ETH1").unwrap();
    let secret = Some(Credentials::Basic(std::env::var("SECRET").unwrap()));
    let mut connector = Connector::websocket(&address, secret).unwrap();
    println!(
        "{} Current block number is {}.",
        working(),
        connector.call(eth_block_number()).unwrap()
    )
}
