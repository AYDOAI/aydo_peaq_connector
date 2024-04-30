use crate::calls::call::Call;
use crate::errors::NodeError;
use codec::Decode;
use frame_metadata::{v14::RuntimeMetadataV14, RuntimeMetadataPrefixed};
use serde::Deserialize;

pub struct RuntimeMetadata;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct RuntimeMetadataSuccess {
    jsonrpc: String,
    result: String,
    id: u8,
}

impl Call for RuntimeMetadata {
    type ResultType = RuntimeMetadataV14;
    const METHOD: &'static str = "state_getMetadata";

    async fn get(
        &self,
        url: &str,
        params: Option<Vec<String>>,
    ) -> Result<Self::ResultType, NodeError> {
        let body = self.body(url, params).await?;
        let metadata_hex = match serde_json::from_str::<RuntimeMetadataSuccess>(&body) {
            Ok(data) => data.result,
            Err(e) => return Err(NodeError::CouldNotGetRuntimeMetadata(e.to_string())),
        };

        let metadata_bytes = match hex::decode(&metadata_hex.trim_start_matches("0x")) {
            Ok(bytes) => bytes,
            Err(e) => return Err(NodeError::CouldNotDecodeMetadataHex(e.to_string())),
        };

        let (_, runntime_metadata) =
            match RuntimeMetadataPrefixed::decode(&mut metadata_bytes.as_slice()) {
                Ok(res) => (res.0, res.1),
                Err(e) => return Err(NodeError::CouldNotDecodeMetadataBytes(e.to_string())),
            };

        match runntime_metadata {
            frame_metadata::RuntimeMetadata::V14(metadata) => Ok(metadata),
            _ => Err(NodeError::CouldNotDecodeMetadataLatest),
        }
    }
}
