use crate::client::Client;
use crate::error::Error;
use serde_json::Value;

impl Client {
    pub async fn resolve(&self, url: &str) -> Result<Value, Error> {
        self.query("resolve", |req| req.query(&[("url", url)]))
            .await
    }
}
