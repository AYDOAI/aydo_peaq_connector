use crate::calls::call::Call;
use crate::errors::NodeError;
use serde::Deserialize;
use sp_core::H256;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BlockHashSuccess {
    jsonrpc: String,
    result: String,
    id: u8,
}

pub struct BlockHash;

impl Call for BlockHash {
    type ResultType = H256;
    const METHOD: &'static str = "chain_getBlockHash";

    async fn get(
        &self,
        url: &str,
        params: Option<Vec<String>>,
    ) -> Result<Self::ResultType, NodeError> {
        let body = self.body(url, params).await?;
        let res = match serde_json::from_str::<BlockHashSuccess>(&body) {
            Ok(nonce) => nonce.result,
            Err(e) => return Err(NodeError::CouldNotGetGenesisHash(e.to_string())),
        };

        match H256::from_str(res.as_str()) {
            Ok(hash) => Ok(hash),
            Err(e) => Err(NodeError::CouldNotParseGenesisHash(e.to_string())),
        }
    }
}
