use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

use tonic::service::interceptor::InterceptedService;
use tonic::transport::{Certificate, ClientTlsConfig, Endpoint, Uri};
use zeroize::Zeroizing;

use crate::error::{Error, Result};
use crate::protos::*;

type Service = InterceptedService<tonic::transport::Channel, MacaroonInterceptor>;

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

/// Convenience type alias for signer client.
#[cfg(feature = "signrpc")]
pub type SignerClient = signrpc::signer_client::SignerClient<Service>;

/// Convenience type alias for router client.
#[cfg(feature = "routerrpc")]
pub type RouterClient = routerrpc::router_client::RouterClient<Service>;

/// Convenience type alias for invoices client.
#[cfg(feature = "invoicesrpc")]
pub type InvoicesClient = invoicesrpc::invoices_client::InvoicesClient<Service>;

/// Convenience type alias for state service client.
#[cfg(feature = "staterpc")]
pub type StateClient = lnrpc::state_client::StateClient<Service>;

/// Convenience type alias for taproot assets client.
#[cfg(feature = "taprpc")]
pub type TaprootAssetsClient = taprpc::taproot_assets_client::TaprootAssetsClient<Service>;

/// Convenience type alias for asset wallet client.
#[cfg(feature = "assetwalletrpc")]
pub type AssetWalletClient = assetwalletrpc::asset_wallet_client::AssetWalletClient<Service>;

/// Convenience type alias for mint client.
#[cfg(feature = "mintrpc")]
pub type MintClient = mintrpc::mint_client::MintClient<Service>;

/// Convenience type alias for price oracle client.
#[cfg(feature = "priceoraclerpc")]
pub type PriceOracleClient = priceoraclerpc::price_oracle_client::PriceOracleClient<Service>;

/// Convenience type alias for RFQ client.
#[cfg(feature = "rfqrpc")]
pub type RfqClient = rfqrpc::rfq_client::RfqClient<Service>;

/// Convenience type alias for tap channel client.
#[cfg(feature = "tapchannelrpc")]
pub type TapChannelClient =
    tapchannelrpc::taproot_asset_channels_client::TaprootAssetChannelsClient<Service>;

/// Convenience type alias for tap dev client.
#[cfg(feature = "tapdevrpc")]
pub type TapDevClient = tapdevrpc::tap_dev_client::TapDevClient<Service>;

/// Convenience type alias for universal client.
#[cfg(feature = "universerpc")]
pub type UniversalClient = universerpc::universe_client::UniverseClient<Service>;

