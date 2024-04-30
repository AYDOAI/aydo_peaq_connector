use crate::calls::http::Http;
use crate::errors::NodeError;
use std::future::Future;

pub trait Call: Sized {
    type ResultType;
    const METHOD: &'static str;

    fn request(&self, params: Option<Vec<String>>) -> Http {
        Http::new(Self::METHOD, params)
    }

    fn response(
        &self,
        url: &str,
        params: Option<Vec<String>>,
    ) -> impl Future<Output = Result<reqwest::Response, NodeError>> + Send
    where
        Self: Sync,
    {
        async {
            let http = self.request(params);
            match http.send(url).await {
                Ok(result) => Ok(result),
                Err(e) => Err(NodeError::CouldNotSendHttpsRequest(e.to_string())),
            }
        }
    }

    fn body(
        &self,
        url: &str,
        params: Option<Vec<String>>,
    ) -> impl Future<Output = Result<String, NodeError>> + Send
    where
        Self: Sync,
    {
        async {
            let response = self.response(url, params).await?;
            match response.text().await {
                Ok(body) => Ok(body),
                Err(e) => Err(NodeError::CouldNotSendHttpsRequest(e.to_string())),
            }
        }
    }

    fn get(
        &self,
        url: &str,
        params: Option<Vec<String>>,
    ) -> impl Future<Output = Result<Self::ResultType, NodeError>> + Send;
}
