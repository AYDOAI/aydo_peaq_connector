use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Cound not connect to the blockchain node\n{0}")]
    CouldNotConnectToNode(String),
    #[error("Cound not close socket connection")]
    CouldNotCloseSocketConnection,
    #[error("Cound not send message to the node\n{0}")]
    CouldNotSendMessageToNode(String),
    #[error("Cound not read a response from the blockchain node\n{0}")]
    CouldNotReadMessageFromNode(String),
    #[error("Cound not get account nonce\n{0}")]
    CouldNotGetAccountNonce(String),
    #[error("Cound not get genesis hash\n{0}")]
    CouldNotGetGenesisHash(String),
    #[error("Cound not parse genesis hash\n{0}")]
    CouldNotParseGenesisHash(String),
    #[error("Cound not get metadata\n{0}")]
    CouldNotGetMetadata(String),
    #[error("Cound not decode metadata from hex to bytes\n{0}")]
    CouldNotDecodeMetadataHex(String),
    #[error("Cound not decode metadata from bytes\n{0}")]
    CouldNotDecodeMetadataBytes(String),
    #[error("Cound not decode latest metadata")]
    CouldNotDecodeMetadataLatest,
    #[error("Cound not decode metadata v14\n{0}")]
    CouldNotDecodeMetadataV14(String),
    #[error("Cound not get runtime version\n{0}")]
    CouldNotGetRuntimeVersion(String),
    #[error("Cound not get runtime metadata\n{0}")]
    CouldNotGetRuntimeMetadata(String),
    #[error("Cound get block data\n{0}")]
    CouldNotGetBlock(String),
    #[error("Cound not get storage value\n{0}")]
    CouldNotGetStorageValue(String),
    #[error("Cound not send an extrinsic transaction\n{0}")]
    CouldNotCallExtrinsic(String),
    #[error("Cound not get index for pallet \n{0}")]
    CouldNotGetPalletIndex(String),
    #[error("Cound not get index for method \n{0}")]
    CouldNotGetMethodIndex(String),

    #[error("Cound not send HTTP request to the node \n{0}")]
    CouldNotSendHttpsRequest(String),
}
