use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),

    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Other error: {0}")]
    Other(String),
}

pub struct HtmlDocument {
    document: Html,
}

impl HtmlDocument {
    pub fn new(html: &str) -> Self {
        Self {
            document: Html::parse_document(html),
        }
    }

    // 直接獲取內部的 Html 對象，以便於兼容原有代碼
    pub fn get_document(&self) -> &Html {
        &self.document
    }

    // 使用與原代碼相同的方式解析選擇器
    pub fn parse_selector(&self, selector: &str) -> Result<Selector, ScraperError> {
        Selector::parse(selector).map_err(|_| ScraperError::InvalidSelector(selector.to_string()))
    }
}

pub struct DataExtractor<'a> {
    document: &'a HtmlDocument,
}

impl<'a> DataExtractor<'a> {
    pub fn new(document: &'a HtmlDocument) -> Self {
        Self { document }
    }

    // 重新實現與原代碼更接近的提取方法
    pub fn extract_elements(
        &self,
        container_selector: &str,
        element_selector: &str,
    ) -> Result<Vec<scraper::ElementRef>, ScraperError> {
        let container = self.document.parse_selector(container_selector)?;
        let element = self.document.parse_selector(element_selector)?;

        // 與原代碼保持一致：先選擇一個容器，然後在容器內選擇元素
        let result = self
            .document
            .get_document()
            .select(&container)
            .next()
            .map(|container| container.select(&element).collect::<Vec<_>>())
            .unwrap_or_default();

        Ok(result)
    }

    pub fn extract_attribute_pairs(
        &self,
        container_selector: &str,
        element_selector: &str,
        first_attr: &str,
        second_attr: &str,
    ) -> Result<Vec<(String, String)>, ScraperError> {
        let elements = self.extract_elements(container_selector, element_selector)?;

        let pairs = elements
            .into_iter()
            .map(|el| {
                let first = el.attr(first_attr).unwrap_or("").to_string();
                let second = el.attr(second_attr).unwrap_or("").to_string();
                (first, second)
            })
            .filter(|(first, _)| !first.is_empty())
            .collect();

        Ok(pairs)
    }
}

pub struct WebScraper {
    client: Client,
}

impl WebScraper {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_page(&self, url: &str) -> Result<String, ScraperError> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| ScraperError::RequestError(e))?;
        let body = response
            .text()
            .await
            .map_err(|e| ScraperError::RequestError(e))?;
        Ok(body)
    }

    // 新增 POST 表單請求方法
    pub async fn post_form(
        &self,
        url: &str,
        params: HashMap<String, String>,
    ) -> Result<String, ScraperError> {
        let response = self
            .client
            .post(url)
            .form(&params)
            .send()
            .await
            .map_err(ScraperError::RequestError)?;

        let body = response.text().await.map_err(ScraperError::RequestError)?;

        Ok(body)
    }

    // 如果需要更靈活的配置，可以添加帶有額外選項的方法
    pub async fn post_form_with_headers(
        &self,
        url: &str,
        params: HashMap<String, String>,
        additional_headers: Option<HashMap<String, String>>,
    ) -> Result<String, ScraperError> {
        let mut request = self.client.post(url).form(&params);

        // 添加額外的請求頭
        if let Some(headers) = additional_headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        let response = request.send().await.map_err(ScraperError::RequestError)?;

        let body = response.text().await.map_err(ScraperError::RequestError)?;

        Ok(body)
    }
}
