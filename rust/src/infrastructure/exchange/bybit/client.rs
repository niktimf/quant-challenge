use crypto_botters::bybit::{BybitHttpUrl, BybitOption};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BybitClient {
    pub client: Arc<crypto_botters::Client>,
}

impl BybitClient {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        let mut client = crypto_botters::Client::new();
        client.update_default_option(BybitOption::Key(api_key.to_string()));
        client.update_default_option(BybitOption::Secret(secret_key.to_string()));
        #[cfg(feature = "demo")]
        client.update_default_option(BybitOption::HttpUrl(BybitHttpUrl::Test));
        #[cfg(feature = "prod")]
        client.update_default_option(BybitOption::HttpUrl(BybitHttpUrl::Bybit));

        Self {
            client: Arc::new(client),
        }
    }
}
