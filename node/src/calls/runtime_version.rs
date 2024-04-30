use crate::calls::call::Call;
use crate::errors::NodeError;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StateGetRuntimeVersionApis {
    Int(u32),
    Str(String),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeData {
    apis: Vec<[StateGetRuntimeVersionApis; 2]>,
    authoring_version: u32,
    impl_name: String,
    impl_version: u32,
    spec_name: String,
    pub spec_version: u32,
    pub transaction_version: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RuntimeVersionSuccess {
    jsonrpc: String,
    result: RuntimeData,
    id: u8,
}

pub struct RuntimeVersion;

impl Call for RuntimeVersion {
    type ResultType = RuntimeData;
    const METHOD: &'static str = "state_getRuntimeVersion";

    async fn get(
        &self,
        url: &str,
        params: Option<Vec<String>>,
    ) -> Result<Self::ResultType, NodeError> {
        let body = self.body(url, params).await?;
        match serde_json::from_str::<RuntimeVersionSuccess>(&body) {
            Ok(nonce) => Ok(nonce.result),
            Err(e) => Err(NodeError::CouldNotGetAccountNonce(e.to_string())),
        }
    }
}