/// A builder for configuring and constructing a [`Client`] to connect to LND via gRPC.
///
/// This builder allows you to specify connection details, authentication credentials (macaroon),
/// and TLS certificates either from file paths or from in-memory contents. Use the various
/// methods to set the desired options, then call [`build`] to create a [`Client`].
///
/// # Example
/// ```rust
/// let client = ClientBuilder::new()
///     .address("https://localhost:10009")
///     .macaroon_path("~/.lnd/admin.macaroon")
///     .cert_path("~/.lnd/tls.cert")
///     .build()
///     .await?;
/// ```
///
/// You can also use in-memory credentials:
/// ```rust
/// let client = ClientBuilder::new()
///     .address("https://localhost:10009")
///     .macaroon_contents(hex_macaroon_string)
///     .cert_contents(pem_cert_string)
///     .build()
///     .await?;
/// ```
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    address: Option<String>,
    macaroon_path: Option<PathBuf>,
    macaroon_contents: Option<Zeroizing<String>>,
    cert_path: Option<PathBuf>,
    cert_contents: Option<String>,
    timeout: Option<Duration>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    /// Creates a new [`ClientBuilder`] with no fields set.
    pub fn new() -> Self {
        Self {
            address: None,
            macaroon_path: None,
            macaroon_contents: None,
            cert_path: None,
            cert_contents: None,
            timeout: None,
        }
    }

    /// Sets the address (URL) of the LND node to connect to.
    ///
    /// The address must begin with "https://".
    ///
    /// # Arguments
    /// * `address` - The gRPC endpoint of the LND node (e.g., "https://localhost:10009").
    pub fn address(mut self, address: impl ToString) -> Self {
        self.address = Some(address.to_string());
        self
    }

    /// Sets the path to the macaroon file for authentication.
    ///
    /// # Arguments
    /// * `path` - Filesystem path to the macaroon file (e.g., "~/.lnd/admin.macaroon").
    ///
    /// This is mutually exclusive with [`macaroon_contents`].
    pub fn macaroon_path(mut self, path: impl AsRef<Path> + Into<PathBuf>) -> Self {
        self.macaroon_path = Some(path.into());
        self
    }

    /// Sets the contents of the macaroon for authentication, as a hex-encoded string.
    ///
    /// # Arguments
    /// * `contents` - The macaroon as a hex-encoded string.
    ///
    /// This is mutually exclusive with [`macaroon_path`].
    pub fn macaroon_contents(mut self, contents: impl ToString) -> Self {
        self.macaroon_contents = Some(Zeroizing::new(contents.to_string()));
        self
    }

    /// Sets the path to the TLS certificate file for the LND node.
    ///
    /// # Arguments
    /// * `path` - Filesystem path to the PEM-encoded certificate file (e.g., "~/.lnd/tls.cert").
    ///
    /// This is mutually exclusive with [`cert_contents`].
    pub fn cert_path(mut self, path: impl AsRef<Path> + Into<PathBuf>) -> Self {
        self.cert_path = Some(path.into());
        self
    }

    /// Sets the contents of the TLS certificate for the LND node, as a PEM-encoded string.
    ///
    /// # Arguments
    /// * `contents` - The PEM-encoded certificate string.
    ///
    /// This is mutually exclusive with [`cert_path`].
    pub fn cert_contents(mut self, contents: impl ToString) -> Self {
        self.cert_contents = Some(contents.to_string());
        self
    }

    /// Sets the timeout for the all connections.
    ///
    /// # Arguments
    /// * `timeout` - The timeout duration.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Finalizes the builder and attempts to connect to the LND node, returning a [`Client`].
    ///
    /// # Errors
    /// Returns an error if any required field is missing (such as address or macaroon),
    /// or if the connection or credential loading fails.
    pub async fn build(self) -> Result<Client> {
        let address = self.address.ok_or(Error::MissingAddress)?;

        let macaroon = if let Some(path) = self.macaroon_path {
            load_macaroon(path).await?
        } else {
            self.macaroon_contents.ok_or(Error::MissingMacaroon)?
        };

        let cert = if let Some(path) = self.cert_path {
            Some(load_file(path).await?)
        } else {
            self.cert_contents.map(|contents| contents.as_bytes().to_vec())
        };

        do_connect(address, cert.map(Certificate::from_pem), macaroon, self.timeout).await
    }
}
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
    #[cfg(feature = "staterpc")]
    state: StateClient,
    #[cfg(feature = "taprpc")]
    taproot_assets: TaprootAssetsClient,
    #[cfg(feature = "assetwalletrpc")]
    asset_wallet: AssetWalletClient,
    #[cfg(feature = "mintrpc")]
    mint: MintClient,
    #[cfg(feature = "priceoraclerpc")]
    price_oracle: PriceOracleClient,
    #[cfg(feature = "rfqrpc")]
    rfq: RfqClient,
    #[cfg(feature = "tapchannelrpc")]
    tap_channel: TapChannelClient,
    #[cfg(feature = "tapdevrpc")]
    tap_dev: TapDevClient,
    #[cfg(feature = "universerpc")]
    universal: UniversalClient,
}

impl Client {
    /// Returns a builder for a client.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

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

    /// Returns the state service client.
    #[cfg(feature = "staterpc")]
    pub fn state(&mut self) -> &mut StateClient {
        &mut self.state
    }

    /// Returns a read-only state service client.
    #[cfg(feature = "staterpc")]
    pub fn state_read_only(self) -> StateClient {
        self.state
    }

    /// Returns the taproot assets client.
    #[cfg(feature = "taprpc")]
    pub fn taproot_assets(&mut self) -> &mut TaprootAssetsClient {
        &mut self.taproot_assets
    }

    /// Returns a read-only taproot assets client.
    #[cfg(feature = "taprpc")]
    pub fn taproot_assets_read_only(self) -> TaprootAssetsClient {
        self.taproot_assets
    }

