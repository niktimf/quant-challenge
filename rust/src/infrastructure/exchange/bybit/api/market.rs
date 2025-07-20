use crate::infrastructure::exchange::bybit::api::BybitMarket;
use crate::infrastructure::exchange::bybit::api::models::{BybitResponse, Category, OrderbookData};
use crate::infrastructure::exchange::bybit::client::BybitClient;
use crate::infrastructure::exchange::bybit::config::ORDERBOOK_REST_URL;
use async_trait::async_trait;
use crypto_botters::bybit::BybitOption;

#[async_trait]
impl BybitMarket for BybitClient {
    async fn get_orderbook(
        &self,
        category: Category,
        symbol: &str,
        limit: &str,
    ) -> anyhow::Result<OrderbookData> {
        let query_params = vec![
            ("category", category.as_ref()),
            ("symbol", symbol),
            ("limit", limit),
        ];

        let response: BybitResponse<OrderbookData> = self
            .client
            .get(
                ORDERBOOK_REST_URL,
                Some(&query_params),
                [BybitOption::Default],
            )
            .await?;
        Ok(response.result)
    }
}
