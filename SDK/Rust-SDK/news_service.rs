use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::{ApiTubeClient, ApiTubeError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Option<u64>,
    pub domain: Option<String>,
    pub home_page_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub href: Option<String>,
    pub source: Source,
    pub published_at: Option<String>,
    pub language: Option<String>,
    pub categories: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsResponse {
    pub status: String,
    pub page: u32,
    pub limit: u32,
    pub results: Vec<Article>,
}

#[derive(Debug, Clone, Default)]
pub struct NewsQueryParams {
    pub per_page: Option<u32>,
    pub offset: Option<u32>,
    pub language: Option<String>,
    pub categories: Option<String>,
    pub published_at: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl NewsQueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn categories(mut self, categories: impl Into<String>) -> Self {
        self.categories = Some(categories.into());
        self
    }

    fn to_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();

        if let Some(per_page) = self.per_page {
            params.insert("per_page".to_string(), per_page.to_string());
        }
        if let Some(offset) = self.offset {
            params.insert("offset".to_string(), offset.to_string());
        }
        if let Some(ref language) = self.language {
            params.insert("language.code".to_string(), language.clone());
        }
        if let Some(ref categories) = self.categories {
            params.insert("category.id".to_string(), categories.clone());
        }
        if let Some(ref published_at) = self.published_at {
            params.insert("published_at".to_string(), published_at.clone());
        }
        if let Some(ref sort_by) = self.sort_by {
            params.insert("sort.by".to_string(), sort_by.clone());
        }
        if let Some(ref sort_order) = self.sort_order {
            params.insert("sort.order".to_string(), sort_order.clone());
        }

        params
    }
}

pub struct ApiTubeNewsService<'a> {
    client: &'a ApiTubeClient,
}

impl<'a> ApiTubeNewsService<'a> {
    pub fn new(client: &'a ApiTubeClient) -> Self {
        Self { client }
    }

    pub async fn get_everything(&self, params: NewsQueryParams) -> Result<NewsResponse, ApiTubeError> {
        self.client
            .request("/everything", Some(params.to_params()))
            .await
    }

    pub async fn get_top_headlines(&self, params: NewsQueryParams) -> Result<NewsResponse, ApiTubeError> {
        self.client
            .request("/top-headlines", Some(params.to_params()))
            .await
    }

    pub async fn get_by_category(
        &self,
        category: &str,
        params: NewsQueryParams,
    ) -> Result<NewsResponse, ApiTubeError> {
        let params = params.categories(category);
        self.get_everything(params).await
    }

    pub async fn get_by_language(
        &self,
        language: &str,
        params: NewsQueryParams,
    ) -> Result<NewsResponse, ApiTubeError> {
        let params = params.language(language);
        self.get_everything(params).await
    }
}
