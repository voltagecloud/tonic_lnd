// include_str! is not supported in attributes yet
#![doc = r###"
Rust implementation of LND RPC client using async GRPC library `tonic`.

## About

**Warning: this crate is in early development and may have unknown problems!
Review it before using with mainnet funds!**

This crate implements LND GRPC using [`tonic`](https://docs.rs/tonic/) and [`prost`](https://docs.rs/prost/).
Apart from being up-to-date at the time of writing (:D) it also allows `async` usage.
It contains vendored `rpc.proto` file so LND source code is not *required*
but accepts an environment variable `LND_REPO_DIR` which overrides the vendored `rpc.proto` file.
This can be used to test new features in non-released `lnd`.
(Actually, the motivating project using this library is that case. :))

## Usage

There's no setup needed beyond adding the crate to your `Cargo.toml`.
If you need to change the `rpc.proto` input set the environment variable `LND_REPO_DIR` to the directory with cloned `lnd` during build.

Here's an example of retrieving information from LND (`getinfo` call).
You can find the same example in crate root for your convenience.

```no_run
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

#[tokio::main]
async fn main() {
    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");
    let address = args.next().expect("missing arguments: address, cert file, macaroon file");
    let cert_file = args.next().expect("missing arguments: cert file, macaroon file");
    let macaroon_file = args.next().expect("missing argument: macaroon file");
    let address = address.into_string().expect("address is not UTF-8");

    // Connecting to LND requires only address, cert file, and macaroon file
    let mut client = tonic_lnd::connect(address, cert_file, macaroon_file)
        .await
        .expect("failed to connect");

    let info = client
        .lightning()
        // All calls require at least empty parameter
        .get_info(tonic_lnd::lnrpc::GetInfoRequest {})
        .await
        .expect("failed to get info");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
```

## MSRV

1.65.0

## License

MITNFA
"###]

/// This is part of public interface so it's re-exported.
pub extern crate tonic;

pub use error::ConnectError;
use error::InternalConnectError;
use http_body::combinators::UnsyncBoxBody;
use hyper::client::HttpConnector;
use hyper::Uri;
use hyper_rustls::HttpsConnector;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tonic::codegen::{Bytes, InterceptedService};
use tonic::Status;

type Service = InterceptedService<
    hyper::Client<HttpsConnector<HttpConnector>, UnsyncBoxBody<Bytes, Status>>,
    MacaroonInterceptor,
>;

/// Convenience type alias for lightning client.
#[cfg(feature = "lightningrpc")]
pub type LightningClient = lnrpc::lightning_client::LightningClient<Service>;

/// Convenience type alias for wallet client.
#[cfg(feature = "walletrpc")]
pub type WalletKitClient = walletrpc::wallet_kit_client::WalletKitClient<Service>;

/// Convenience type alias for peers service client.
#[cfg(feature = "peersrpc")]
pub type PeersClient = peersrpc::peers_client::PeersClient<Service>;

/// Convenience type alias for versioner service client.
#[cfg(feature = "versionrpc")]
pub type VersionerClient = verrpc::versioner_client::VersionerClient<Service>;

// Convenience type alias for signer client.
#[cfg(feature = "signrpc")]
pub type SignerClient = signrpc::signer_client::SignerClient<Service>;

/// Convenience type alias for router client.
#[cfg(feature = "routerrpc")]
pub type RouterClient = routerrpc::router_client::RouterClient<Service>;

/// Convenience type alias for invoices client.
#[cfg(feature = "invoicesrpc")]
pub type InvoicesClient = invoicesrpc::invoices_client::InvoicesClient<Service>;

/// The client returned by `connect` function
///
/// This is a convenience type which you most likely want to use instead of raw client.
#[derive(Clone)]
pub struct Client {
    #[cfg(feature = "lightningrpc")]
    lightning: LightningClient,
    #[cfg(feature = "walletrpc")]
    wallet: WalletKitClient,
    #[cfg(feature = "signrpc")]
    signer: SignerClient,
    #[cfg(feature = "peersrpc")]
    peers: PeersClient,
    #[cfg(feature = "versionrpc")]
    version: VersionerClient,
    #[cfg(feature = "routerrpc")]
    router: RouterClient,
    #[cfg(feature = "invoicesrpc")]
    invoices: InvoicesClient,
}

