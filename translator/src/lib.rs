mod translator_error;

pub use translator_error::TranslatorError;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Translator {
    client: reqwest::Client,
    base_url: reqwest::Url,
}

impl Translator {
    pub fn new(base_url: &str) -> Result<Self, TranslatorError> {
        let base_url = reqwest::Url::parse(base_url)?;

        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
        })
    }

    #[tracing::instrument]
    pub async fn yoda(&self, text: String) -> Result<String, TranslatorError> {
        let url = self.base_url.join("yoda.json")?;
        let request = Request { text };
        let response: Response = self
            .client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.contents.translated)
    }

    #[tracing::instrument]
    pub async fn shakespeare(&self, text: String) -> Result<String, TranslatorError> {
        let url = self.base_url.join("shakespeare.json")?;
        let request = Request { text };
        let response: Response = self
            .client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.contents.translated)
    }
}

#[derive(Deserialize, Debug)]
struct Response {
    contents: Contents,
}

#[derive(Deserialize, Debug)]
struct Contents {
    translated: String,
}

#[derive(Serialize)]
struct Request {
    text: String,
}
