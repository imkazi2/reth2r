/// Represents _all_ transaction requests to/from RPC.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(alias = "TxRequest")]
pub struct RR2TransactionRequest {
    /// The address of the transaction author.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Address>,
    /// The destination address of the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<TxKind>,
    /// The legacy gas price.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub gas_price: Option<u128>,
    /// The max base fee per gas the sender is willing to pay.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub max_fee_per_gas: Option<u128>,
    /// The max priority fee per gas the sender is willing to pay, also called the miner tip.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub max_priority_fee_per_gas: Option<u128>,
    /// The max fee per blob gas for EIP-4844 blob transactions.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub max_fee_per_blob_gas: Option<u128>,
    /// The gas limit for the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub gas: Option<u128>,
    /// The value transferred in the transaction, in wei.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    /// Transaction data.
    #[serde(default, flatten)]
    pub input: TransactionInput,
    /// The nonce of the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub nonce: Option<u64>,
    /// The chain ID for the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub chain_id: Option<ChainId>,
    /// An EIP-2930 access list, which lowers cost for accessing accounts and storages in the list. See [EIP-2930](https://eips.ethereum.org/EIPS/eip-2930) for more information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_list: Option<AccessList>,
    /// The EIP-2718 transaction type. See [EIP-2718](https://eips.ethereum.org/EIPS/eip-2718) for more information.
    #[serde(
        default,
        rename = "type",
        skip_serializing_if = "Option::is_none",
        with = "alloy_serde::quantity::opt"
    )]
    #[doc(alias = "tx_type")]
    pub transaction_type: Option<u8>,
    /// Blob versioned hashes for EIP-4844 transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob_versioned_hashes: Option<Vec<B256>>,
    /// Blob sidecar for EIP-4844 transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidecar: Option<BlobTransactionSidecar>,
    #[serde(default)]
    pub enable_access_list: bool,
    #[serde(default)]
    pub enable_logs: bool,
}