impl Client {
    /// Returns the lightning client.
    #[cfg(feature = "lightningrpc")]
    pub fn lightning(&mut self) -> &mut LightningClient {
        &mut self.lightning
    }

    /// Returns the wallet client.
    #[cfg(feature = "walletrpc")]
    pub fn wallet(&mut self) -> &mut WalletKitClient {
        &mut self.wallet
    }

    /// Returns the signer client.
    #[cfg(feature = "signrpc")]
    pub fn signer(&mut self) -> &mut SignerClient {
        &mut self.signer
    }

    /// Returns the versioner client.
    #[cfg(feature = "versionrpc")]
    pub fn versioner(&mut self) -> &mut VersionerClient {
        &mut self.version
    }

    /// Returns the peers client.
    #[cfg(feature = "peersrpc")]
    pub fn peers(&mut self) -> &mut PeersClient {
        &mut self.peers
    }

    /// Returns the router client.
    #[cfg(feature = "routerrpc")]
    pub fn router(&mut self) -> &mut RouterClient {
        &mut self.router
    }

    /// Returns the invoices client.
    #[cfg(feature = "invoicesrpc")]
    pub fn invoices(&mut self) -> &mut InvoicesClient {
        &mut self.invoices
    }
}

/// [`tonic::Status`] is re-exported as `Error` for convenience.
pub type Error = tonic::Status;

mod error;

macro_rules! try_map_err {
    ($result:expr, $mapfn:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => return Err($mapfn(error).into()),
        }
    };
}

/// Messages and other types generated by `tonic`/`prost`
///
/// This is the go-to module you will need to look in to find documentation on various message
/// types. However it may be better to start from methods on the [`LightningClient`](lnrpc::lightning_client::LightningClient) type.
#[cfg(feature = "lightningrpc")]
pub mod lnrpc {
    tonic::include_proto!("lnrpc");
}

#[cfg(feature = "walletrpc")]
pub mod walletrpc {
    tonic::include_proto!("walletrpc");
}

#[cfg(feature = "signrpc")]
pub mod signrpc {
    tonic::include_proto!("signrpc");
}

#[cfg(feature = "peersrpc")]
pub mod peersrpc {
    tonic::include_proto!("peersrpc");
}

#[cfg(feature = "routerrpc")]
pub mod routerrpc {
    tonic::include_proto!("routerrpc");
}

#[cfg(feature = "versionrpc")]
pub mod verrpc {
    tonic::include_proto!("verrpc");
}

#[cfg(feature = "invoicesrpc")]
pub mod invoicesrpc {
    tonic::include_proto!("invoicesrpc");
}

/// Supplies requests with macaroon
#[derive(Clone)]
pub struct MacaroonInterceptor {
    macaroon: String,
}

impl tonic::service::Interceptor for MacaroonInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Error> {
        request.metadata_mut().insert(
            "macaroon",
            tonic::metadata::MetadataValue::from_str(&self.macaroon)
                .expect("hex produced non-ascii"),
        );
        Ok(request)
    }
}

async fn load_macaroon(
    path: impl AsRef<Path> + Into<PathBuf>,
) -> Result<String, InternalConnectError> {
    let macaroon =
        tokio::fs::read(&path)
            .await
            .map_err(|error| InternalConnectError::ReadFile {
                file: path.into(),
                error,
            })?;
    Ok(hex::encode(&macaroon))
}

