use crate::{
    calls::{call::Call, runtime_metadata::RuntimeMetadata},
    errors::NodeError,
};
use frame_metadata::v14::RuntimeMetadataV14;
use scale_info::TypeDef::Variant;
use std::future::Future;

pub struct Metadata(RuntimeMetadataV14);

impl Metadata {
    async fn new(url: &str) -> Result<Self, NodeError> {
        let metadata_v14 = RuntimeMetadata.get(url, None).await?;
        Ok(Metadata(metadata_v14))
    }

    fn get(self) -> RuntimeMetadataV14 {
        self.0
    }
}

pub struct Pallet(String);
pub struct Method(String);

impl Pallet {
    pub fn new(name: &str) -> Self {
        Pallet(name.to_string())
    }
}

impl Method {
    pub fn new(name: &str) -> Self {
        Method(name.to_string())
    }
}

pub trait Searchable: Sized {
    fn get_index(&self, url: &str) -> impl Future<Output = Result<u8, NodeError>> + Send;
}

impl Searchable for Pallet {
    async fn get_index(&self, url: &str) -> Result<u8, NodeError> {
        let index = Metadata::new(url)
            .await?
            .get()
            .pallets
            .iter()
            .find(|pred| pred.name.eq(&self.0))
            .map(|pred| pred.index);

        match index {
            Some(index) => Ok(index),
            None => Err(NodeError::CouldNotGetPalletIndex(self.0.clone())),
        }
    }
}

impl Searchable for Method {
    async fn get_index(&self, url: &str) -> Result<u8, NodeError> {
        let metadata = Metadata::new(url).await?.get().types.types;
        let mut iter = metadata.iter();

        while let Some(item) = iter.next() {
            if let Variant(res) = &item.ty.type_def {
                let index = res
                    .variants
                    .iter()
                    .find(|pred| pred.name.eq(&self.0))
                    .map(|pred| pred.index);
                if let Some(index) = index {
                    return Ok(index);
                }
            }
        }

        Err(NodeError::CouldNotGetPalletIndex(self.0.clone()))
    }
}
