use alloy::consensus::{Signed, Typed2718};
use alloy::network::eip2718::{Decodable2718, Encodable2718};
use alloy::rlp::{Encodable, Header};
use serde::{Deserialize, Serialize};

use super::tx_type::TxType;
use super::unsigned_tx::eip712::TxEip712;
/// Transaction envelope is a wrapper around the transaction data.
/// See [`alloy::consensus::TxEnvelope`](https://docs.rs/alloy/latest/alloy/consensus/enum.TxEnvelope.html) for more details.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    into = "serde_from::TaggedTxEnvelope",
    from = "serde_from::MaybeTaggedTxEnvelope"
)]
pub enum TxEnvelope {
    /// Ethereum-native transaction.
    Native(alloy::consensus::TxEnvelope),
    /// ZKsync-native EIP712 transaction.
    Eip712(Signed<TxEip712>),
}

impl TxEnvelope {
    /// Returns true if the transaction is a legacy transaction.
    #[inline]
    pub const fn is_legacy(&self) -> bool {
        match self {
            Self::Native(inner) => inner.is_legacy(),
            Self::Eip712(_) => false,
        }
    }

    /// Returns true if the transaction is an EIP-2930 transaction.
    #[inline]
    pub const fn is_eip2930(&self) -> bool {
        match self {
            Self::Native(inner) => inner.is_eip2930(),
            Self::Eip712(_) => false,
        }
    }

    /// Returns true if the transaction is an EIP-1559 transaction.
    #[inline]
    pub const fn is_eip1559(&self) -> bool {
        match self {
            Self::Native(inner) => inner.is_eip1559(),
            Self::Eip712(_) => false,
        }
    }

    /// Returns true if the transaction is an EIP-4844 transaction.
    #[inline]
    pub const fn is_eip4844(&self) -> bool {
        match self {
            Self::Native(inner) => inner.is_eip4844(),
            Self::Eip712(_) => false,
        }
    }

    /// Returns true if the transaction is an EIP-7702 transaction.
    #[inline]
    pub const fn is_eip7702(&self) -> bool {
        match self {
            Self::Native(inner) => inner.is_eip7702(),
            Self::Eip712(_) => false,
        }
    }

    /// Returns true if the transaction is an EIP-712 transaction.
    #[inline]
    pub const fn is_eip712(&self) -> bool {
        matches!(self, Self::Eip712(_))
    }

    /// Returns true if the transaction is replay protected.
    ///
    /// All non-legacy transactions are replay protected, as the chain id is
    /// included in the transaction body. Legacy transactions are considered
    /// replay protected if the `v` value is not 27 or 28, according to the
    /// rules of [EIP-155].
    ///
    /// [EIP-155]: https://eips.ethereum.org/EIPS/eip-155
    #[inline]
    pub const fn is_replay_protected(&self) -> bool {
        match self {
            Self::Native(inner) => inner.is_replay_protected(),
            Self::Eip712(_) => true,
        }
    }

    /// Returns the [`TxLegacy`] variant if the transaction is a legacy transaction.
    pub const fn as_legacy(&self) -> Option<&Signed<alloy::consensus::TxLegacy>> {
        match self {
            Self::Native(inner) => inner.as_legacy(),
            Self::Eip712(_) => None,
        }
    }

    /// Returns the [`TxEip2930`] variant if the transaction is an EIP-2930 transaction.
    pub const fn as_eip2930(&self) -> Option<&Signed<alloy::consensus::TxEip2930>> {
        match self {
            Self::Native(inner) => inner.as_eip2930(),
            Self::Eip712(_) => None,
        }
    }

    /// Returns the [`TxEip1559`] variant if the transaction is an EIP-1559 transaction.
    pub const fn as_eip1559(&self) -> Option<&Signed<alloy::consensus::TxEip1559>> {
        match self {
            Self::Native(inner) => inner.as_eip1559(),
            Self::Eip712(_) => None,
        }
    }

