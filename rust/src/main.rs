mod application;
mod domain;
mod infrastructure;

use crate::infrastructure::exchange::bybit::api::BybitMarket;
use crate::infrastructure::exchange::bybit::api::models::Category;
use crate::infrastructure::exchange::bybit::client::BybitClient;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let client = BybitClient::new("", "");
    let order_book = client
        .get_orderbook(Category::Spot, "BTCUSDT", "25")
        .await
        .unwrap();
    println!("{:?}", order_book);
}
