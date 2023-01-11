use raudius::{client::Client, error::Error};

// runs through all the code samples in the docs
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().build().await?;

    client.get_playlist("DOPRl").await?;
    client.search_playlists("Hot & New").await?;
    client.get_tranding_playlists(None).await?;
    client.get_tranding_playlists(Some("week")).await?;
    client.get_tranding_playlists(Some("month")).await?;
    client.get_tranding_playlists(Some("year")).await?;
    client.get_tranding_playlists(Some("allTime")).await?;
    client.get_playlist_tracks("DOPRl").await?;

    Ok(())
}
