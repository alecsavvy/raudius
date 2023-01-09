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
pub struct UserResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::User>>,
}

impl UserResponse {
    pub fn new() -> UserResponse {
        UserResponse {
            data: None,
        }
    }
}


