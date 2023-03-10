use crate::client::Client;
use crate::error::Error;
use crate::models::{TrackResponse, TrackSearch, TracksResponse};

impl Client {
    /// Get a track.
    pub async fn get_track(&self, track_id: &str) -> Result<TrackResponse, Error> {
        self.get(&format!("tracks/{}", track_id)).await
    }

    /// Search for tracks.
    pub async fn search_tracks(
        &self,
        query: &str,
        only_downloadable: Option<bool>,
    ) -> Result<TrackSearch, Error> {
        self.query("tracks/search", |mut req| {
            req = req.query(&[("query", query)]);
            if let Some(dl) = only_downloadable {
                req = req.query(&[("only_downloadable", dl)]);
            }
            req
        })
        .await
    }

    /// Get pending tracks based on genre, time, or a combination.
    pub async fn get_trending_tracks(
        &self,
        genre: Option<&str>,
        time: Option<&str>,
    ) -> Result<TracksResponse, Error> {
        self.query("tracks/trending", |mut req| {
            if let Some(genre) = genre {
                req = req.query(&[("genre", genre)]);
            }

            if let Some(time) = time {
                req = req.query(&[("time", time)]);
            }
            req
        })
        .await
    }

    /// Get multiple tracks.
    pub async fn get_bulk_tracks(
        &self,
        permalink: Option<Vec<&str>>,
        id: Option<Vec<&str>>,
    ) -> Result<TracksResponse, Error> {
        self.query("tracks", |mut req| {
            if let Some(permalink) = &permalink {
                for link in permalink {
                    req = req.query(&[("permalink", link)]);
                }
            }
            if let Some(id) = &id {
                for id in id {
                    req = req.query(&[("id", id)]);
                }
            }
            req
        })
        .await
    }

    /// Get MP3 bytes of a track. Does not chunk the request. This method will download the entire track.
    ///
    /// * `max_bytes` - if the track HEAD request is larger than this the method will return an error.
    pub async fn stream_track(
        &self,
        track_id: &str,
        max_bytes: Option<u128>,
    ) -> Result<Vec<u8>, Error> {
        let url = &format!("tracks/{}/stream", track_id);
        let content_size = self.head(url).await?;
        let content_size = content_size
            .parse::<u128>()
            .map_err(|e| Error::HeaderError(e.to_string()))?;
        if let Some(max_bytes) = max_bytes {
            if max_bytes > content_size {
                return Err(Error::TrackTooLarge(format!(
                    "file too large at {} bytes when max bytes is set to {}",
                    content_size, max_bytes
                )));
            }
        };
        let track = self.get_raw(url).await?;
        Ok(track.to_vec())
    }
}
