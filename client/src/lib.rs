pub async fn get_user() {
    use openapi::models::UserResponse;
    let result = reqwest::get("https://blockdaemon-audius-discovery-02.bdnodes.net/v1/users/nlGNe")
        .await
        .unwrap()
        .json::<UserResponse>()
        .await
        .unwrap();

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        get_user().await;
    }
}
