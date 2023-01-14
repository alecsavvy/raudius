use crate::client::Client;
use crate::error::Error;
use crate::models::{
    PlaylistResponse, PlaylistSearchResult, PlaylistTracksResponse, TrendingPlaylistsResponse,
};

impl Client {
    /// Get a playlist.
    pub async fn get_playlist(&self, playlist_id: &str) -> Result<PlaylistResponse, Error> {
        self.get(&format!("playlists/{}", playlist_id)).await
    }

    /// Get trending playlists based on the time query.
    pub async fn get_trending_playlists(
        &self,
        time: Option<&str>,
    ) -> Result<TrendingPlaylistsResponse, Error> {
        self.query("playlists/tranding", |req| {
            if let Some(time) = time {
                req.query(&[("time", time)])
            } else {
                req
            }
        })
        .await
    }

    /// Get tracks in a playlist.
    pub async fn get_playlist_tracks(
        &self,
        playlist_id: &str,
    ) -> Result<PlaylistTracksResponse, Error> {
        self.get(&format!("playlists/{}/tracks", playlist_id)).await
    }

    /// Search for playlists on an arbitrary query string.
    pub async fn search_playlists(&self, query: &str) -> Result<PlaylistSearchResult, Error> {
        self.query("playlists/search", |req| req.query(&[("query", query)]))
            .await
    }
}
