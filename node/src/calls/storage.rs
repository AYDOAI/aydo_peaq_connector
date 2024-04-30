use crate::calls::call::Call;
use crate::errors::NodeError;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StorageSuccess {
    Some(StorageSuccessSome),
    None(StorageSuccessNone),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct StorageSuccessSome {
    jsonrpc: String,
    result: String,
    id: u8,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct StorageSuccessNone {
    jsonrpc: String,
    result: Option<u32>,
    id: u8,
}

pub struct Storage;

impl Call for Storage {
    type ResultType = Option<String>;
    const METHOD: &'static str = "state_getStorage";

    async fn get(
        &self,
        url: &str,
        params: Option<Vec<String>>,
    ) -> Result<Self::ResultType, NodeError> {
        let body = self.body(url, params).await?;
        let res = match serde_json::from_str::<StorageSuccess>(&body) {
            Ok(nonce) => nonce,
            Err(e) => return Err(NodeError::CouldNotGetStorageValue(e.to_string())),
        };

        match res {
            StorageSuccess::Some(res) => Ok(Some(res.result)),
            StorageSuccess::None(_) => Ok(None),
        }
    }
}
