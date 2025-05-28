use reqwest::{Client, ClientBuilder};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct EthResponse<T> {
    pub result: T,
}

#[derive(Clone)]
pub struct EthClient {
    client: Client,
    url: String,
}

impl EthClient {
    pub fn new(url: &str) -> Self {
        let client = ClientBuilder::new()
            .pool_max_idle_per_host(10)
            .build()
            .unwrap();
        Self {
            client,
            url: url.to_string(),
        }
    }

    async fn post_rpc<T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, reqwest::Error> {
        let req_body = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });
        let resp = self
            .client
            .post(&self.url)
            .json(&req_body)
            .send()
            .await?
            .json::<EthResponse<T>>()
            .await?;
        Ok(resp.result)
    }

    pub async fn block_number(&self) -> Result<u64, reqwest::Error> {
        let hex: String = self.post_rpc("eth_blockNumber", json!([])).await?;
        Ok(u64::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap())
    }

    pub async fn gas_price(&self) -> Result<u64, reqwest::Error> {
        let hex: String = self.post_rpc("eth_gasPrice", json!([])).await?;
        Ok(u64::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap())
    }

    pub async fn accounts(&self) -> Result<Vec<String>, reqwest::Error> {
        self.post_rpc("eth_accounts", json!([])).await
    }
}
