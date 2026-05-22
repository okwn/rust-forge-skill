use std::sync::Arc;
use thiserror::Error;
use {{workspace_name}}_core::Item;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("item not found")]
    NotFound,

    #[error("server error: {0}")]
    ServerError(String),
}

pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn list_items(&self) -> Result<Vec<Item>, ClientError> {
        let url = format!("{}/api/v1/items", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ClientError::NotFound);
        }

        let items = response.json::<Vec<Item>>().await?;
        Ok(items)
    }

    pub async fn create_item(&self, name: &str, description: Option<&str>) -> Result<Item, ClientError> {
        let url = format!("{}/api/v1/items", self.base_url);
        let payload = serde_json::json!({
            "name": name,
            "description": description
        });

        let response = self.client.post(&url).json(&payload).send().await?;

        if !response.status().is_success() {
            return Err(ClientError::ServerError(response.text().await?));
        }

        let item = response.json::<Item>().await?;
        Ok(item)
    }

    pub async fn get_item(&self, id: uuid::Uuid) -> Result<Item, ClientError> {
        let url = format!("{}/api/v1/items/{}", self.base_url, id);
        let response = self.client.get(&url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ClientError::NotFound);
        }

        let item = response.json::<Item>().await?;
        Ok(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = ApiClient::new("http://localhost:8080");
        // Would use wiremock for full tests
        assert_eq!(client.base_url, "http://localhost:8080");
    }
}