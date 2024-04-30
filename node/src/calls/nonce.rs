use crate::calls::call::Call;
use crate::errors::NodeError;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct NonceSuccess {
    jsonrpc: String,
    result: u32,
    id: u8,
}

pub struct Nonce;

impl Call for Nonce {
    type ResultType = u32;
    const METHOD: &'static str = "system_accountNextIndex";

    async fn get(
        &self,
        url: &str,
        params: Option<Vec<String>>,
    ) -> Result<Self::ResultType, NodeError> {
        let body = self.body(url, params).await?;
        match serde_json::from_str::<NonceSuccess>(&body) {
            Ok(nonce) => Ok(nonce.result),
            Err(e) => Err(NodeError::CouldNotGetAccountNonce(e.to_string())),
        }
    }
}
