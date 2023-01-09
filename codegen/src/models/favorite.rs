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
pub struct Favorite {
    #[serde(rename = "favorite_item_id")]
    pub favorite_item_id: String,
    #[serde(rename = "favorite_type")]
    pub favorite_type: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl Favorite {
    pub fn new(favorite_item_id: String, favorite_type: String, user_id: String) -> Favorite {
        Favorite {
            favorite_item_id,
            favorite_type,
            user_id,
        }
    }
}


