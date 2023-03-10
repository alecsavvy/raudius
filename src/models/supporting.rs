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
pub struct Supporting {
    #[serde(rename = "rank")]
    pub rank: i32,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "receiver")]
    pub receiver: Box<crate::models::User>,
}

impl Supporting {
    pub fn new(rank: i32, amount: String, receiver: crate::models::User) -> Supporting {
        Supporting {
            rank,
            amount,
            receiver: Box::new(receiver),
        }
    }
}


