// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

// @generated
// Transaction data is transferred via 1 stream with batches until terminated.
// One stream consists:
//   StreamStatus: INIT with start_version x_0
//   loop:
//     (for kth iteration, start_version = x_k)
//     stream of TransactionOutput each with size(number of transactions)
//         t_0, t_1, ...
//     StreamStatus:
//       BATCH_END == x_k + sum(t_i) == x_{k + 1}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsOutput {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionOutput>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionOutput {
    /// Encoded aptos.proto.transaction.v1.Transaction.
    #[prost(string, tag = "1")]
    pub encoded_proto_data: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamStatus {
    #[prost(enumeration = "stream_status::StatusType", tag = "1")]
    pub r#type: i32,
    /// Required. Start version of current batch/stream, inclusive.
    #[prost(uint64, tag = "2")]
    pub start_version: u64,
    /// End version of current *batch*, exclusive.
    #[prost(uint64, optional, tag = "3")]
    pub end_version: ::core::option::Option<u64>,
}
/// Nested message and enum types in `StreamStatus`.
pub mod stream_status {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StatusType {
        /// Signal for the start of the stream.
        Init = 0,
        /// Signal for the end of the batch.
        /// Responses between BATCH_ENDs are NOT garanteed to be in order.
        /// Data in the response is garanteed to be in order.
        BatchEnd = 1,
    }
    impl StatusType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StatusType::Init => "INIT",
                StatusType::BatchEnd => "BATCH_END",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDatastreamRequest {
    /// Required; start version of current stream.
    #[prost(uint64, tag = "1")]
    pub starting_version: u64,
    /// Required; number of processors.
    #[prost(uint64, tag = "3")]
    pub processor_task_count: u64,
    /// Required; number of transactions per processor.
    #[prost(uint64, tag = "2")]
    pub processor_batch_size: u64,
    /// Required; number of transactions in the response.
    /// Each batch will be split into multiple responses with size upto output_batch_size.
    #[prost(uint64, tag = "4")]
    pub output_batch_size: u64,
    /// Required; for validation purpose.
    #[prost(uint32, tag = "5")]
    pub chain_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDatastreamResponse {
    #[prost(oneof = "raw_datastream_response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<raw_datastream_response::Response>,
}
/// Nested message and enum types in `RawDatastreamResponse`.
pub mod raw_datastream_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseType {
        /// Signal for streaming status.
        Status = 0,
        /// Data for transactions.
        Data = 1,
    }
    impl ResponseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResponseType::Status => "STATUS",
                ResponseType::Data => "DATA",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        Status(super::StreamStatus),
        #[prost(message, tag = "2")]
        Data(super::TransactionsOutput),
    }
}
/// Encoded file descriptor set for the `aptos.datastream.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xe5, 0x1a, 0x0a, 0x24, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x73,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x2f, 0x76, 0x31, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x61, 0x70, 0x74, 0x6f, 0x73,
    0x2e, 0x64, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x76, 0x31, 0x22, 0x60,
    0x0a, 0x12, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x4f, 0x75,
    0x74, 0x70, 0x75, 0x74, 0x12, 0x4a, 0x0a, 0x0c, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x61, 0x70, 0x74,
    0x6f, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x76, 0x31,
    0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4f, 0x75, 0x74, 0x70,
    0x75, 0x74, 0x52, 0x0c, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x22, 0x5b, 0x0a, 0x11, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4f,
    0x75, 0x74, 0x70, 0x75, 0x74, 0x12, 0x2c, 0x0a, 0x12, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x64,
    0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x10, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x64, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x44,
    0x61, 0x74, 0x61, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0xd2, 0x01,
    0x0a, 0x0c, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x40,
    0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2c, 0x2e, 0x61,
    0x70, 0x74, 0x6f, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e,
    0x76, 0x31, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x12, 0x23, 0x0a, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73, 0x74, 0x61, 0x72, 0x74, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x24, 0x0a, 0x0b, 0x65, 0x6e, 0x64, 0x5f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x0a, 0x65, 0x6e,
    0x64, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x88, 0x01, 0x01, 0x22, 0x25, 0x0a, 0x0a, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x54, 0x79, 0x70, 0x65, 0x12, 0x08, 0x0a, 0x04, 0x49, 0x4e, 0x49,
    0x54, 0x10, 0x00, 0x12, 0x0d, 0x0a, 0x09, 0x42, 0x41, 0x54, 0x43, 0x48, 0x5f, 0x45, 0x4e, 0x44,
    0x10, 0x01, 0x42, 0x0e, 0x0a, 0x0c, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x22, 0xec, 0x01, 0x0a, 0x14, 0x52, 0x61, 0x77, 0x44, 0x61, 0x74, 0x61, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x29, 0x0a, 0x10, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x30, 0x0a, 0x14, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73,
    0x73, 0x6f, 0x72, 0x5f, 0x74, 0x61, 0x73, 0x6b, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x12, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x54,
    0x61, 0x73, 0x6b, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x30, 0x0a, 0x14, 0x70, 0x72, 0x6f, 0x63,
    0x65, 0x73, 0x73, 0x6f, 0x72, 0x5f, 0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x73, 0x69, 0x7a, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x12, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f,
    0x72, 0x42, 0x61, 0x74, 0x63, 0x68, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x2a, 0x0a, 0x11, 0x6f, 0x75,
    0x74, 0x70, 0x75, 0x74, 0x5f, 0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x42, 0x61, 0x74,
    0x63, 0x68, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f,
    0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x49,
    0x64, 0x22, 0xc6, 0x01, 0x0a, 0x15, 0x52, 0x61, 0x77, 0x44, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3b, 0x0a, 0x06, 0x73,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x61, 0x70,
    0x74, 0x6f, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x76,
    0x31, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x48, 0x00,
    0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x3d, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x64,
    0x61, 0x74, 0x61, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x48,
    0x00, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0x25, 0x0a, 0x0d, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x53, 0x54, 0x41, 0x54,
    0x55, 0x53, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x44, 0x41, 0x54, 0x41, 0x10, 0x01, 0x42, 0x0a,
    0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x32, 0x79, 0x0a, 0x0d, 0x49, 0x6e,
    0x64, 0x65, 0x78, 0x65, 0x72, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x12, 0x68, 0x0a, 0x0d, 0x52,
    0x61, 0x77, 0x44, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x12, 0x29, 0x2e, 0x61,
    0x70, 0x74, 0x6f, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e,
    0x76, 0x31, 0x2e, 0x52, 0x61, 0x77, 0x44, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2a, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e,
    0x64, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x61,
    0x77, 0x44, 0x61, 0x74, 0x61, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x30, 0x01, 0x4a, 0xd8, 0x12, 0x0a, 0x06, 0x12, 0x04, 0x03, 0x00, 0x49, 0x01,
    0x0a, 0x44, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x03, 0x00, 0x12, 0x32, 0x3a, 0x20, 0x43, 0x6f, 0x70,
    0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x28, 0x63, 0x29, 0x20, 0x41, 0x70, 0x74, 0x6f, 0x73,
    0x0a, 0x20, 0x53, 0x50, 0x44, 0x58, 0x2d, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2d, 0x49,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x3a, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68,
    0x65, 0x2d, 0x32, 0x2e, 0x30, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x05, 0x00, 0x1c,
    0x0a, 0xef, 0x02, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x11, 0x00, 0x13, 0x01, 0x32, 0xe2, 0x02,
    0x20, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x61, 0x74,
    0x61, 0x20, 0x69, 0x73, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x72, 0x65, 0x64,
    0x20, 0x76, 0x69, 0x61, 0x20, 0x31, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c,
    0x20, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x4f, 0x6e,
    0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74,
    0x73, 0x3a, 0x0a, 0x20, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x3a, 0x20, 0x49, 0x4e, 0x49, 0x54, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x78, 0x5f, 0x30, 0x0a, 0x20,
    0x20, 0x6c, 0x6f, 0x6f, 0x70, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x28, 0x66, 0x6f, 0x72, 0x20,
    0x6b, 0x74, 0x68, 0x20, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x78,
    0x5f, 0x6b, 0x29, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x6f,
    0x66, 0x20, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4f, 0x75, 0x74,
    0x70, 0x75, 0x74, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x73, 0x69,
    0x7a, 0x65, 0x28, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x29, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x74, 0x5f, 0x30, 0x2c, 0x20, 0x74, 0x5f, 0x31, 0x2c, 0x20, 0x2e, 0x2e, 0x2e,
    0x0a, 0x20, 0x20, 0x20, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x42, 0x41, 0x54, 0x43, 0x48, 0x5f, 0x45,
    0x4e, 0x44, 0x20, 0x3d, 0x3d, 0x20, 0x78, 0x5f, 0x6b, 0x20, 0x2b, 0x20, 0x73, 0x75, 0x6d, 0x28,
    0x74, 0x5f, 0x69, 0x29, 0x20, 0x3d, 0x3d, 0x20, 0x78, 0x5f, 0x7b, 0x6b, 0x20, 0x2b, 0x20, 0x31,
    0x7d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x11, 0x08, 0x1a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x12, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x12, 0x1d, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x12, 0x2d, 0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x15, 0x00, 0x19, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x15, 0x08, 0x19, 0x0a, 0x3e, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x17, 0x02, 0x20, 0x1a, 0x31, 0x20, 0x45, 0x6e, 0x63, 0x6f, 0x64,
    0x65, 0x64, 0x20, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72,
    0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x17, 0x09, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x17, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x18,
    0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x09, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x18, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x1b, 0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x1b, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1c, 0x02,
    0x23, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x07, 0x11,
    0x0a, 0x34, 0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x04, 0x0d, 0x1a,
    0x25, 0x20, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1e, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x1e, 0x0b, 0x0c, 0x0a, 0xa5, 0x01, 0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x22, 0x04, 0x12, 0x1a, 0x95, 0x01, 0x20, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x0a, 0x20, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x73, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x42, 0x41,
    0x54, 0x43, 0x48, 0x5f, 0x45, 0x4e, 0x44, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x4e, 0x4f, 0x54,
    0x20, 0x67, 0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65,
    0x20, 0x69, 0x6e, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x2e, 0x0a, 0x20, 0x44, 0x61, 0x74, 0x61,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x69, 0x73, 0x20, 0x67, 0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x04, 0x0d, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x22, 0x10, 0x11, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x24, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x24, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x24, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x24, 0x14, 0x15, 0x0a, 0x4a, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26,
    0x02, 0x1b, 0x1a, 0x3d, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x53,
    0x74, 0x61, 0x72, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x2f, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x2c, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x73, 0x69, 0x76, 0x65, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x26, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x26, 0x09, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x26, 0x19, 0x1a, 0x0a, 0x39, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x28, 0x02, 0x22, 0x1a, 0x2c, 0x20, 0x45, 0x6e, 0x64, 0x20, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x20, 0x2a, 0x62, 0x61, 0x74, 0x63, 0x68, 0x2a, 0x2c, 0x20, 0x65, 0x78, 0x63, 0x6c, 0x75,
    0x73, 0x69, 0x76, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x28,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x28, 0x12, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x28, 0x20, 0x21, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2b, 0x00, 0x37, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x2b, 0x08, 0x1c, 0x0a, 0x39, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03,
    0x2d, 0x02, 0x1e, 0x1a, 0x2c, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x3b, 0x20,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66,
    0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x09, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x1c, 0x1d, 0x0a, 0x2e, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x02, 0x22, 0x1a, 0x21, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69,
    0x72, 0x65, 0x64, 0x3b, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x70,
    0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x2f, 0x09, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x2f, 0x20, 0x21, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x31,
    0x02, 0x22, 0x1a, 0x31, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x3b, 0x20, 0x6e,
    0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x70, 0x65, 0x72, 0x20, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73,
    0x73, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x31, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x31, 0x09,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31, 0x20, 0x21, 0x0a,
    0x95, 0x01, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x34, 0x02, 0x1f, 0x1a, 0x87, 0x01,
    0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x3b, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65,
    0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x2e, 0x0a, 0x20, 0x45, 0x61, 0x63, 0x68, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x69, 0x6e, 0x74,
    0x6f, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x75,
    0x70, 0x74, 0x6f, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x5f, 0x62, 0x61, 0x74, 0x63, 0x68,
    0x5f, 0x73, 0x69, 0x7a, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x34, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x34, 0x09, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x34, 0x1d,
    0x1e, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x36, 0x02, 0x16, 0x1a, 0x23,
    0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x3b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x75, 0x72, 0x70, 0x6f, 0x73,
    0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x36, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x36, 0x09, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x36, 0x14, 0x15, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x3a, 0x00, 0x45, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x3a, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x04, 0x00, 0x12, 0x04, 0x3b,
    0x02, 0x40, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x07,
    0x14, 0x0a, 0x2d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x04, 0x0f,
    0x1a, 0x1e, 0x20, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x04, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x3d, 0x0d, 0x0e,
    0x0a, 0x27, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x04, 0x0d, 0x1a,
    0x18, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x3f, 0x0b, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x08,
    0x00, 0x12, 0x04, 0x41, 0x02, 0x44, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x08, 0x00, 0x01,
    0x12, 0x03, 0x41, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x42,
    0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x42, 0x04, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x42, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x43, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x43, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x43, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x43, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x47, 0x00, 0x49, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x47, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x48, 0x04, 0x53, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x48, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x48, 0x16, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x48,
    0x35, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x48, 0x3c, 0x51,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("aptos.datastream.v1.serde.rs");
include!("aptos.datastream.v1.tonic.rs");
// @@protoc_insertion_point(module)
