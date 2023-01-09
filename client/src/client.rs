use crate::error::Error;
use reqwest::header::CONTENT_LENGTH;
use reqwest::RequestBuilder;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) host: String,
    pub(crate) app_name: Option<String>,
    pub(crate) client: reqwest::Client,
    pub(crate) version: String,
}

impl Client {
    pub fn new() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub(crate) async fn head(&self, path: &str) -> Result<String, Error> {
        let url = &format!("{}/{}/{}", &self.host, &self.version, path);
        let req = self.client.head(url);
        let res = req.send().await?;
        let res = res.headers();
        let content_length = res
            .get(CONTENT_LENGTH)
            .ok_or_else(|| Error::HeaderError("no content length in HEAD request".to_string()))?;
        let content_length = content_length
            .to_str()
            .map_err(|e| Error::HeaderError(e.to_string()))?
            .to_string();
        Ok(content_length)
    }

    pub(crate) async fn get<T>(&self, path: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.query(path, |req| req).await
    }

    pub(crate) async fn query<T, F>(&self, path: &str, f: F) -> Result<T, Error>
    where
        T: DeserializeOwned,
        F: Fn(RequestBuilder) -> RequestBuilder,
    {
        let url = &format!("{}/{}/{}", &self.host, &self.version, path);
        let mut req = self.client.get(url);
        if let Some(app) = &self.app_name {
            req = req.query(&[("app_name", app)]);
        }
        let req = f(req);
        let res = req.send().await?.json().await?;
        Ok(res)
    }
}

pub struct ClientBuilder {
    host: Option<String>,
    app_name: Option<String>,
    client: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            app_name: None,
            client: None,
        }
    }

    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    pub fn app_name(mut self, app_name: String) -> Self {
        self.app_name = Some(app_name);
        self
    }

    async fn discover_host() -> Result<String, Error> {
        use rand::seq::SliceRandom;
        let hosts = reqwest::Client::new()
            .get("https://api.audius.co")
            .send()
            .await?
            .json::<Hosts>()
            .await?
            .data;
        let host = hosts
            .choose(&mut rand::thread_rng())
            .ok_or_else(|| Error::BuilderError("no hosts found".to_string()))?;
        Ok(host.to_owned())
    }

    pub async fn build(self) -> Result<Client, Error> {
        let client = self.client.unwrap_or_default();
        let app_name = self.app_name;
        let host = match &self.host {
            Some(host) => host.to_owned(),
            None => ClientBuilder::discover_host().await?,
        };
        Ok(Client {
            host,
            client,
            app_name,
            version: "v1".to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct Hosts {
    data: Vec<String>,
}
