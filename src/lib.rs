//! Async Rust client for the [LND gRPC API](https://github.com/lightningnetwork/lnd) using [`tonic`](https://docs.rs/tonic/) and [`prost`](https://docs.rs/prost/).
//!
//! # Overview
//!
//! This crate provides convenient, async access to the Lightning Network Daemon (LND) via gRPC, with vendored proto files for all major LND RPC APIs. It is designed for ergonomic integration with Rust async codebases, and supports feature flags for fine-grained control of enabled APIs and TLS implementations.
//!
//! ## Supported LND APIs (Features)
//!
//! Each LND RPC API is behind a Cargo feature flag. **All features are enabled by default** for a complete client, but you can select a subset for slimmer builds. See the `[features]` section in `Cargo.toml` for details.
//!
//! - `lightningrpc` (core Lightning API)
//! - `walletrpc` (WalletKit, depends on `signrpc`)
//! - `signrpc` (Signer)
//! - `peersrpc` (Peers)
//! - `routerrpc` (Router)
//! - `invoicesrpc` (Invoices)
//! - `staterpc` (State)
//! - `versionrpc` (Versioner)
//! - `all` (enables all RPCs)
//! - TLS backend selection: `ring` (default), `aws-lc`
//! - TLS root CA selection: `tls-native-roots`, `tls-webpki-roots`, `tls`
//!
//! **Default features:** `all`, `ring`, `tls`
//!
//! At least one TLS backend is required. The default is `ring`.
//!
//! ## Example
//!
//! Connect to LND using file paths for cert and macaroon:
//!
//! ```rust,no_run
//! use voltage_tonic_lnd::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), voltage_tonic_lnd::Error> {
//!     let client = Client::builder()
//!         .address("https://localhost:10009")
//!         .macaroon_path("/path/to/admin.macaroon")
//!         .cert_path("/path/to/tls.cert")
//!         .build()
//!         .await?;
//!     // Use client.lightning(), client.wallet(), etc.
//!     Ok(())
//! }
//! ```
//!
//! Or using in-memory credentials:
//!
//! ```rust,no_run
//! use voltage_tonic_lnd::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), voltage_tonic_lnd::Error> {
//!     let client = Client::builder()
//!         .address("https://localhost:10009")
//!         .macaroon_contents(HEX_MACAROON_STRING)
//!         .cert_contents(PEM_CERT_STRING)
//!         .build()
//!         .await?;
//!     Ok(())
//! }
//! ```
//!
//! See the crate README and `ClientBuilder` docs for more usage details.
//!
//! ### Example: Custom Timeout, No Cert
//!
//! You can set a timeout and skip the cert (using system roots or insecure connection, depending on your TLS features):
//!
//! ```rust,no_run
//! use voltage_tonic_lnd::Client;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), voltage_tonic_lnd::Error> {
//!     let client = Client::builder()
//!         .address("https://localhost:10009")
//!         .macaroon_path("/path/to/admin.macaroon")
//!         .timeout(Duration::from_secs(10))
//!         .build()
//!         .await?;
//!     Ok(())
//! }
//! ```

#![allow(clippy::large_enum_variant)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::doc_overindented_list_items)]

mod client;
mod error;
mod protos;

pub use client::*;
pub use error::*;
pub use protos::*;
pub use tonic;
