use crate::calls::call::Call;
use crate::errors::NodeError;
use serde::Deserialize;

#[allow(dead_code)]
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ExtrinsicSuccess {
    jsonrpc: String,
    result: String,
    id: u8,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ExtrinsicResult {
    Success(ExtrinsicSuccess),
    Error(ExtrinsicError),
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ExtrinsicError {
    jsonrpc: String,
    error: ExtrinsicErrorBody,
    id: u8,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ExtrinsicErrorBody {
    code: usize,
    message: String,
    data: String,
}

pub struct Extrinsic;

impl Call for Extrinsic {
    type ResultType = String;
    const METHOD: &'static str = "author_submitExtrinsic";

    async fn get(
        &self,
        url: &str,
        params: Option<Vec<String>>,
    ) -> Result<Self::ResultType, NodeError> {
        let body = self.body(url, params).await?;
        let res = match serde_json::from_str::<ExtrinsicResult>(&body) {
            Ok(res) => res,
            Err(e) => return Err(NodeError::CouldNotCallExtrinsic(e.to_string())),
        };

        match res {
            ExtrinsicResult::Success(res) => Ok(res.result),
            ExtrinsicResult::Error(e) => {
                let error = format!("{}. {}", e.error.message, e.error.data);
                return Err(NodeError::CouldNotCallExtrinsic(error));
            }
        }
    }
}