/// Connects to LND using given address and credentials
///
/// This function does all required processing of the cert file and macaroon file, so that you
/// don't have to. The address must begin with "https://", though.
///
/// This is considered the recommended way to connect to LND. An alternative function to use
/// already-read certificate or macaroon data is currently **not** provided to discourage such use.
/// LND occasionally changes that data which would lead to errors and in turn in worse application.
///
/// If you have a motivating use case for use of direct data feel free to open an issue and
/// explain.
#[cfg_attr(feature = "tracing", tracing::instrument(name = "Connecting to LND"))]
pub async fn connect<CP, MP>(
    address: String,
    cert_file: CP,
    macaroon_file: MP,
) -> Result<Client, ConnectError>
where
    CP: AsRef<Path> + Into<PathBuf> + std::fmt::Debug,
    MP: AsRef<Path> + Into<PathBuf> + std::fmt::Debug,
{
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls::config(cert_file).await?)
        .https_or_http()
        .enable_http2()
        .build();
    let macaroon = load_macaroon(macaroon_file).await?;

    let svc = InterceptedService::new(
        hyper::Client::builder().build(connector),
        MacaroonInterceptor { macaroon },
    );
    let uri =
        Uri::from_str(address.as_str()).map_err(|error| InternalConnectError::InvalidAddress {
            address,
            error: Box::new(error),
        })?;

    let client = Client {
        #[cfg(feature = "lightningrpc")]
        lightning: lnrpc::lightning_client::LightningClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "walletrpc")]
        wallet: walletrpc::wallet_kit_client::WalletKitClient::with_origin(
            svc.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "peersrpc")]
        peers: peersrpc::peers_client::PeersClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "signrpc")]
        signer: signrpc::signer_client::SignerClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "versionrpc")]
        version: verrpc::versioner_client::VersionerClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "routerrpc")]
        router: routerrpc::router_client::RouterClient::with_origin(svc.clone(), uri.clone()),
        #[cfg(feature = "invoicesrpc")]
        invoices: invoicesrpc::invoices_client::InvoicesClient::with_origin(
            svc.clone(),
            uri.clone(),
        ),
    };
    Ok(client)
}

mod tls {
    use crate::error::{ConnectError, InternalConnectError};
    use rustls::{
        client::{ClientConfig, ServerCertVerified, ServerCertVerifier},
        Certificate, Error as TLSError, ServerName,
    };
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
        time::SystemTime,
    };

    pub(crate) async fn config(
        path: impl AsRef<Path> + Into<PathBuf>,
    ) -> Result<ClientConfig, ConnectError> {
        Ok(ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::new(CertVerifier::load(path).await?))
            .with_no_client_auth())
    }

    pub(crate) struct CertVerifier {
        certs: Vec<Vec<u8>>,
    }

    impl CertVerifier {
        pub(crate) async fn load(
            path: impl AsRef<Path> + Into<PathBuf>,
        ) -> Result<Self, InternalConnectError> {
            let contents = try_map_err!(tokio::fs::read(&path).await, |error| {
                InternalConnectError::ReadFile {
                    file: path.into(),
                    error,
                }
            });
            let mut reader = &*contents;

            let certs = try_map_err!(rustls_pemfile::certs(&mut reader), |error| {
                InternalConnectError::ParseCert {
                    file: path.into(),
                    error,
                }
            });

            Ok(CertVerifier { certs: certs })
        }
    }

    impl ServerCertVerifier for CertVerifier {
        fn verify_server_cert(
            &self,
            end_entity: &Certificate,
            intermediates: &[Certificate],
            _server_name: &ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp_response: &[u8],
            _now: SystemTime,
        ) -> Result<ServerCertVerified, TLSError> {
            let mut certs = intermediates.iter().collect::<Vec<&Certificate>>();
            certs.push(end_entity);

            if self.certs.len() != certs.len() {
                return Err(TLSError::General(format!(
                    "Mismatched number of certificates (Expected: {}, Presented: {})",
                    self.certs.len(),
                    certs.len()
                )));
            }
            for (c, p) in self.certs.iter().zip(certs.iter()) {
                if *p.0 != **c {
                    return Err(TLSError::General(format!(
                        "Server certificates do not match ours"
                    )));
                }
            }
            Ok(ServerCertVerified::assertion())
        }
    }
}