    /// Returns the [`TxEip4844`] variant if the transaction is an EIP-4844 transaction.
    pub const fn as_eip4844(&self) -> Option<&Signed<alloy::consensus::TxEip4844Variant>> {
        match self {
            Self::Native(inner) => inner.as_eip4844(),
            Self::Eip712(_) => None,
        }
    }

    /// Returns the [`TxEip7702`] variant if the transaction is an EIP-7702 transaction.
    pub const fn as_eip7702(&self) -> Option<&Signed<alloy::consensus::TxEip7702>> {
        match self {
            Self::Native(inner) => inner.as_eip7702(),
            Self::Eip712(_) => None,
        }
    }

    /// Returns the [`TxEip712`] variant if the transaction is an EIP-712 transaction.
    pub const fn as_eip712(&self) -> Option<&Signed<TxEip712>> {
        match self {
            Self::Native(_) => None,
            Self::Eip712(inner) => Some(inner),
        }
    }

    /// Calculate the signing hash for the transaction.
    pub fn signature_hash(&self) -> alloy::primitives::B256 {
        match self {
            Self::Native(inner) => inner.signature_hash(),
            Self::Eip712(inner) => inner.signature_hash(),
        }
    }

    /// Return the reference to signature.
    pub const fn signature(&self) -> &alloy::primitives::Signature {
        match self {
            Self::Native(inner) => inner.signature(),
            Self::Eip712(inner) => inner.signature(),
        }
    }

    /// Return the hash of the inner Signed.
    #[doc(alias = "transaction_hash")]
    pub fn tx_hash(&self) -> &alloy::primitives::B256 {
        match self {
            Self::Native(inner) => inner.tx_hash(),
            Self::Eip712(inner) => inner.hash(),
        }
    }

    /// Return the [`TxType`] of the inner txn.
    #[doc(alias = "transaction_type")]
    pub const fn tx_type(&self) -> crate::network::tx_type::TxType {
        match self {
            Self::Native(inner) => match inner.tx_type() {
                alloy::consensus::TxType::Legacy => crate::network::tx_type::TxType::Legacy,
                alloy::consensus::TxType::Eip2930 => crate::network::tx_type::TxType::Eip2930,
                alloy::consensus::TxType::Eip1559 => crate::network::tx_type::TxType::Eip1559,
                alloy::consensus::TxType::Eip4844 => crate::network::tx_type::TxType::Eip4844,
                alloy::consensus::TxType::Eip7702 => crate::network::tx_type::TxType::Eip7702,
            },
            Self::Eip712(_) => crate::network::tx_type::TxType::Eip712,
        }
    }

    /// Return the length of the inner txn, including type byte length
    pub fn eip2718_encoded_length(&self) -> usize {
        match self {
            Self::Native(inner) => inner.eip2718_encoded_length(),
            Self::Eip712(inner) => inner.tx().encoded_length(inner.signature()),
        }
    }
}

impl Typed2718 for TxEnvelope {
    fn ty(&self) -> u8 {
        match self {
            Self::Native(inner) => inner.ty(),
            Self::Eip712(inner) => inner.tx().tx_type() as u8,
        }
    }
}

impl Encodable2718 for TxEnvelope {
    fn type_flag(&self) -> Option<u8> {
        match self {
            Self::Native(inner) => inner.type_flag(),
            Self::Eip712(inner) => Some(inner.tx().tx_type() as u8),
        }
    }

    fn encode_2718_len(&self) -> usize {
        match self {
            Self::Native(inner) => inner.encode_2718_len(),
            Self::Eip712(inner) => {
                let payload_length = inner.tx().fields_len()
                    + inner.signature().rlp_rs_len()
                    + inner.signature().v().length();
                Header {
                    list: true,
                    payload_length,
                }
                .length()
                    + payload_length
            }
        }
    }

    fn encode_2718(&self, out: &mut dyn alloy::primitives::bytes::BufMut) {
        match self {
            Self::Native(inner) => inner.encode_2718(out),
            Self::Eip712(tx) => {
                tx.tx().encode_with_signature(tx.signature(), out);
            }
        }
    }
}

