use crate::client::Client;
use crate::error::Error;
use openapi::models::{
    PlaylistResponse, PlaylistSearchResult, PlaylistTracksResponse, TrendingPlaylistsResponse,
};

impl Client {
    pub async fn get_playlist(&self, playlist_id: &str) -> Result<PlaylistResponse, Error> {
        self.get(&format!("playlists/{}", playlist_id)).await
    }

    pub async fn get_tranding_playlists(
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

    pub async fn get_playlist_tracks(
        &self,
        playlist_id: &str,
    ) -> Result<PlaylistTracksResponse, Error> {
        self.get(&format!("playlists/{}/tracks", playlist_id)).await
    }

    pub async fn search_playlists(&self, query: &str) -> Result<PlaylistSearchResult, Error> {
        self.query("playlists/search", |req| req.query(&[("query", query)]))
            .await
    }
}