    /// Returns the asset wallet client.
    #[cfg(feature = "assetwalletrpc")]
    pub fn asset_wallet(&mut self) -> &mut AssetWalletClient {
        &mut self.asset_wallet
    }

    /// Returns a read-only asset wallet client.
    #[cfg(feature = "assetwalletrpc")]
    pub fn asset_wallet_read_only(self) -> AssetWalletClient {
        self.asset_wallet
    }

    /// Returns the mint client.
    #[cfg(feature = "mintrpc")]
    pub fn mint(&mut self) -> &mut MintClient {
        &mut self.mint
    }

    /// Returns a read-only mint client.
    #[cfg(feature = "mintrpc")]
    pub fn mint_read_only(self) -> MintClient {
        self.mint
    }

    /// Returns the price oracle client.
    #[cfg(feature = "priceoraclerpc")]
    pub fn price_oracle(&mut self) -> &mut PriceOracleClient {
        &mut self.price_oracle
    }

    /// Returns a read-only price oracle client.
    #[cfg(feature = "priceoraclerpc")]
    pub fn price_oracle_read_only(self) -> PriceOracleClient {
        self.price_oracle
    }

    /// Returns the RFQ client.
    #[cfg(feature = "rfqrpc")]
    pub fn rfq(&mut self) -> &mut RfqClient {
        &mut self.rfq
    }

    /// Returns a read-only RFQ client.
    #[cfg(feature = "rfqrpc")]
    pub fn rfq_read_only(self) -> RfqClient {
        self.rfq
    }

    /// Returns the tap channel client.
    #[cfg(feature = "tapchannelrpc")]
    pub fn tap_channel(&mut self) -> &mut TapChannelClient {
        &mut self.tap_channel
    }

    /// Returns a read-only tap channel client.
    #[cfg(feature = "tapchannelrpc")]
    pub fn tap_channel_read_only(self) -> TapChannelClient {
        self.tap_channel
    }

    /// Returns the tap dev client.
    #[cfg(feature = "tapdevrpc")]
    pub fn tap_dev(&mut self) -> &mut TapDevClient {
        &mut self.tap_dev
    }

    /// Returns a read-only tap dev client.
    #[cfg(feature = "tapdevrpc")]
    pub fn tap_dev_read_only(self) -> TapDevClient {
        self.tap_dev
    }

    /// Returns the universal client.
    #[cfg(feature = "universerpc")]
    pub fn universal(&mut self) -> &mut UniversalClient {
        &mut self.universal
    }

    /// Returns a read-only universal client.
    #[cfg(feature = "universerpc")]
    pub fn universal_read_only(self) -> UniversalClient {
        self.universal
    }
}

/// Supplies requests with macaroon
#[derive(Clone)]
pub struct MacaroonInterceptor {
    macaroon: Zeroizing<String>,
}

impl tonic::service::Interceptor for MacaroonInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        request.metadata_mut().insert(
            "macaroon",
            tonic::metadata::MetadataValue::from_str(&self.macaroon)
                .expect("hex produced non-ascii"),
        );
        Ok(request)
    }
}

async fn load_file(path: impl AsRef<Path> + Into<PathBuf>) -> std::io::Result<Vec<u8>> {
    tokio::fs::read(&path).await
}

