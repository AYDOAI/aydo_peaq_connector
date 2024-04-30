use crate::calls::call::Call;
use crate::errors::NodeError;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BlockSuccess {
    jsonrpc: String,
    result: BlockResult,
    id: u8,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BlockResult {
    block: BlockData,
    justifications: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct BlockData {
    extrinsics: Vec<String>,
    header: BlockHeader,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockHeader {
    digest: BlockHeaderDigest,
    extrinsics_root: String,
    number: String,
    parent_hash: String,
    state_root: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct BlockHeaderDigest {
    logs: Vec<String>,
}

pub struct Block;

impl Call for Block {
    type ResultType = BlockResult;
    const METHOD: &'static str = "chain_getBlock";

    async fn get(
        &self,
        url: &str,
        params: Option<Vec<String>>,
    ) -> Result<Self::ResultType, NodeError> {
        let body = self.body(url, params).await?;
        match serde_json::from_str::<BlockSuccess>(&body) {
            Ok(nonce) => Ok(nonce.result),
            Err(e) => Err(NodeError::CouldNotGetAccountNonce(e.to_string())),
        }
    }
}
