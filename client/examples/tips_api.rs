use client::{client::Client, error::Error, tips::TipQuery};

// runs through all the code samples in the docs
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().build().await?;

    client.get_tips(None).await?;

    let query = TipQuery::default();
    client.get_tips(Some(query)).await?;

    let query = TipQuery {
        user_id: Some("nlGNe".to_owned()),
        ..Default::default()
    };
    client.get_tips(Some(query)).await?;

    Ok(())
}