impl Decodable2718 for TxEnvelope {
    fn typed_decode(ty: u8, buf: &mut &[u8]) -> alloy::network::eip2718::Eip2718Result<Self> {
        match ty {
            _ if ty == (TxType::Eip712 as u8) => {
                let tx = TxEip712::decode_signed_fields(buf)?;
                Ok(Self::Eip712(tx))
            }
            _ => {
                let inner = alloy::consensus::TxEnvelope::typed_decode(ty, buf)?;
                Ok(Self::Native(inner))
            }
        }
    }

    fn fallback_decode(buf: &mut &[u8]) -> alloy::network::eip2718::Eip2718Result<Self> {
        let inner = alloy::consensus::TxEnvelope::fallback_decode(buf)?;
        Ok(Self::Native(inner))
    }
}

impl AsRef<dyn alloy::consensus::Transaction> for TxEnvelope {
    fn as_ref(&self) -> &dyn alloy::consensus::Transaction {
        match self {
            TxEnvelope::Native(inner) => inner,
            TxEnvelope::Eip712(signed_inner) => signed_inner.tx(),
        }
    }
}

impl alloy::consensus::Transaction for TxEnvelope {
    fn chain_id(&self) -> Option<alloy::primitives::ChainId> {
        self.as_ref().chain_id()
    }

    fn nonce(&self) -> u64 {
        self.as_ref().nonce()
    }

    fn gas_limit(&self) -> u64 {
        self.as_ref().gas_limit()
    }

    fn gas_price(&self) -> Option<u128> {
        self.as_ref().gas_price()
    }

    fn max_fee_per_gas(&self) -> u128 {
        self.as_ref().max_fee_per_gas()
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        self.as_ref().max_priority_fee_per_gas()
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        self.as_ref().max_fee_per_blob_gas()
    }

    fn priority_fee_or_price(&self) -> u128 {
        self.as_ref().priority_fee_or_price()
    }

    fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        self.as_ref().effective_gas_price(base_fee)
    }

    fn is_dynamic_fee(&self) -> bool {
        self.as_ref().is_dynamic_fee()
    }

    fn kind(&self) -> alloy::primitives::TxKind {
        self.as_ref().kind()
    }

    fn is_create(&self) -> bool {
        self.as_ref().is_create()
    }

    fn value(&self) -> alloy::primitives::U256 {
        self.as_ref().value()
    }

    fn input(&self) -> &alloy::primitives::Bytes {
        self.as_ref().input()
    }

    fn access_list(&self) -> Option<&alloy::rpc::types::AccessList> {
        self.as_ref().access_list()
    }

    fn blob_versioned_hashes(&self) -> Option<&[alloy::primitives::B256]> {
        self.as_ref().blob_versioned_hashes()
    }

    fn authorization_list(&self) -> Option<&[alloy::eips::eip7702::SignedAuthorization]> {
        self.as_ref().authorization_list()
    }
}

mod serde_from {
    //! NB: Why do we need this?
    //!
    //! We are following the same approach as [`alloy::consensus::TxEnvelope`] but with an additional
    //! ZKsync-specific transaction type (`type: "0x71"`).
    //!
    //! Because the tag may be missing, we need an abstraction over tagged (with
    //! type) and untagged (always legacy). This is [`MaybeTaggedTxEnvelope`].
    //!
    //! The tagged variant is [`TaggedTxEnvelope`], which always has a type tag.
    //!
    //! We serialize via [`TaggedTxEnvelope`] and deserialize via
    //! [`MaybeTaggedTxEnvelope`].
    //!
    //! zkSync RPC responses may omit `accessList` on type-2 (EIP-1559) transactions.
    //! The deserialization path handles this by defaulting it to an empty list.
    use crate::network::tx_envelope::TxEnvelope;
    use crate::network::unsigned_tx::eip712::TxEip712;
    use alloy::consensus::{Signed, TxEip1559, TxEip2930, TxEip4844Variant, TxEip7702, TxLegacy};

