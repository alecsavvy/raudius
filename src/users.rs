use crate::models::*;
use crate::{client::Client, error::Error};

impl Client {
    /// Get a user.
    pub async fn get_user(&self, user_id: &str) -> Result<UserResponse, Error> {
        self.get(&format!("users/{}", user_id)).await
    }

    /// Get a users connected wallets.
    pub async fn get_user_connected_wallets(
        &self,
        user_id: &str,
    ) -> Result<UserAssociatedWalletResponse, Error> {
        self.get(&format!("users/{}/connected_wallets", user_id))
            .await
    }

    // Get a users favorite tracks.
    pub async fn get_user_favorite_tracks(
        &self,
        user_id: &str,
    ) -> Result<FavoritesResponse, Error> {
        self.get(&format!("users/{}/favorites", user_id)).await
    }

    /// Get a users reposts.
    pub async fn get_user_reposts(&self, user_id: &str) -> Result<Reposts, Error> {
        self.get(&format!("users/{}/reposts", user_id)).await
    }

    /// Get a users supporters.
    pub async fn get_user_supporters(&self, user_id: &str) -> Result<GetSupporters, Error> {
        self.get(&format!("users/{}/supporters", user_id)).await
    }

    /// Get who supports a specific user.
    pub async fn get_user_supportings(&self, user_id: &str) -> Result<GetSupporting, Error> {
        self.get(&format!("users/{}/supporting", user_id)).await
    }

    /// Get a users most used tags.
    pub async fn get_user_most_used_track_tags(
        &self,
        user_id: &str,
    ) -> Result<TagsResponse, Error> {
        self.get(&format!("users/{}/tags", user_id)).await
    }

    /// Get a users tracks.
    pub async fn get_user_tracks(&self, user_id: &str) -> Result<TracksResponse, Error> {
        self.get(&format!("users/{}/tracks", user_id)).await
    }

    /// Get a users id based on a wallet address.
    pub async fn get_user_id(
        &self,
        wallet_address: &str,
    ) -> Result<UserAssociatedWalletResponse, Error> {
        self.query("users/id", |req| {
            req.query(&[("associated_wallet", wallet_address)])
        })
        .await
    }

    /// Search for users on an arbitrary query string.
    pub async fn search_users(&self, query: &str) -> Result<UserSearch, Error> {
        self.query("users/search", |req| req.query(&[("query", query)]))
            .await
    }

    /// Verify the token.
    pub async fn verify_token(&self, token: &str) -> Result<VerifyToken, Error> {
        self.query("users/verify_token", |req| req.query(&[("token", token)]))
            .await
    }
}
