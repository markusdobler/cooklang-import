use reqwest::Client;
use std::error::Error;
use std::time::Duration;

pub struct RequestFetcher {
    client: Client,
}

impl RequestFetcher {
    pub fn new(timeout: Option<Duration>, user_agent: Option<String>) -> Self {
        let timeout = timeout.unwrap_or(Duration::from_secs(30));
        let user_agent = user_agent.unwrap_or_else(|| {
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:149.0) Gecko/20100101 Firefox/149.0".to_string()
        });
        let client = Client::builder()
            .timeout(timeout)
            .user_agent(user_agent)
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub async fn fetch(&self, url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let response = self.client.get(url).send().await?;
        let status = response.status();
        if !status.is_success() {
            return Err(format!(
                "Failed to fetch page: HTTP {} ({})",
                status.as_u16(),
                status.canonical_reason().unwrap_or("Unknown")
            )
            .into());
        }
        let html = response.text().await?;
        Ok(html)
    }
}
