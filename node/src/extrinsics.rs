use codec::{Codec, Compact, Encode};
use sp_core::crypto::AccountId32;
use sp_core::sr25519;
use sp_core::Pair;
use sp_runtime::{generic::Era, MultiAddress, MultiSignature};

use crate::calls::{
    block_hash::BlockHash, call::Call, nonce::Nonce, runtime_version::RuntimeVersion,
};

use crate::errors::NodeError;
use crate::metadata::Searchable;
use crate::metadata::{Method, Pallet};

pub struct Extrinsic<T: ExtrinsicCall> {
    pair: sr25519::Pair,
    call: T::Call,
}

impl<T: ExtrinsicCall> Extrinsic<T> {
    fn owner(&self) -> AccountId32 {
        AccountId32::from(self.pair.public())
    }
}

impl<T: ExtrinsicCall> Extrinsic<T> {
    pub async fn build(&self, url: &str) -> Result<String, NodeError> {
        let nonce = Nonce.get(url, Some(vec![self.owner().to_string()])).await?;
        let genesis_hash = BlockHash.get(url, Some(vec!["0".to_string()])).await?;
        let runtime = RuntimeVersion.get(url, None).await?;
        let indexes = T::indexes(url).await?;

        let extra = (Era::Immortal, Compact(nonce), Compact(0u128));
        let additional = (
            runtime.spec_version,
            runtime.transaction_version,
            genesis_hash,
            genesis_hash,
        );

        let payload = (indexes, &self.call, &extra, &additional).encode();
        let signature = match payload.len() > 256 {
            true => self.pair.sign(&sp_core::blake2_256(&payload).as_slice()),
            false => self.pair.sign(&payload),
        };

        let extrinsic = self.encode_extrinsic(signature, extra, indexes);
        let extrinsic_hash = format!("0x{}", hex::encode(extrinsic));
        Ok(extrinsic_hash)
    }

    fn encode_extrinsic(
        &self,
        signature: sr25519::Signature,
        extra: (Era, Compact<u32>, Compact<u128>),
        indexes: [u8; 2],
    ) -> Vec<u8> {
        let extrinsic = {
            let mut tmp = Vec::new();

            (0b1000_0000 + 4u8).encode_to(&mut tmp);
            MultiAddress::Id::<_, u32>(self.owner()).encode_to(&mut tmp);
            MultiSignature::Sr25519(signature).encode_to(&mut tmp);

            extra.encode_to(&mut tmp);
            let call = (indexes, &self.call);
            call.encode_to(&mut tmp);

            let len = Compact(tmp.len() as u32);
            let mut encoded = Vec::new();
            len.encode_to(&mut encoded);
            encoded.extend(&tmp);
            encoded
        };
        extrinsic
    }
}

pub struct AddAttribute {}

pub trait ExtrinsicCall: Sized {
    type Call: Codec;
    const PALLET: &'static str;
    const METHOD: &'static str;

    fn new(pair: sr25519::Pair, call: Self::Call) -> Extrinsic<Self> {
        Extrinsic { pair, call }
    }

    fn indexes(url: &str) -> impl std::future::Future<Output = Result<[u8; 2], NodeError>> + Send {
        async {
            Ok([
                Pallet::new(Self::PALLET).get_index(url).await?,
                Method::new(Self::METHOD).get_index(url).await?,
            ])
        }
    }
}

impl ExtrinsicCall for AddAttribute {
    type Call = (AccountId32, Vec<u8>, Vec<u8>, Option<u32>);
    const PALLET: &'static str = "PeaqDid";
    const METHOD: &'static str = "add_attribute";
}