async fn load_macaroon(
    path: impl AsRef<Path> + Into<PathBuf>,
) -> std::io::Result<Zeroizing<String>> {
    let macaroon = load_file(path).await?;

    Ok(Zeroizing::new(hex::encode(macaroon)))
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
#[deprecated(since = "0.2.0", note = "Use `Client::builder` instead.")]
pub async fn connect<CP, MP>(address: String, cert_file: CP, macaroon_file: MP) -> Result<Client>
where
    CP: AsRef<Path> + Into<PathBuf> + std::fmt::Debug,
    MP: AsRef<Path> + Into<PathBuf> + std::fmt::Debug,
{
    Client::builder()
        .address(address)
        .cert_path(cert_file)
        .macaroon_path(macaroon_file)
        .build()
        .await
}

/// connect_from_memory connects to LND using in-memory cert and macaroon instead of from file paths.
/// `cert`` is a PEM encoded string
/// `macaroon`` is a hex-encoded string
/// These credentials can get out of date! Make sure you are pulling fresh credentials when using this function.
#[deprecated(since = "0.2.0", note = "Use `Client::builder` instead.")]
pub async fn connect_from_memory(
    address: impl ToString,
    cert_pem: impl ToString,
    macaroon: impl ToString,
) -> Result<Client> {
    Client::builder()
        .address(address)
        .cert_contents(cert_pem)
        .macaroon_contents(macaroon)
        .build()
        .await
}

/// connect_from_memory_with_system_certs connects to LND using in-memory macaroon and system certs.
/// `macaroon`` is a hex-encoded string
/// These credentials can get out of date! Make sure you are pulling fresh credentials when using this function.
#[deprecated(since = "0.2.0", note = "Use `Client::builder` instead.")]
pub async fn connect_from_memory_with_system_certs(
    address: impl ToString,
    macaroon: impl ToString,
) -> Result<Client> {
    Client::builder().address(address).macaroon_contents(macaroon).build().await
}

async fn do_connect(
    address: String,
    certs: Option<Certificate>,
    macaroon: Zeroizing<String>,
    timeout: Option<Duration>,
) -> Result<Client> {
    let mut tls_config = ClientTlsConfig::new().with_enabled_roots();
    if let Some(cert) = certs {
        tls_config = tls_config.ca_certificate(cert);
    }

    let mut endpoint = Endpoint::from_shared(address.clone())?.tls_config(tls_config)?;
    if let Some(timeout) = timeout {
        endpoint = endpoint.timeout(timeout);
    }

    let channel = endpoint.connect().await?;
    let channel = InterceptedService::new(
        channel,
        MacaroonInterceptor {
            macaroon,
        },
    );

    let uri = Uri::from_str(address.as_str())?;

    let client = Client {
        #[cfg(feature = "lightningrpc")]
        lightning: lnrpc::lightning_client::LightningClient::with_origin(
            channel.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "walletrpc")]
        wallet: walletrpc::wallet_kit_client::WalletKitClient::with_origin(
            channel.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "peersrpc")]
        peers: peersrpc::peers_client::PeersClient::with_origin(channel.clone(), uri.clone()),
        #[cfg(feature = "signrpc")]
        signer: signrpc::signer_client::SignerClient::with_origin(channel.clone(), uri.clone()),
        #[cfg(feature = "versionrpc")]
        version: verrpc::versioner_client::VersionerClient::with_origin(
            channel.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "routerrpc")]
        router: routerrpc::router_client::RouterClient::with_origin(channel.clone(), uri.clone()),
        #[cfg(feature = "invoicesrpc")]
        invoices: invoicesrpc::invoices_client::InvoicesClient::with_origin(
            channel.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "staterpc")]
        state: lnrpc::state_client::StateClient::with_origin(channel.clone(), uri.clone()),
        #[cfg(feature = "taprpc")]
        taproot_assets: taprpc::taproot_assets_client::TaprootAssetsClient::with_origin(
            channel.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "assetwalletrpc")]
        asset_wallet: assetwalletrpc::asset_wallet_client::AssetWalletClient::with_origin(
            channel.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "mintrpc")]
        mint: mintrpc::mint_client::MintClient::with_origin(channel.clone(), uri.clone()),
        #[cfg(feature = "priceoraclerpc")]
        price_oracle: priceoraclerpc::price_oracle_client::PriceOracleClient::with_origin(
            channel.clone(),
            uri.clone(),
        ),
        #[cfg(feature = "rfqrpc")]
        rfq: rfqrpc::rfq_client::RfqClient::with_origin(channel.clone(), uri.clone()),
        #[cfg(feature = "tapchannelrpc")]
        tap_channel:
            tapchannelrpc::taproot_asset_channels_client::TaprootAssetChannelsClient::with_origin(
                channel.clone(),
                uri.clone(),
            ),
        #[cfg(feature = "tapdevrpc")]
        tap_dev: tapdevrpc::tap_dev_client::TapDevClient::with_origin(channel.clone(), uri.clone()),
        #[cfg(feature = "universerpc")]
        universal: universerpc::universe_client::UniverseClient::with_origin(channel, uri),
    };

    Ok(client)
}
