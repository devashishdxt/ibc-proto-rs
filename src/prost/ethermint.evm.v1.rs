/// Params defines the EVM module parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// evm_denom represents the token denomination used to run the EVM state
    /// transitions.
    #[prost(string, tag = "1")]
    pub evm_denom: ::prost::alloc::string::String,
    /// enable_create toggles state transitions that use the vm.Create function
    #[prost(bool, tag = "2")]
    pub enable_create: bool,
    /// enable_call toggles state transitions that use the vm.Call function
    #[prost(bool, tag = "3")]
    pub enable_call: bool,
    /// extra_eips defines the additional EIPs for the vm.Config
    #[prost(int64, repeated, packed = "false", tag = "4")]
    pub extra_eips: ::prost::alloc::vec::Vec<i64>,
    /// chain_config defines the EVM chain configuration parameters
    #[prost(message, optional, tag = "5")]
    pub chain_config: ::core::option::Option<ChainConfig>,
    /// allow_unprotected_txs defines if replay-protected (i.e non EIP155
    /// signed) transactions can be executed on the state machine.
    #[prost(bool, tag = "6")]
    pub allow_unprotected_txs: bool,
}
/// ChainConfig defines the Ethereum ChainConfig parameters using *sdk.Int values
/// instead of *big.Int.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainConfig {
    /// homestead_block switch (nil no fork, 0 = already homestead)
    #[prost(string, tag = "1")]
    pub homestead_block: ::prost::alloc::string::String,
    /// dao_fork_block corresponds to TheDAO hard-fork switch block (nil no fork)
    #[prost(string, tag = "2")]
    pub dao_fork_block: ::prost::alloc::string::String,
    /// dao_fork_support defines whether the nodes supports or opposes the DAO hard-fork
    #[prost(bool, tag = "3")]
    pub dao_fork_support: bool,
    /// eip150_block: EIP150 implements the Gas price changes
    /// (<https://github.com/ethereum/EIPs/issues/150>) EIP150 HF block (nil no fork)
    #[prost(string, tag = "4")]
    pub eip150_block: ::prost::alloc::string::String,
    /// eip150_hash: EIP150 HF hash (needed for header only clients as only gas pricing changed)
    #[prost(string, tag = "5")]
    pub eip150_hash: ::prost::alloc::string::String,
    /// eip155_block: EIP155Block HF block
    #[prost(string, tag = "6")]
    pub eip155_block: ::prost::alloc::string::String,
    /// eip158_block: EIP158 HF block
    #[prost(string, tag = "7")]
    pub eip158_block: ::prost::alloc::string::String,
    /// byzantium_block: Byzantium switch block (nil no fork, 0 = already on byzantium)
    #[prost(string, tag = "8")]
    pub byzantium_block: ::prost::alloc::string::String,
    /// constantinople_block: Constantinople switch block (nil no fork, 0 = already activated)
    #[prost(string, tag = "9")]
    pub constantinople_block: ::prost::alloc::string::String,
    /// petersburg_block: Petersburg switch block (nil same as Constantinople)
    #[prost(string, tag = "10")]
    pub petersburg_block: ::prost::alloc::string::String,
    /// istanbul_block: Istanbul switch block (nil no fork, 0 = already on istanbul)
    #[prost(string, tag = "11")]
    pub istanbul_block: ::prost::alloc::string::String,
    /// muir_glacier_block: Eip-2384 (bomb delay) switch block (nil no fork, 0 = already activated)
    #[prost(string, tag = "12")]
    pub muir_glacier_block: ::prost::alloc::string::String,
    /// berlin_block: Berlin switch block (nil = no fork, 0 = already on berlin)
    #[prost(string, tag = "13")]
    pub berlin_block: ::prost::alloc::string::String,
    /// london_block: London switch block (nil = no fork, 0 = already on london)
    #[prost(string, tag = "17")]
    pub london_block: ::prost::alloc::string::String,
    /// arrow_glacier_block: Eip-4345 (bomb delay) switch block (nil = no fork, 0 = already activated)
    #[prost(string, tag = "18")]
    pub arrow_glacier_block: ::prost::alloc::string::String,
    /// gray_glacier_block: EIP-5133 (bomb delay) switch block (nil = no fork, 0 = already activated)
    #[prost(string, tag = "20")]
    pub gray_glacier_block: ::prost::alloc::string::String,
    /// merge_netsplit_block: Virtual fork after The Merge to use as a network splitter
    #[prost(string, tag = "21")]
    pub merge_netsplit_block: ::prost::alloc::string::String,
    /// shanghai_block switch block (nil = no fork, 0 = already on shanghai)
    #[prost(string, tag = "22")]
    pub shanghai_block: ::prost::alloc::string::String,
    /// cancun_block switch block (nil = no fork, 0 = already on cancun)
    #[prost(string, tag = "23")]
    pub cancun_block: ::prost::alloc::string::String,
}
/// State represents a single Storage key value pair item.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    /// key is the stored key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// value is the stored value for the given key
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// TransactionLogs define the logs generated from a transaction execution
/// with a given hash. It it used for import/export data as transactions are not
/// persisted on blockchain state after an upgrade.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionLogs {
    /// hash of the transaction
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// logs is an array of Logs for the given transaction hash
    #[prost(message, repeated, tag = "2")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
