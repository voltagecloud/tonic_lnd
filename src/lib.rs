#![allow(clippy::large_enum_variant)]
#![allow(clippy::doc_lazy_continuation)]
/// This is part of public interface so it's re-exported.
pub extern crate tonic;

pub use error::ConnectError;
use error::InternalConnectError;
use http_body_util::combinators::UnsyncBoxBody;
use hyper::Uri;
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use rustls::client::ClientConfig;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tonic::codegen::Bytes;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::{ClientTlsConfig, Endpoint};
use tonic::Status;

type Service = InterceptedService<
    hyper_util::client::legacy::Client<HttpsConnector<HttpConnector>, UnsyncBoxBody<Bytes, Status>>,
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

    /// Returns a read-only lightning client.
    #[cfg(feature = "lightningrpc")]
    pub fn lightning_read_only(self) -> LightningClient {
        self.lightning
    }

    /// Returns the wallet client.
    #[cfg(feature = "walletrpc")]
    pub fn wallet(&mut self) -> &mut WalletKitClient {
        &mut self.wallet
    }

    /// Returns a read-only wallet client.
    #[cfg(feature = "walletrpc")]
    pub fn wallet_read_only(self) -> WalletKitClient {
        self.wallet
    }

    /// Returns the signer client.
    #[cfg(feature = "signrpc")]
    pub fn signer(&mut self) -> &mut SignerClient {
        &mut self.signer
    }

    /// Returns a read-only signer client.
    #[cfg(feature = "signrpc")]
    pub fn signer_read_only(self) -> SignerClient {
        self.signer
    }

    /// Returns the versioner client.
    #[cfg(feature = "versionrpc")]
    pub fn versioner(&mut self) -> &mut VersionerClient {
        &mut self.version
    }

    /// Returns a read-only versioner client.
    #[cfg(feature = "versionrpc")]
    pub fn versioner_read_only(self) -> VersionerClient {
        self.version
    }

    /// Returns the peers client.
    #[cfg(feature = "peersrpc")]
    pub fn peers(&mut self) -> &mut PeersClient {
        &mut self.peers
    }

    /// Returns a read-only peers client.
    #[cfg(feature = "peersrpc")]
    pub fn peers_read_only(self) -> PeersClient {
        self.peers
    }

    /// Returns the router client.
    #[cfg(feature = "routerrpc")]
    pub fn router(&mut self) -> &mut RouterClient {
        &mut self.router
    }

    /// Returns a read-only router client.
    #[cfg(feature = "routerrpc")]
    pub fn router_read_only(self) -> RouterClient {
        self.router
    }

    /// Returns the invoices client.
    #[cfg(feature = "invoicesrpc")]
    pub fn invoices(&mut self) -> &mut InvoicesClient {
        &mut self.invoices
    }

    /// Returns a read-only invoices client.
    #[cfg(feature = "invoicesrpc")]
    pub fn invoices_read_only(self) -> InvoicesClient {
        self.invoices
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
    Ok(hex::encode(macaroon))
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
pub async fn connect<CP, MP>(
    address: String,
    cert_file: CP,
    macaroon_file: MP,
) -> Result<Client, ConnectError>
where
    CP: AsRef<Path> + Into<PathBuf> + std::fmt::Debug,
    MP: AsRef<Path> + Into<PathBuf> + std::fmt::Debug,
{
    let macaroon = load_macaroon(macaroon_file).await?;
    let tls_config = tls::config(tls::Cert::Path(cert_file)).await?;
    do_connect(address, tls_config, macaroon).await
}

/// connect_from_memory connects to LND using in-memory cert and macaroon instead of from file paths.
/// `cert`` is a PEM encoded string
/// `macaroon`` is a hex-encoded string
/// These credentials can get out of date! Make sure you are pulling fresh credentials when using this function.
pub async fn connect_from_memory(
    address: String,
    cert_pem: String,
    macaroon: String,
) -> Result<Client, ConnectError> {
    let tls_config = tls::config(tls::Cert::<String>::Bytes(cert_pem.into_bytes())).await?;
    do_connect(address, tls_config, macaroon).await
}

#[cfg(feature = "rustls-platform-verifier")]
pub async fn connect_from_memory_with_system_certs(
    address: impl ToString,
    macaroon: impl ToString,
) -> Result<Client, ConnectError> {
    let address = address.to_string();
    let macaroon = macaroon.to_string();
    let config = rustls_platform_verifier::tls_config();
    do_connect(address, config, macaroon).await
}

async fn do_connect(
    address: String,
    tls_config: ClientConfig,
    macaroon: String,
) -> Result<Client, ConnectError> {
    let mut endpoint = Endpoint::from_shared(address.clone())
        .map_err(InternalConnectError::Endpoint)?
        .tls_config(ClientTlsConfig::new().rustls_client_config(tls_config))?;

    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls_config)
        .https_or_http()
        .enable_http2()
        .build();
    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(connector);

    let svc = InterceptedService::new(client, MacaroonInterceptor { macaroon });
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
        invoices: invoicesrpc::invoices_client::InvoicesClient::with_origin(svc, uri),
    };

    Ok(client)
}

