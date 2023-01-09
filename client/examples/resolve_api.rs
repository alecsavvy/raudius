use client::{client::Client, error::Error};

// runs through all the code samples in the docs
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().build().await?;
    client
        .resolve("https://audius.co/camouflybeats/hypermantra-86216")
        .await?;
    Ok(())
}