/// Log represents an protobuf compatible Ethereum Log that defines a contract
/// log event. These events are generated by the LOG opcode and stored/indexed by
/// the node.
///
/// NOTE: address, topics and data are consensus fields. The rest of the fields
/// are derived, i.e. filled in by the nodes, but not secured by consensus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    /// address of the contract that generated the event
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// topics is a list of topics provided by the contract.
    #[prost(string, repeated, tag = "2")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// data which is supplied by the contract, usually ABI-encoded
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// block_number of the block in which the transaction was included
    #[prost(uint64, tag = "4")]
    pub block_number: u64,
    /// tx_hash is the transaction hash
    #[prost(string, tag = "5")]
    pub tx_hash: ::prost::alloc::string::String,
    /// tx_index of the transaction in the block
    #[prost(uint64, tag = "6")]
    pub tx_index: u64,
    /// block_hash of the block in which the transaction was included
    #[prost(string, tag = "7")]
    pub block_hash: ::prost::alloc::string::String,
    /// index of the log in the block
    #[prost(uint64, tag = "8")]
    pub index: u64,
    /// removed is true if this log was reverted due to a chain
    /// reorganisation. You must pay attention to this field if you receive logs
    /// through a filter query.
    #[prost(bool, tag = "9")]
    pub removed: bool,
}
/// TxResult stores results of Tx execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResult {
    /// contract_address contains the ethereum address of the created contract (if
    /// any). If the state transition is an evm.Call, the contract address will be
    /// empty.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// bloom represents the bloom filter bytes
    #[prost(bytes = "vec", tag = "2")]
    pub bloom: ::prost::alloc::vec::Vec<u8>,
    /// tx_logs contains the transaction hash and the proto-compatible ethereum
    /// logs.
    #[prost(message, optional, tag = "3")]
    pub tx_logs: ::core::option::Option<TransactionLogs>,
    /// ret defines the bytes from the execution.
    #[prost(bytes = "vec", tag = "4")]
    pub ret: ::prost::alloc::vec::Vec<u8>,
    /// reverted flag is set to true when the call has been reverted
    #[prost(bool, tag = "5")]
    pub reverted: bool,
    /// gas_used notes the amount of gas consumed while execution
    #[prost(uint64, tag = "6")]
    pub gas_used: u64,
}
/// AccessTuple is the element type of an access list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTuple {
    /// address is a hex formatted ethereum address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// storage_keys are hex formatted hashes of the storage keys
    #[prost(string, repeated, tag = "2")]
    pub storage_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// TraceConfig holds extra parameters to trace functions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceConfig {
    /// tracer is a custom javascript tracer
    #[prost(string, tag = "1")]
    pub tracer: ::prost::alloc::string::String,
    /// timeout overrides the default timeout of 5 seconds for JavaScript-based tracing
    /// calls
    #[prost(string, tag = "2")]
    pub timeout: ::prost::alloc::string::String,
    /// reexec defines the number of blocks the tracer is willing to go back
    #[prost(uint64, tag = "3")]
    pub reexec: u64,
    /// disable_stack switches stack capture
    #[prost(bool, tag = "5")]
    pub disable_stack: bool,
    /// disable_storage switches storage capture
    #[prost(bool, tag = "6")]
    pub disable_storage: bool,
    /// debug can be used to print output during capture end
    #[prost(bool, tag = "8")]
    pub debug: bool,
    /// limit defines the maximum length of output, but zero means unlimited
    #[prost(int32, tag = "9")]
    pub limit: i32,
    /// overrides can be used to execute a trace using future fork rules
    #[prost(message, optional, tag = "10")]
    pub overrides: ::core::option::Option<ChainConfig>,
    /// enable_memory switches memory capture
    #[prost(bool, tag = "11")]
    pub enable_memory: bool,
    /// enable_return_data switches the capture of return data
    #[prost(bool, tag = "12")]
    pub enable_return_data: bool,
    /// tracer_json_config configures the tracer using a JSON string
    #[prost(string, tag = "13")]
    pub tracer_json_config: ::prost::alloc::string::String,
}
/// MsgEthereumTx encapsulates an Ethereum transaction as an SDK message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEthereumTx {
    /// data is inner transaction data of the Ethereum transaction
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<super::super::super::google::protobuf::Any>,
    /// size is the encoded storage size of the transaction (DEPRECATED)
    #[prost(double, tag = "2")]
    pub size: f64,
    /// hash of the transaction in hex format
    #[prost(string, tag = "3")]
    pub hash: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag = "4")]
    pub deprecated_from: ::prost::alloc::string::String,
    /// from is the bytes of ethereum signer address. This address value is checked
    /// against the address derived from the signature (V, R, S) using the
    /// secp256k1 elliptic curve
    #[prost(bytes = "vec", tag = "5")]
    pub from: ::prost::alloc::vec::Vec<u8>,
}
/// LegacyTx is the transaction data of regular Ethereum transactions.
/// NOTE: All non-protected transactions (i.e non EIP155 signed) will fail if the
/// AllowUnprotectedTxs parameter is disabled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyTx {
    /// nonce corresponds to the account nonce (transaction sequence).
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
    /// gas_price defines the value for each gas unit
    #[prost(string, tag = "2")]
    pub gas_price: ::prost::alloc::string::String,
    /// gas defines the gas limit defined for the transaction.
    #[prost(uint64, tag = "3")]
    pub gas: u64,
    /// to is the hex formatted address of the recipient
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
    /// value defines the unsigned integer value of the transaction amount.
    #[prost(string, tag = "5")]
    pub value: ::prost::alloc::string::String,
    /// data is the data payload bytes of the transaction.
    #[prost(bytes = "vec", tag = "6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// v defines the signature value
    #[prost(bytes = "vec", tag = "7")]
    pub v: ::prost::alloc::vec::Vec<u8>,
    /// r defines the signature value
    #[prost(bytes = "vec", tag = "8")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    /// s define the signature value
    #[prost(bytes = "vec", tag = "9")]
    pub s: ::prost::alloc::vec::Vec<u8>,
}
/// AccessListTx is the data of EIP-2930 access list transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessListTx {
    /// chain_id of the destination EVM chain
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// nonce corresponds to the account nonce (transaction sequence).
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
    /// gas_price defines the value for each gas unit
    #[prost(string, tag = "3")]
    pub gas_price: ::prost::alloc::string::String,
    /// gas defines the gas limit defined for the transaction.
    #[prost(uint64, tag = "4")]
    pub gas: u64,
    /// to is the recipient address in hex format
    #[prost(string, tag = "5")]
    pub to: ::prost::alloc::string::String,
    /// value defines the unsigned integer value of the transaction amount.
    #[prost(string, tag = "6")]
    pub value: ::prost::alloc::string::String,
    /// data is the data payload bytes of the transaction.
    #[prost(bytes = "vec", tag = "7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// accesses is an array of access tuples
    #[prost(message, repeated, tag = "8")]
    pub accesses: ::prost::alloc::vec::Vec<AccessTuple>,
    /// v defines the signature value
    #[prost(bytes = "vec", tag = "9")]
    pub v: ::prost::alloc::vec::Vec<u8>,
    /// r defines the signature value
    #[prost(bytes = "vec", tag = "10")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    /// s define the signature value
    #[prost(bytes = "vec", tag = "11")]
    pub s: ::prost::alloc::vec::Vec<u8>,
}
/// DynamicFeeTx is the data of EIP-1559 dinamic fee transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicFeeTx {
    /// chain_id of the destination EVM chain
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// nonce corresponds to the account nonce (transaction sequence).
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
    /// gas_tip_cap defines the max value for the gas tip
    #[prost(string, tag = "3")]
    pub gas_tip_cap: ::prost::alloc::string::String,
    /// gas_fee_cap defines the max value for the gas fee
    #[prost(string, tag = "4")]
    pub gas_fee_cap: ::prost::alloc::string::String,
    /// gas defines the gas limit defined for the transaction.
    #[prost(uint64, tag = "5")]
    pub gas: u64,
    /// to is the hex formatted address of the recipient
    #[prost(string, tag = "6")]
    pub to: ::prost::alloc::string::String,
    /// value defines the the transaction amount.
    #[prost(string, tag = "7")]
    pub value: ::prost::alloc::string::String,
    /// data is the data payload bytes of the transaction.
    #[prost(bytes = "vec", tag = "8")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// accesses is an array of access tuples
    #[prost(message, repeated, tag = "9")]
    pub accesses: ::prost::alloc::vec::Vec<AccessTuple>,
    /// v defines the signature value
    #[prost(bytes = "vec", tag = "10")]
    pub v: ::prost::alloc::vec::Vec<u8>,
    /// r defines the signature value
    #[prost(bytes = "vec", tag = "11")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    /// s define the signature value
    #[prost(bytes = "vec", tag = "12")]
    pub s: ::prost::alloc::vec::Vec<u8>,
}
/// ExtensionOptionsEthereumTx is an extension option for ethereum transactions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionOptionsEthereumTx {}
/// MsgEthereumTxResponse defines the Msg/EthereumTx response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEthereumTxResponse {
    /// hash of the ethereum transaction in hex format. This hash differs from the
    /// Tendermint sha256 hash of the transaction bytes. See
    /// <https://github.com/tendermint/tendermint/issues/6539> for reference
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// logs contains the transaction hash and the proto-compatible ethereum
    /// logs.
    #[prost(message, repeated, tag = "2")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    /// ret is the returned data from evm function (result or data supplied with revert
    /// opcode)
    #[prost(bytes = "vec", tag = "3")]
    pub ret: ::prost::alloc::vec::Vec<u8>,
    /// vm_error is the error returned by vm execution
    #[prost(string, tag = "4")]
    pub vm_error: ::prost::alloc::string::String,
    /// gas_used specifies how much gas was consumed by the transaction
    #[prost(uint64, tag = "5")]
    pub gas_used: u64,
    /// include the block hash for json-rpc to use
    #[prost(bytes = "vec", tag = "6")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
}
/// MsgUpdateParams defines a Msg for updating the x/evm module parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/evm parameters to update.
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
/// Generated client implementations.
#[cfg(feature = "client")]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the evm Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// EthereumTx defines a method submitting Ethereum transactions.
        pub async fn ethereum_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgEthereumTx>,
        ) -> std::result::Result<
            tonic::Response<super::MsgEthereumTxResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Msg/EthereumTx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Msg", "EthereumTx"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateParams defined a governance operation for updating the x/evm module parameters.
        /// The authority is hard-coded to the Cosmos SDK x/gov module account
        pub async fn update_params(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Msg/UpdateParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Msg", "UpdateParams"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "server")]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        /// EthereumTx defines a method submitting Ethereum transactions.
        async fn ethereum_tx(
            &self,
            request: tonic::Request<super::MsgEthereumTx>,
        ) -> std::result::Result<
            tonic::Response<super::MsgEthereumTxResponse>,
            tonic::Status,
        >;
        /// UpdateParams defined a governance operation for updating the x/evm module parameters.
        /// The authority is hard-coded to the Cosmos SDK x/gov module account
        async fn update_params(
            &self,
            request: tonic::Request<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
            tonic::Status,
        >;
    }
    /// Msg defines the evm Msg service.
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ethermint.evm.v1.Msg/EthereumTx" => {
                    #[allow(non_camel_case_types)]
                    struct EthereumTxSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgEthereumTx>
                    for EthereumTxSvc<T> {
                        type Response = super::MsgEthereumTxResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgEthereumTx>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::ethereum_tx(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EthereumTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Msg/UpdateParams" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateParamsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateParams>
                    for UpdateParamsSvc<T> {
                        type Response = super::MsgUpdateParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Msg>::update_params(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "ethermint.evm.v1.Msg";
    }
}
/// EventEthereumTx defines the event for an Ethereum transaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventEthereumTx {
    /// amount
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
    /// eth_hash is the Ethereum hash of the transaction
    #[prost(string, tag = "2")]
    pub eth_hash: ::prost::alloc::string::String,
    /// index of the transaction in the block
    #[prost(string, tag = "3")]
    pub index: ::prost::alloc::string::String,
    /// gas_used is the amount of gas used by the transaction
    #[prost(string, tag = "4")]
    pub gas_used: ::prost::alloc::string::String,
    /// hash is the Tendermint hash of the transaction
    #[prost(string, tag = "5")]
    pub hash: ::prost::alloc::string::String,
    /// recipient of the transaction
    #[prost(string, tag = "6")]
    pub recipient: ::prost::alloc::string::String,
    /// eth_tx_failed contains a VM error should it occur
    #[prost(string, tag = "7")]
    pub eth_tx_failed: ::prost::alloc::string::String,
}
/// EventTxLog defines the event for an Ethereum transaction log
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTxLog {
    /// tx_logs is an array of transaction logs
    #[prost(string, repeated, tag = "1")]
    pub tx_logs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventMessage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMessage {
    /// module which emits the event
    #[prost(string, tag = "1")]
    pub module: ::prost::alloc::string::String,
    /// sender of the message
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// tx_type is the type of the message
    #[prost(string, tag = "3")]
    pub tx_type: ::prost::alloc::string::String,
}
/// EventBlockBloom defines an Ethereum block bloom filter event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBlockBloom {
    /// bloom is the bloom filter of the block
    #[prost(string, tag = "1")]
    pub bloom: ::prost::alloc::string::String,
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountRequest {
    /// address is the ethereum hex address to query the account for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryAccountResponse is the response type for the Query/Account RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountResponse {
    /// balance is the balance of the EVM denomination.
    #[prost(string, tag = "1")]
    pub balance: ::prost::alloc::string::String,
    /// code_hash is the hex-formatted code bytes from the EOA.
    #[prost(string, tag = "2")]
    pub code_hash: ::prost::alloc::string::String,
    /// nonce is the account's sequence number.
    #[prost(uint64, tag = "3")]
    pub nonce: u64,
}
/// QueryCosmosAccountRequest is the request type for the Query/CosmosAccount RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCosmosAccountRequest {
    /// address is the ethereum hex address to query the account for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryCosmosAccountResponse is the response type for the Query/CosmosAccount
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCosmosAccountResponse {
    /// cosmos_address is the cosmos address of the account.
    #[prost(string, tag = "1")]
    pub cosmos_address: ::prost::alloc::string::String,
    /// sequence is the account's sequence number.
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    /// account_number is the account number
    #[prost(uint64, tag = "3")]
    pub account_number: u64,
}
/// QueryValidatorAccountRequest is the request type for the
/// Query/ValidatorAccount RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorAccountRequest {
    /// cons_address is the validator cons address to query the account for.
    #[prost(string, tag = "1")]
    pub cons_address: ::prost::alloc::string::String,
}
/// QueryValidatorAccountResponse is the response type for the
/// Query/ValidatorAccount RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorAccountResponse {
    /// account_address is the cosmos address of the account in bech32 format.
    #[prost(string, tag = "1")]
    pub account_address: ::prost::alloc::string::String,
    /// sequence is the account's sequence number.
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    /// account_number is the account number
    #[prost(uint64, tag = "3")]
    pub account_number: u64,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceRequest {
    /// address is the ethereum hex address to query the balance for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    /// balance is the balance of the EVM denomination.
    #[prost(string, tag = "1")]
    pub balance: ::prost::alloc::string::String,
}
/// QueryStorageRequest is the request type for the Query/Storage RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorageRequest {
    /// address is the ethereum hex address to query the storage state for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// key defines the key of the storage state
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// QueryStorageResponse is the response type for the Query/Storage RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorageResponse {
    /// value defines the storage state value hash associated with the given key.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// QueryCodeRequest is the request type for the Query/Code RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeRequest {
    /// address is the ethereum hex address to query the code for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryCodeResponse is the response type for the Query/Code RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCodeResponse {
    /// code represents the code bytes from an ethereum address.
    #[prost(bytes = "vec", tag = "1")]
    pub code: ::prost::alloc::vec::Vec<u8>,
}
/// QueryTxLogsRequest is the request type for the Query/TxLogs RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTxLogsRequest {
    /// hash is the ethereum transaction hex hash to query the logs for.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryTxLogsResponse is the response type for the Query/TxLogs RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTxLogsResponse {
    /// logs represents the ethereum logs generated from the given transaction.
    #[prost(message, repeated, tag = "1")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryParamsRequest defines the request type for querying x/evm parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse defines the response type for querying x/evm parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params define the evm module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// EthCallRequest defines EthCall request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthCallRequest {
    /// args uses the same json format as the json rpc api.
    #[prost(bytes = "vec", tag = "1")]
    pub args: ::prost::alloc::vec::Vec<u8>,
    /// gas_cap defines the default gas cap to be used
    #[prost(uint64, tag = "2")]
    pub gas_cap: u64,
    /// proposer_address of the requested block in hex format
    #[prost(bytes = "vec", tag = "3")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    /// chain_id is the eip155 chain id parsed from the requested block header
    #[prost(int64, tag = "4")]
    pub chain_id: i64,
    /// state overrides encoded as json
    #[prost(bytes = "vec", tag = "5")]
    pub overrides: ::prost::alloc::vec::Vec<u8>,
}
/// EstimateGasResponse defines EstimateGas response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EstimateGasResponse {
    /// gas returns the estimated gas
    #[prost(uint64, tag = "1")]
    pub gas: u64,
    /// ret is the returned data from evm function (result or data supplied with revert
    /// opcode)
    #[prost(bytes = "vec", tag = "2")]
    pub ret: ::prost::alloc::vec::Vec<u8>,
    /// vm_error is the error returned by vm execution
    #[prost(string, tag = "3")]
    pub vm_error: ::prost::alloc::string::String,
}
/// QueryTraceTxRequest defines TraceTx request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceTxRequest {
    /// msg is the MsgEthereumTx for the requested transaction
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<MsgEthereumTx>,
    /// trace_config holds extra parameters to trace functions.
    #[prost(message, optional, tag = "3")]
    pub trace_config: ::core::option::Option<TraceConfig>,
    /// predecessors is an array of transactions included in the same block
    /// need to be replayed first to get correct context for tracing.
    #[prost(message, repeated, tag = "4")]
    pub predecessors: ::prost::alloc::vec::Vec<MsgEthereumTx>,
    /// block_number of requested transaction
    #[prost(int64, tag = "5")]
    pub block_number: i64,
    /// block_hash of requested transaction
    #[prost(string, tag = "6")]
    pub block_hash: ::prost::alloc::string::String,
    /// block_time of requested transaction
    #[prost(message, optional, tag = "7")]
    pub block_time: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    /// proposer_address is the proposer of the requested block
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    /// chain_id is the the eip155 chain id parsed from the requested block header
    #[prost(int64, tag = "9")]
    pub chain_id: i64,
}
/// QueryTraceTxResponse defines TraceTx response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceTxResponse {
    /// data is the response serialized in bytes
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryTraceBlockRequest defines TraceTx request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceBlockRequest {
    /// txs is an array of messages in the block
    #[prost(message, repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<MsgEthereumTx>,
    /// trace_config holds extra parameters to trace functions.
    #[prost(message, optional, tag = "3")]
    pub trace_config: ::core::option::Option<TraceConfig>,
    /// block_number of the traced block
    #[prost(int64, tag = "5")]
    pub block_number: i64,
    /// block_hash (hex) of the traced block
    #[prost(string, tag = "6")]
    pub block_hash: ::prost::alloc::string::String,
    /// block_time of the traced block
    #[prost(message, optional, tag = "7")]
    pub block_time: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    /// proposer_address is the address of the requested block
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    /// chain_id is the eip155 chain id parsed from the requested block header
    #[prost(int64, tag = "9")]
    pub chain_id: i64,
}
/// QueryTraceBlockResponse defines TraceBlock response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTraceBlockResponse {
    /// data is the response serialized in bytes
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryBaseFeeRequest defines the request type for querying the EIP1559 base
/// fee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseFeeRequest {}
/// QueryBaseFeeResponse returns the EIP1559 base fee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseFeeResponse {
    /// base_fee is the EIP1559 base fee
    #[prost(string, tag = "1")]
    pub base_fee: ::prost::alloc::string::String,
}
/// Generated client implementations.
#[cfg(feature = "client")]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Account queries an Ethereum account.
        pub async fn account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/Account",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "Account"));
            self.inner.unary(req, path, codec).await
        }
        /// CosmosAccount queries an Ethereum account's Cosmos Address.
        pub async fn cosmos_account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCosmosAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCosmosAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/CosmosAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "CosmosAccount"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidatorAccount queries an Ethereum account's from a validator consensus
        /// Address.
        pub async fn validator_account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/ValidatorAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "ValidatorAccount"));
            self.inner.unary(req, path, codec).await
        }
        /// Balance queries the balance of a the EVM denomination for a single
        /// EthAccount.
        pub async fn balance(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBalanceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/Balance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "Balance"));
            self.inner.unary(req, path, codec).await
        }
        /// Storage queries the balance of all coins for a single account.
        pub async fn storage(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryStorageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryStorageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/Storage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "Storage"));
            self.inner.unary(req, path, codec).await
        }
        /// Code queries the balance of all coins for a single account.
        pub async fn code(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/Code",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "Code"));
            self.inner.unary(req, path, codec).await
        }
        /// Params queries the parameters of x/evm module.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/Params",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        /// EthCall implements the `eth_call` rpc api
        pub async fn eth_call(
            &mut self,
            request: impl tonic::IntoRequest<super::EthCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgEthereumTxResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/EthCall",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "EthCall"));
            self.inner.unary(req, path, codec).await
        }
        /// EstimateGas implements the `eth_estimateGas` rpc api
        pub async fn estimate_gas(
            &mut self,
            request: impl tonic::IntoRequest<super::EthCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateGasResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/EstimateGas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "EstimateGas"));
            self.inner.unary(req, path, codec).await
        }
        /// TraceTx implements the `debug_traceTransaction` rpc api
        pub async fn trace_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTraceTxRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraceTxResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/TraceTx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "TraceTx"));
            self.inner.unary(req, path, codec).await
        }
        /// TraceBlock implements the `debug_traceBlockByNumber` and `debug_traceBlockByHash` rpc api
        pub async fn trace_block(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTraceBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraceBlockResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/TraceBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "TraceBlock"));
            self.inner.unary(req, path, codec).await
        }
        /// BaseFee queries the base fee of the parent block of the current block,
        /// it's similar to feemarket module's method, but also checks london hardfork status.
        pub async fn base_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBaseFeeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBaseFeeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ethermint.evm.v1.Query/BaseFee",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ethermint.evm.v1.Query", "BaseFee"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "server")]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// Account queries an Ethereum account.
        async fn account(
            &self,
            request: tonic::Request<super::QueryAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAccountResponse>,
            tonic::Status,
        >;
        /// CosmosAccount queries an Ethereum account's Cosmos Address.
        async fn cosmos_account(
            &self,
            request: tonic::Request<super::QueryCosmosAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCosmosAccountResponse>,
            tonic::Status,
        >;
        /// ValidatorAccount queries an Ethereum account's from a validator consensus
        /// Address.
        async fn validator_account(
            &self,
            request: tonic::Request<super::QueryValidatorAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorAccountResponse>,
            tonic::Status,
        >;
        /// Balance queries the balance of a the EVM denomination for a single
        /// EthAccount.
        async fn balance(
            &self,
            request: tonic::Request<super::QueryBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBalanceResponse>,
            tonic::Status,
        >;
        /// Storage queries the balance of all coins for a single account.
        async fn storage(
            &self,
            request: tonic::Request<super::QueryStorageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryStorageResponse>,
            tonic::Status,
        >;
        /// Code queries the balance of all coins for a single account.
        async fn code(
            &self,
            request: tonic::Request<super::QueryCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCodeResponse>,
            tonic::Status,
        >;
        /// Params queries the parameters of x/evm module.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryParamsResponse>,
            tonic::Status,
        >;
        /// EthCall implements the `eth_call` rpc api
        async fn eth_call(
            &self,
            request: tonic::Request<super::EthCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MsgEthereumTxResponse>,
            tonic::Status,
        >;
        /// EstimateGas implements the `eth_estimateGas` rpc api
        async fn estimate_gas(
            &self,
            request: tonic::Request<super::EthCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateGasResponse>,
            tonic::Status,
        >;
        /// TraceTx implements the `debug_traceTransaction` rpc api
        async fn trace_tx(
            &self,
            request: tonic::Request<super::QueryTraceTxRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraceTxResponse>,
            tonic::Status,
        >;
        /// TraceBlock implements the `debug_traceBlockByNumber` and `debug_traceBlockByHash` rpc api
        async fn trace_block(
            &self,
            request: tonic::Request<super::QueryTraceBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryTraceBlockResponse>,
            tonic::Status,
        >;
        /// BaseFee queries the base fee of the parent block of the current block,
        /// it's similar to feemarket module's method, but also checks london hardfork status.
        async fn base_fee(
            &self,
            request: tonic::Request<super::QueryBaseFeeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBaseFeeResponse>,
            tonic::Status,
        >;
    }
    /// Query defines the gRPC querier service.
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ethermint.evm.v1.Query/Account" => {
                    #[allow(non_camel_case_types)]
                    struct AccountSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryAccountRequest>
                    for AccountSvc<T> {
                        type Response = super::QueryAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::account(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/CosmosAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CosmosAccountSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryCosmosAccountRequest>
                    for CosmosAccountSvc<T> {
                        type Response = super::QueryCosmosAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryCosmosAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::cosmos_account(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CosmosAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/ValidatorAccount" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorAccountSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryValidatorAccountRequest>
                    for ValidatorAccountSvc<T> {
                        type Response = super::QueryValidatorAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidatorAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::validator_account(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidatorAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/Balance" => {
                    #[allow(non_camel_case_types)]
                    struct BalanceSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryBalanceRequest>
                    for BalanceSvc<T> {
                        type Response = super::QueryBalanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryBalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::balance(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/Storage" => {
                    #[allow(non_camel_case_types)]
                    struct StorageSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryStorageRequest>
                    for StorageSvc<T> {
                        type Response = super::QueryStorageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryStorageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::storage(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StorageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/Code" => {
                    #[allow(non_camel_case_types)]
                    struct CodeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryCodeRequest>
                    for CodeSvc<T> {
                        type Response = super::QueryCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryCodeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::code(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest>
                    for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::params(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/EthCall" => {
                    #[allow(non_camel_case_types)]
                    struct EthCallSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::EthCallRequest>
                    for EthCallSvc<T> {
                        type Response = super::MsgEthereumTxResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EthCallRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::eth_call(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EthCallSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/EstimateGas" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateGasSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::EthCallRequest>
                    for EstimateGasSvc<T> {
                        type Response = super::EstimateGasResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EthCallRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::estimate_gas(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EstimateGasSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/TraceTx" => {
                    #[allow(non_camel_case_types)]
                    struct TraceTxSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTraceTxRequest>
                    for TraceTxSvc<T> {
                        type Response = super::QueryTraceTxResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTraceTxRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::trace_tx(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TraceTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/TraceBlock" => {
                    #[allow(non_camel_case_types)]
                    struct TraceBlockSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryTraceBlockRequest>
                    for TraceBlockSvc<T> {
                        type Response = super::QueryTraceBlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTraceBlockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::trace_block(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TraceBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ethermint.evm.v1.Query/BaseFee" => {
                    #[allow(non_camel_case_types)]
                    struct BaseFeeSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryBaseFeeRequest>
                    for BaseFeeSvc<T> {
                        type Response = super::QueryBaseFeeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryBaseFeeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::base_fee(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BaseFeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "ethermint.evm.v1.Query";
    }
}
/// GenesisState defines the evm module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// accounts is an array containing the ethereum genesis accounts.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<GenesisAccount>,
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// GenesisAccount defines an account to be initialized in the genesis state.
/// Its main difference between with Geth's GenesisAccount is that it uses a
/// custom storage type and that it doesn't contain the private key field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisAccount {
    /// address defines an ethereum hex formated address of an account
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// code defines the hex bytes of the account code.
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// storage defines the set of state key values for the account.
    #[prost(message, repeated, tag = "3")]
    pub storage: ::prost::alloc::vec::Vec<State>,
}
