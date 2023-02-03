// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0
use aptos_protos::datastream::v1::indexer_stream_client::IndexerStreamClient;

pub mod worker;

pub type GrpcClientType = IndexerStreamClient<tonic::transport::Channel>;

/// Create a gRPC client with exponential backoff.
pub async fn create_grpc_client(address: String) -> GrpcClientType {
    backoff::future::retry(backoff::ExponentialBackoff::default(), || async {
        Ok(IndexerStreamClient::connect(address.clone()).await?)
    })
    .await
    .unwrap()
}
