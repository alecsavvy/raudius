use crate::client::Client;
use crate::error::Error;
use serde_json::Value;

#[derive(Debug, Default)]
pub struct TipQuery {
    pub offset: Option<i32>,
    pub limit: Option<i32>,
    pub user_id: Option<String>,
    pub receiver_min_followers: Option<i32>,
    pub receiver_is_verified: Option<bool>,
    pub current_user_follows: Option<String>,
    pub unique_by: Option<String>,
}

impl Client {
    pub async fn get_tips(&self, query: Option<TipQuery>) -> Result<Value, Error> {
        let url = "tips";
        match query {
            None => self.get(url).await,
            Some(query) => {
                self.query(url, |mut req| {
                    // add query params if they are present
                    // TODO: macroize this
                    if let Some(param) = query.offset {
                        req = req.query(&[("offset", param)]);
                    }
                    if let Some(param) = query.limit {
                        req = req.query(&[("limit", param)]);
                    }

                    if let Some(param) = &query.user_id {
                        req = req.query(&[("user_id", param)]);
                    }

                    if let Some(param) = query.receiver_min_followers {
                        req = req.query(&[("receiver_min_followers", param)]);
                    }

                    if let Some(param) = query.receiver_is_verified {
                        req = req.query(&[("receiver_is_verified", param)]);
                    }

                    if let Some(param) = &query.current_user_follows {
                        req = req.query(&[("current_user_follows", param)]);
                    }

                    if let Some(param) = &query.unique_by {
                        req = req.query(&[("unique_by", param)]);
                    }

                    req
                })
                .await
            }
        }
    }
}
