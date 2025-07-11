use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlResponse {
    pub success: bool,
    pub data: Option<CrawlData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlData {
    pub markdown: Option<String>,
    pub html: Option<String>,
    pub url: Option<String>,
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
            base_url: "https://api.firecrawl.dev".to_string(),
            client: Client::new(),
        }
    }

    pub async fn crawl_webpage(&self, url: &str) -> Result<CrawlResponse, Error> {
        let response = self
            .client
            .post(&format!("{}/v1/scrape", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "url": url,
                "formats": ["markdown"],
                "onlyMainContent": true,
                "parsePDF": true,
                "maxAge": 14400000
            }))
            .send()
            .await
            .map_err(Error::RequestFailed)?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ApiError(format!(
                "API returned error status: {}, message: {}",
                status,
                error_text
            )));
        }

        let crawl_response = response.json::<CrawlResponse>().await
            .map_err(Error::RequestFailed)?;

        Ok(crawl_response)
    }
}