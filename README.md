# Tonic LND client

[![Crate](https://img.shields.io/crates/v/voltage-tonic-lnd.svg?logo=rust)](https://crates.io/crates/voltage-tonic-lnd)
[![Documentation](https://img.shields.io/static/v1?logo=read-the-docs&label=docs.rs&message=voltage-tonic-lnd&color=informational)](https://docs.rs/voltage-tonic-lnd/)

Rust implementation of LND RPC client using async gRPC library `tonic`.

## About

**Warning: this crate is in early development and may have unknown problems!
Review it before using with mainnet funds!**

This crate supports the following LND RPC APIs (from LND [v0.19.1-beta](https://github.com/lightningnetwork/lnd/tree/v0.19.1-beta)):
- [Lightning](https://lightning.engineering/api-docs/category/lightning-service)
- [WalletKit](https://lightning.engineering/api-docs/category/walletkit-service)
- [Signer](https://lightning.engineering/api-docs/category/signer-service)
- [Peer](https://lightning.engineering/api-docs/category/peers-service)
- [Router](https://lightning.engineering/api-docs/category/router-service)
- [Invoices](https://lightning.engineering/api-docs/category/invoices-service)
- [State](https://lightning.engineering/api-docs/category/state-service)
- [Versioner](https://lightning.engineering/api-docs/category/versioner-service)

This crate also supports [Taproot Assets](https://github.com/lightninglabs/taproot-assets) RPC APIs (from Taproot Assets [v0.6.1](https://github.com/lightninglabs/taproot-assets/tree/v0.6.1)):
- [TaprootAssets](https://github.com/lightninglabs/taproot-assets/blob/main/taprpc/taprootassets.proto) (core Taproot Assets API)
- [AssetWallet](https://github.com/lightninglabs/taproot-assets/blob/main/taprpc/assetwalletrpc/assetwallet.proto) (asset wallet management)
- [Mint](https://github.com/lightninglabs/taproot-assets/blob/main/taprpc/mintrpc/mint.proto) (asset minting)
- [PriceOracle](https://github.com/lightninglabs/taproot-assets/blob/main/taprpc/priceoraclerpc/price_oracle.proto) (price oracle service)
- [RFQ](https://github.com/lightninglabs/taproot-assets/blob/main/taprpc/rfqrpc/rfq.proto) (request for quote)
- [TapChannel](https://github.com/lightninglabs/taproot-assets/blob/main/taprpc/tapchannelrpc/tapchannel.proto) (Taproot Asset channels)
- [TapDev](https://github.com/lightninglabs/taproot-assets/blob/main/taprpc/tapdevrpc/tapdev.proto) (development tools)
- [Universe](https://github.com/lightninglabs/taproot-assets/blob/main/taprpc/universerpc/universe.proto) (universe server)

This crate implements LND gRPC using [`tonic`](https://docs.rs/tonic/) and [`prost`](https://docs.rs/prost/), providing async usage and vendored `*.proto` files (LND source not required by default). You can override the proto files at build time by setting the `LND_REPO_DIR` environment variable, to test against unreleased LND features.

## Features & Cargo Flags

Each RPC API is behind a Cargo feature flag. All features are enabled by default, but you can select a subset for slimmer builds. See the `[features]` section in `Cargo.toml` for details. Example features:

**LND Features:**
- `lightningrpc` (core Lightning API)
- `walletrpc` (WalletKit, depends on `signrpc`)
- `signrpc` (Signer)
- `peersrpc` (Peers)
- `routerrpc` (Router)
- `invoicesrpc` (Invoices)
- `staterpc` (State)
- `versionrpc` (Versioner)
- `lightning` (enables all LND RPCs)

**Taproot Assets Features:**
- `taprpc` (core Taproot Assets API)
- `assetwalletrpc` (asset wallet management)
- `mintrpc` (asset minting)
- `priceoraclerpc` (price oracle service)
- `rfqrpc` (request for quote)
- `tapchannelrpc` (Taproot Asset channels)
- `tapdevrpc` (development tools)
- `universerpc` (universe server)
- `taprootassets` (enables all Taproot Assets RPCs)

**Meta Features:**
- `all` (enables all LND and Taproot Assets RPCs)

**TLS Configuration:**
- TLS backend selection: `ring`, `aws-lc`
- TLS root CA selection: `tls-native-roots`, `tls-webpki-roots`, `tls`

At least one TLS backend is required. `ring` is currently used as the default.

See `Cargo.toml` for the full list and combinations.

## Usage

Since most of the LND RPCs supported by this crate can be used in isolation, and your project likely only needs a subset of these RPCs, we expose each RPC under [Cargo feature gates](https://doc.rust-lang.org/cargo/reference/features.html). See the Cargo manifest for the [latest supported features](https://github.com/Kixunil/tonic_lnd/blob/master/Cargo.toml)

All features are included by default, but you can explicitly select the features you want for a [slimmer dependency and faster compilations](https://github.com/Kixunil/tonic_lnd/pull/29#issuecomment-1352385426).

## Usage

Add the crate to your `Cargo.toml`:

```toml
voltage-tonic-lnd = "0.1"
```

By default, all features are enabled. To customize, specify features:

```toml
voltage-tonic-lnd = { version = "0.1", default-features = false, features = ["lightningrpc", "routerrpc", "aws-lc", "tls-native-roots"] }
```

To use Taproot Assets features:

```toml
voltage-tonic-lnd = { version = "0.1", default-features = false, features = ["lightningrpc", "taprootassets", "ring", "tls-native-roots"] }
```

If you need to override the proto files, set the `LND_REPO_DIR` environment variable to a directory with a cloned [`lnd`](https://github.com/lightningnetwork/lnd.git) repo during build. For Taproot Assets proto files, set the `TAPROOT_ASSETS_REPO_DIR` environment variable to a directory with a cloned [`taproot-assets`](https://github.com/lightninglabs/taproot-assets.git) repo.

### Example: Connect and Get Info

You can use the builder API for flexible connection:

```rust
#[tokio::main]
async fn main() -> voltage_tonic_lnd::Result<()> {
    let client = voltage_tonic_lnd::Client::builder()
        .address("https://localhost:10009")
        .macaroon_path("/path/to/admin.macaroon")
        .cert_path("/path/to/tls.cert")
        .build()
        .await?;

    let info = client.lightning().get_info(voltage_tonic_lnd::lnrpc::GetInfoRequest {}).await?;
    println!("{:#?}", info);
    Ok(())
}
```

See more [examples in the repo](https://github.com/voltagecloud/tonic-lnd/tree/master/examples) for advanced usage (router, invoices, payments, intercept HTLCs, etc).

### Alternative: In-Memory Credentials

```rust
let client = voltage_tonic_lnd::Client::builder()
    .address("https://localhost:10009")
    .macaroon_contents(hex_macaroon_string)
    .cert_contents(pem_cert_string)
    .build()
    .await?;
```

## Minimum Supported Rust Version (MSRV)

1.75.0

## License

MITNFA
