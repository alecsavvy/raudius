/*
 * API
 *
 * Audius V1 API
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Track {
    #[serde(rename = "artwork", skip_serializing_if = "Option::is_none")]
    pub artwork: Option<Box<crate::models::TrackArtwork>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "genre", skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "mood", skip_serializing_if = "Option::is_none")]
    pub mood: Option<String>,
    #[serde(rename = "release_date", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(rename = "remix_of", skip_serializing_if = "Option::is_none")]
    pub remix_of: Option<Box<crate::models::RemixParent>>,
    #[serde(rename = "repost_count")]
    pub repost_count: i32,
    #[serde(rename = "favorite_count")]
    pub favorite_count: i32,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "user")]
    pub user: Box<crate::models::User>,
    #[serde(rename = "duration")]
    pub duration: i32,
    #[serde(rename = "downloadable", skip_serializing_if = "Option::is_none")]
    pub downloadable: Option<bool>,
    #[serde(rename = "play_count")]
    pub play_count: i32,
    #[serde(rename = "permalink", skip_serializing_if = "Option::is_none")]
    pub permalink: Option<String>,
}

impl Track {
    pub fn new(id: String, repost_count: i32, favorite_count: i32, title: String, user: crate::models::User, duration: i32, play_count: i32) -> Track {
        Track {
            artwork: None,
            description: None,
            genre: None,
            id,
            mood: None,
            release_date: None,
            remix_of: None,
            repost_count,
            favorite_count,
            tags: None,
            title,
            user: Box::new(user),
            duration,
            downloadable: None,
            play_count,
            permalink: None,
        }
    }
}


