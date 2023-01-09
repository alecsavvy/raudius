use client::{client::Client, error::Error};

// runs through all the code samples in the docs
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().build().await?;

    client.get_track("D7KyD").await?;
    client.search_tracks("baauer b2b", None).await?;
    client.search_tracks("baauer b2b", Some(true)).await?;
    client.search_tracks("baauer b2b", Some(false)).await?;
    client.get_trending_tracks(None, None).await?;
    client.get_trending_tracks(Some("metal"), None).await?;
    client.get_trending_tracks(None, Some("week")).await?;
    client
        .get_trending_tracks(Some("jazz"), Some("week"))
        .await?;

    // get bulk tracks not supported yet
    Ok(())
}
