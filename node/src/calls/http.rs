use crate::errors::NodeError;
use reqwest::Response;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Http {
    id: u8,
    jsonrpc: String,
    method: String,
    params: Vec<String>,
}

impl Http {
    pub fn new(method: &str, params: Option<Vec<String>>) -> Self {
        let params = match params {
            Some(p) => p,
            None => Vec::new(),
        };
        Http {
            id: 1,
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
        }
    }

    pub async fn send(&self, url: &str) -> Result<Response, NodeError> {
        let response = reqwest::Client::new().post(url).json(&self).send().await;

        match response {
            Ok(result) => Ok(result),
            Err(e) => Err(NodeError::CouldNotSendHttpsRequest(e.to_string())),
        }
    }
}

trait Call {}