mod tls {
    use crate::error::{ConnectError, InternalConnectError};
    use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
    use rustls::crypto::{verify_tls12_signature, verify_tls13_signature};
    use rustls::{client::ClientConfig, DigitallySignedStruct, RootCertStore, SignatureScheme};
    use rustls_pki_types::{CertificateDer, ServerName, UnixTime};

    use std::{
        path::{Path, PathBuf},
        sync::Arc,
    };

    pub(crate) async fn config<P: AsRef<Path> + Into<PathBuf>>(
        cert: Cert<P>,
    ) -> Result<ClientConfig, ConnectError> {
        let hybrid_verifier = HybridCertVerifier::load(cert).await?;

        Ok(ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(hybrid_verifier))
            .with_no_client_auth())
    }

    #[derive(Debug)]
    pub(crate) struct HybridCertVerifier<'a> {
        exact_certs: Vec<CertificateDer<'a>>,
        standard_verifier: Arc<dyn ServerCertVerifier>,
    }

    impl<'a> HybridCertVerifier<'a> {
        pub(crate) async fn load<P: AsRef<Path> + Into<PathBuf>>(
            cert: Cert<P>,
        ) -> Result<Self, InternalConnectError> {
            let contents = match cert {
                Cert::Path(path) => {
                    try_map_err!(tokio::fs::read(&path).await, |error| {
                        InternalConnectError::ReadFile {
                            file: path.into(),
                            error,
                        }
                    })
                }
                Cert::Bytes(bytes) => bytes,
            };

            let mut reader = &*contents;
            let data = rustls_pemfile::certs(&mut reader).collect::<Result<Vec<_>, _>>();
            let cert_data = try_map_err!(data, |error| {
                InternalConnectError::ParseCert { file: None, error }
            });

            let mut root_store = RootCertStore::empty();
            for cert in &cert_data {
                if let Err(_err) = root_store.add(CertificateDer::from_slice(cert.as_ref())) {
                    return Err(InternalConnectError::ParseCert {
                        file: None,
                        error: std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Failed to add certificate to root store",
                        ),
                    });
                }
            }

            let standard_verifier =
                rustls::client::WebPkiServerVerifier::builder(Arc::new(root_store))
                    .build()
                    .map_err(InternalConnectError::Verifier)?;

            Ok(HybridCertVerifier {
                exact_certs: cert_data,
                standard_verifier,
            })
        }

        fn try_exact_match(
            &self,
            end_entity: &CertificateDer,
            intermediates: &[CertificateDer],
        ) -> bool {
            let mut presented_certs = intermediates.to_vec();
            presented_certs.push(end_entity.clone());

            // TODO: collapse logic below into single conditional
            if self.exact_certs.len() != presented_certs.len() {
                return false;
            }

            for (expected, presented) in self.exact_certs.iter().zip(presented_certs.iter()) {
                if presented != expected {
                    return false;
                }
            }

            true
        }
    }

    impl<'a> ServerCertVerifier for HybridCertVerifier<'a> {
        fn verify_server_cert(
            &self,
            end_entity: &CertificateDer<'_>,
            intermediates: &[CertificateDer<'_>],
            server_name: &ServerName<'_>,
            ocsp_response: &[u8],
            now: UnixTime,
        ) -> Result<ServerCertVerified, rustls::Error> {
            if self.try_exact_match(end_entity, intermediates) {
                return Ok(ServerCertVerified::assertion());
            }

            self.standard_verifier.verify_server_cert(
                end_entity,
                intermediates,
                server_name,
                ocsp_response,
                now,
            )
        }

        fn verify_tls12_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            verify_tls12_signature(
                message,
                cert,
                dss,
                &rustls::crypto::ring::default_provider().signature_verification_algorithms,
            )
        }

        fn verify_tls13_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            verify_tls13_signature(
                message,
                cert,
                dss,
                &rustls::crypto::ring::default_provider().signature_verification_algorithms,
            )
        }

        fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
            rustls::crypto::ring::default_provider()
                .signature_verification_algorithms
                .supported_schemes()
        }
    }

    pub(crate) enum Cert<P: AsRef<Path> + Into<PathBuf>> {
        Path(P),
        Bytes(Vec<u8>),
    }
}
