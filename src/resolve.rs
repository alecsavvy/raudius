use crate::client::Client;
use crate::error::Error;

impl Client {
    pub async fn resolve(&self, url: &str) -> Result<(), Error> {
        self.query("resolve", |req| req.query(&[("url", url)]))
            .await?;
        Ok(())
    }
}
