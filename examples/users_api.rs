use raudius::{client::Client, error::Error};

// runs through all the code samples in the docs
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().build().await?;

    client.get_user("nlGNe").await?;
    client
        .get_user_id("0x087F08462BbD30fC1775bBA3E58821F4CaD47b6b")
        .await?;
    client.search_users("Brownies").await?;
    client.verify_token("eyJ0eXAiOiJKV1QiLCJhbGciOiJrZWNjYWsyNTYifQ.eyJ1c2VySWQiOjE0MTYxMTUsImVtYWlsIjoiaXNhYWN0ZXN0NDUxQGdtYWlsLmNvbSIsIm5hbWUiOiJ0ZXN0aW5nMTIiLCJoYW5kbGUiOiJ0ZXN0dGVzdDQ1MSIsInZlcmlmaWVkIjpmYWxzZSwic3ViIjoxNDE2MTE1LCJpYXQiOjE2NTY1MTgzMzN9.MHhkZmYyYWY5ZThmNDAxZDUyZDlhNjUxNGRiOTg0ZjM5YjFhOTZkYmNmZmViZjMzZjNkNmEzMTk4OTVlZWE2MTZjNjg0NWIwOGEyOGQ4MTA4OTEyMTc4ZDU0ODRhZGU4M2I1Yzg4ZTUwM2Y3OGYzMDYzZjYxMmQxZDQwYTYwMGZmZDFi").await?;
    client.get_user_connected_wallets("nlGNe").await?;
    client.get_user_favorite_tracks("nlGNe").await?;
    client.get_user_reposts("nlGNe").await?;
    client.get_user_supporters("lzkyZ").await?;
    client.get_user_supportings("aW8mr").await?;
    client.get_user_most_used_track_tags("nlGNe").await?;
    client.get_user_tracks("nlGNe").await?;

    Ok(())
}
