use crate::infrastructure::exchange::bybit::api::models::{Category, OrderbookData};
use async_trait::async_trait;

pub mod market;
pub mod models;

#[async_trait]
pub trait BybitMarket {
    async fn get_orderbook(
        &self,
        category: Category,
        symbol: &str,
        limit: &str,
    ) -> anyhow::Result<OrderbookData>;
}