    #[derive(Debug, serde::Deserialize)]
    #[serde(untagged)]
    pub(crate) enum MaybeTaggedTxEnvelope {
        Tagged(TaggedTxEnvelopeDe),
        Untagged {
            #[serde(
                default,
                rename = "type",
                deserialize_with = "alloy::serde::reject_if_some"
            )]
            _ty: Option<()>,
            #[serde(flatten, with = "alloy::consensus::transaction::signed_legacy_serde")]
            tx: Signed<TxLegacy>,
        },
    }

    /// Serialization-only tagged envelope.
    ///
    /// Keep only the canonical type tags here. Alternate tag aliases matter
    /// only on deserialization and live on [`TaggedTxEnvelopeDe`].
    #[derive(Debug, serde::Serialize)]
    #[serde(tag = "type")]
    pub(crate) enum TaggedTxEnvelope {
        #[serde(
            rename = "0x0",
            with = "alloy::consensus::transaction::signed_legacy_serde"
        )]
        Legacy(Signed<TxLegacy>),
        #[serde(rename = "0x1")]
        Eip2930(Signed<TxEip2930>),
        #[serde(rename = "0x2")]
        Eip1559(Signed<TxEip1559>),
        #[serde(rename = "0x3")]
        Eip4844(Signed<TxEip4844Variant>),
        #[serde(rename = "0x4")]
        Eip7702(Signed<TxEip7702>),
        #[serde(rename = "0x71")]
        Eip712(Signed<TxEip712>),
    }

    /// Deserialization-only tagged envelope.
    ///
    /// Separate from [`TaggedTxEnvelope`] because zkSync RPC responses may omit
    /// `accessList` on type-2 (EIP-1559) transactions. This type uses a permissive
    /// deserializer that defaults the field to an empty list when missing.
    #[derive(Debug, serde::Deserialize)]
    #[serde(tag = "type")]
    pub(crate) enum TaggedTxEnvelopeDe {
        #[serde(
            rename = "0x0",
            alias = "0x00",
            with = "alloy::consensus::transaction::signed_legacy_serde"
        )]
        Legacy(Signed<TxLegacy>),
        #[serde(rename = "0x1", alias = "0x01")]
        Eip2930(Signed<TxEip2930>),
        #[serde(
            rename = "0x2",
            alias = "0x02",
            deserialize_with = "deserialize_eip1559_permissive"
        )]
        Eip1559(Signed<TxEip1559>),
        #[serde(rename = "0x3", alias = "0x03")]
        Eip4844(Signed<TxEip4844Variant>),
        #[serde(rename = "0x4", alias = "0x04")]
        Eip7702(Signed<TxEip7702>),
        #[serde(rename = "0x71")]
        Eip712(Signed<TxEip712>),
    }

    /// Deserializes `Signed<TxEip1559>`, defaulting `accessList` to `[]` when missing.
    ///
    /// Uses a mirror of `TxEip1559` with `#[serde(default)]` on `access_list`
    /// to avoid an intermediate `serde_json::Value` round-trip.
    fn deserialize_eip1559_permissive<'de, D>(
        deserializer: D,
    ) -> Result<Signed<TxEip1559>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use alloy::eips::eip2930::AccessList;
        use alloy::primitives::{B256, Bytes, ChainId, Signature, TxKind, U256};
        use serde::Deserialize;

        /// Mirrors [`TxEip1559`] but defaults `access_list` to empty when missing.
        ///
        /// Keep this in lockstep with alloy's `TxEip1559` serde shape. The
        /// conversion below will fail to compile on field/type additions, but
        /// serde attribute changes upstream still need manual review here.
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct TxEip1559Permissive {
            #[serde(with = "alloy::serde::quantity")]
            chain_id: ChainId,
            #[serde(with = "alloy::serde::quantity")]
            nonce: u64,
            #[serde(with = "alloy::serde::quantity", rename = "gas", alias = "gasLimit")]
            gas_limit: u64,
            #[serde(with = "alloy::serde::quantity")]
            max_fee_per_gas: u128,
            #[serde(with = "alloy::serde::quantity")]
            max_priority_fee_per_gas: u128,
            #[serde(default)]
            to: TxKind,
            value: U256,
            #[serde(default, deserialize_with = "alloy::serde::null_as_default")]
            access_list: AccessList,
            input: Bytes,
        }

        /// Mirrors alloy's internal `Signed` serde helper with the permissive
        /// transaction type flattened alongside the signature.
        #[derive(Deserialize)]
        struct SignedHelper {
            #[serde(flatten)]
            tx: TxEip1559Permissive,
            #[serde(flatten)]
            signature: Signature,
            hash: B256,
        }

        let helper = SignedHelper::deserialize(deserializer)?;
        let tx = TxEip1559 {
            chain_id: helper.tx.chain_id,
            nonce: helper.tx.nonce,
            gas_limit: helper.tx.gas_limit,
            max_fee_per_gas: helper.tx.max_fee_per_gas,
            max_priority_fee_per_gas: helper.tx.max_priority_fee_per_gas,
            to: helper.tx.to,
            value: helper.tx.value,
            access_list: helper.tx.access_list,
            input: helper.tx.input,
        };
        // Match alloy's RPC deserialization path: trust the RPC-provided hash
        // and signature material instead of recomputing or verifying here.
        Ok(Signed::new_unchecked(tx, helper.signature, helper.hash))
    }

    impl From<MaybeTaggedTxEnvelope> for TxEnvelope {
        fn from(value: MaybeTaggedTxEnvelope) -> Self {
            match value {
                MaybeTaggedTxEnvelope::Tagged(tagged) => tagged.into(),
                MaybeTaggedTxEnvelope::Untagged { tx, .. } => {
                    Self::Native(alloy::consensus::TxEnvelope::Legacy(tx))
                }
            }
        }
    }

    impl From<TaggedTxEnvelopeDe> for TxEnvelope {
        fn from(value: TaggedTxEnvelopeDe) -> Self {
            match value {
                TaggedTxEnvelopeDe::Legacy(signed) => {
                    Self::Native(alloy::consensus::TxEnvelope::Legacy(signed))
                }
                TaggedTxEnvelopeDe::Eip2930(signed) => {
                    Self::Native(alloy::consensus::TxEnvelope::Eip2930(signed))
                }
                TaggedTxEnvelopeDe::Eip1559(signed) => {
                    Self::Native(alloy::consensus::TxEnvelope::Eip1559(signed))
                }
                TaggedTxEnvelopeDe::Eip4844(signed) => {
                    Self::Native(alloy::consensus::TxEnvelope::Eip4844(signed))
                }
                TaggedTxEnvelopeDe::Eip7702(signed) => {
                    Self::Native(alloy::consensus::TxEnvelope::Eip7702(signed))
                }
                TaggedTxEnvelopeDe::Eip712(signed) => Self::Eip712(signed),
            }
        }
    }

    impl From<TxEnvelope> for TaggedTxEnvelope {
        fn from(value: TxEnvelope) -> Self {
            match value {
                TxEnvelope::Native(alloy::consensus::TxEnvelope::Legacy(signed)) => {
                    Self::Legacy(signed)
                }
                TxEnvelope::Native(alloy::consensus::TxEnvelope::Eip2930(signed)) => {
                    Self::Eip2930(signed)
                }
                TxEnvelope::Native(alloy::consensus::TxEnvelope::Eip1559(signed)) => {
                    Self::Eip1559(signed)
                }
                TxEnvelope::Native(alloy::consensus::TxEnvelope::Eip4844(signed)) => {
                    Self::Eip4844(signed)
                }
                TxEnvelope::Native(alloy::consensus::TxEnvelope::Eip7702(signed)) => {
                    Self::Eip7702(signed)
                }
                TxEnvelope::Eip712(signed) => Self::Eip712(signed),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::transaction_response::TransactionResponse;
    use alloy::consensus::Transaction;

    /// Round-trip test: serialize a canonical `TxEip1559` via alloy's own
    /// `Serialize`, then deserialize through our `TxEnvelope` permissive path.
    ///
    /// Guards against silent serde attribute drift between alloy's `TxEip1559`
    /// and our `TxEip1559Permissive` mirror struct.
    #[test]
    fn eip1559_roundtrip_through_permissive_path() {
        use alloy::consensus::{Signed, TxEip1559};
        use alloy::eips::eip2930::AccessList;
        use alloy::primitives::{Address, B256, Bytes, Signature, U256};

        let tx = TxEip1559 {
            chain_id: 324,
            nonce: 42,
            gas_limit: 21_000,
            max_fee_per_gas: 4_000_000_000,
            max_priority_fee_per_gas: 1_000_000,
            to: Address::repeat_byte(0xab).into(),
            value: U256::from(1_000_000_000_000_000_000u128),
            access_list: AccessList::default(),
            input: Bytes::from_static(b"\xde\xad"),
        };

        let sig = Signature::test_signature();
        let hash = B256::repeat_byte(0xff);
        let signed = Signed::new_unchecked(tx.clone(), sig, hash);

        // Wrap in TaggedTxEnvelope (our serialization type) to get `"type": "0x2"`.
        let envelope = TxEnvelope::Native(alloy::consensus::TxEnvelope::Eip1559(signed));
        let json = serde_json::to_string(&envelope).expect("serialize");

        // Deserialize back through our permissive path.
        let roundtripped: TxEnvelope = serde_json::from_str(&json).expect("deserialize");
        let inner = roundtripped.as_eip1559().expect("should be EIP-1559").tx();

        assert_eq!(inner.chain_id, tx.chain_id);
        assert_eq!(inner.nonce, tx.nonce);
        assert_eq!(inner.gas_limit, tx.gas_limit);
        assert_eq!(inner.max_fee_per_gas, tx.max_fee_per_gas);
        assert_eq!(inner.max_priority_fee_per_gas, tx.max_priority_fee_per_gas);
        assert_eq!(inner.to, tx.to);
        assert_eq!(inner.value, tx.value);
        assert_eq!(inner.access_list, tx.access_list);
        assert_eq!(inner.input, tx.input);
    }

    /// Regression test: zkSync RPC responses may omit `accessList` on type-2
    /// (EIP-1559) transactions. This reproduces the failure reported in
    /// <https://github.com/matter-labs/alloy-zksync/issues/66>.
    #[test]
    fn deserialize_eip1559_without_access_list() {
        let json = r#"
        {
            "type": "0x2",
            "chainId": "0x144",
            "nonce": "0x5",
            "gas": "0x5208",
            "maxFeePerGas": "0xee6b2800",
            "maxPriorityFeePerGas": "0x0",
            "to": "0x1234567890abcdef1234567890abcdef12345678",
            "value": "0xde0b6b3a7640000",
            "input": "0x",
            "r": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "s": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "v": "0x0",
            "yParity": "0x0",
            "hash": "0x09d047b22ceb10d30bd1a36969e45eb9f63b6d01f16439f4fd0b9f0114177cff"
        }
        "#;

        let envelope: TxEnvelope = serde_json::from_str(json).unwrap();
        assert!(envelope.is_eip1559());
        let access_list = envelope
            .access_list()
            .expect("should have defaulted to empty");
        assert!(access_list.is_empty());
    }

    /// Verify that EIP-1559 transactions WITH `accessList` still deserialize.
    #[test]
    fn deserialize_eip1559_with_access_list() {
        let json = r#"
        {
            "type": "0x2",
            "chainId": "0x144",
            "nonce": "0x5",
            "gas": "0x5208",
            "maxFeePerGas": "0xee6b2800",
            "maxPriorityFeePerGas": "0x0",
            "to": "0x1234567890abcdef1234567890abcdef12345678",
            "value": "0xde0b6b3a7640000",
            "input": "0x",
            "accessList": [],
            "r": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "s": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "v": "0x0",
            "yParity": "0x0",
            "hash": "0x09d047b22ceb10d30bd1a36969e45eb9f63b6d01f16439f4fd0b9f0114177cff"
        }
        "#;

        let envelope: TxEnvelope = serde_json::from_str(json).unwrap();
        assert!(envelope.is_eip1559());
    }

    /// zkSync has also been observed to return `accessList: null`; accept it
    /// the same way alloy does for other permissive RPC providers.
    #[test]
    fn deserialize_eip1559_with_null_access_list() {
        let json = r#"
        {
            "type": "0x2",
            "chainId": "0x144",
            "nonce": "0x5",
            "gas": "0x5208",
            "maxFeePerGas": "0xee6b2800",
            "maxPriorityFeePerGas": "0x0",
            "to": "0x1234567890abcdef1234567890abcdef12345678",
            "value": "0xde0b6b3a7640000",
            "input": "0x",
            "accessList": null,
            "r": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "s": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "v": "0x0",
            "yParity": "0x0",
            "hash": "0x09d047b22ceb10d30bd1a36969e45eb9f63b6d01f16439f4fd0b9f0114177cff"
        }
        "#;

        let envelope: TxEnvelope = serde_json::from_str(json).unwrap();
        assert!(envelope.is_eip1559());
        let access_list = envelope
            .access_list()
            .expect("should have defaulted null to empty");
        assert!(access_list.is_empty());
    }

    /// EIP-712 transactions without `accessList` should also deserialize.
    #[test]
    fn deserialize_eip712_without_access_list() {
        let json = r#"
        {
            "type": "0x71",
            "blockHash": "0x05a9dc0814c1bf7dc96c464663bf24db8e1306e2814c848c87de2ea0684dadb5",
            "blockNumber": "0x41e64c0",
            "chainId": "0x144",
            "from": "0xaede212cda8fc657ac7b4ad5e98fd510624638b3",
            "gas": "0x388d8",
            "gasPrice": "0x2b275d0",
            "hash": "0xfb0fac3fccaaec0a1863e87941ba68752e92f9190f036f1af2b2f485e5829d28",
            "input": "0x51cff8d9000000000000000000000000aede212cda8fc657ac7b4ad5e98fd510624638b3",
            "l1BatchNumber": "0x7c0c8",
            "l1BatchTxIndex": "0x140",
            "maxFeePerGas": "0x2b275d0",
            "maxPriorityFeePerGas": "0x2b275d0",
            "nonce": "0x31db5",
            "r": "0x3e6d7b9325a8ef7b707a02f287c91d7d07b06bb1ad16d645ba35703bc5804936",
            "s": "0x6a8a76d975aec0c560233be60e13bfa2233a95ee7d2262d8c119ec6589e1bc26",
            "to": "0x000000000000000000000000000000000000800a",
            "transactionIndex": "0x0",
            "v": "0x0",
            "value": "0x1"
        }
        "#;

        let envelope: TxEnvelope = serde_json::from_str(json).unwrap();
        assert!(envelope.is_eip712());
    }

    /// Regression test at the `TransactionResponse` level: a full RPC response
    /// for a type-2 transaction without `accessList` should deserialize through
    /// the `TransactionEither` untagged enum path.
    #[test]
    fn deserialize_transaction_response_without_access_list() {
        let json = r#"
        {
            "type": "0x2",
            "blockHash": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "blockNumber": "0x41a3a9b",
            "transactionIndex": "0x0",
            "hash": "0x09d047b22ceb10d30bd1a36969e45eb9f63b6d01f16439f4fd0b9f0114177cff",
            "from": "0x1234567890abcdef1234567890abcdef12345678",
            "to": "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
            "value": "0xde0b6b3a7640000",
            "nonce": "0x5",
            "gas": "0x5208",
            "maxFeePerGas": "0xee6b2800",
            "maxPriorityFeePerGas": "0x0",
            "input": "0x",
            "chainId": "0x144",
            "r": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "s": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            "v": "0x0",
            "yParity": "0x0"
        }
        "#;

        let response: TransactionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.nonce(), 5);
        assert_eq!(response.gas_limit(), 21000);
        assert_eq!(response.max_fee_per_gas(), 0xee6b2800);
        let access_list = response
            .access_list()
            .expect("should have defaulted to empty");
        assert!(access_list.is_empty());
    }
}
