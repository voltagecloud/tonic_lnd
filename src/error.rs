pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Filesystem error: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("Tonic error: {0}")]
    Tonic(#[from] tonic::transport::Error),
    #[error("Invalid address: {0}")]
    InvalidAddress(#[from] http::uri::InvalidUri),
    #[error("Missing address")]
    MissingAddress,
    #[error("Missing macaroon")]
    MissingMacaroon,
}
