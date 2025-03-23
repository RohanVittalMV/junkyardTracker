use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlResponse {
    pub html: String,
    pub status: u16,
    pub url: String,
}

#[derive(Debug)]
pub enum Error {
    RequestFailed(reqwest::Error),
    ApiError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RequestFailed(e) => write!(f, "Request failed: {}", e),
            Error::ApiError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl StdError for Error {}

pub struct FirecrawlClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl FirecrawlClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.firecrawl.com".to_string(),
            client: Client::new(),
        }
    }

    pub async fn crawl_webpage(&self, url: &str) -> Result<CrawlResponse, Error> {
        let response = self
            .client
            .post(&format!("{}/v1/crawl", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "url": url,
                "render_js": true,
                "wait_for": 2000  // Wait 2 seconds for JS to load
            }))
            .send()
            .await
            .map_err(Error::RequestFailed)?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ApiError(format!(
                "API returned error status: {}, message: {}",
                response.status(),
                error_text
            )));
        }

        let crawl_response = response.json::<CrawlResponse>().await
            .map_err(Error::RequestFailed)?;

        Ok(crawl_response)
    }
}