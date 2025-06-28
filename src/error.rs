use std::fmt;
use std::path::PathBuf;

use rustls::server::VerifierBuilderError;

/// Error that could happen during connecting to LND
///
/// This error may be returned by the `connect()` function if connecting failed.
/// It is currently opaque because it's unclear how the variants will look long-term.
/// Thus you probably only want to display it.
#[derive(Debug)]
pub struct ConnectError {
    internal: InternalConnectError,
}

impl From<InternalConnectError> for ConnectError {
    fn from(value: InternalConnectError) -> Self {
        ConnectError { internal: value }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum InternalConnectError {
    ReadFile {
        file: PathBuf,
        error: std::io::Error,
    },
    ParseCert {
        file: Option<PathBuf>,
        error: std::io::Error,
    },
    InvalidAddress {
        address: String,
        error: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    Verifier(VerifierBuilderError),
    Endpoint(tonic::transport::Error),
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use InternalConnectError::*;

        match &self.internal {
            ReadFile { file, .. } => write!(f, "failed to read file {}", file.display()),
            ParseCert { file, .. } => match file {
                Some(file) => write!(f, "failed to parse certificate {}", file.display()),
                None => write!(f, "failed to parse certificate"),
            },
            InvalidAddress { address, .. } => write!(f, "invalid address {}", address),
            Verifier(error) => write!(f, "failed to build verifier: {}", error),
            Endpoint(error) => write!(f, "connection error: {}", error),
        }
    }
}

impl std::error::Error for ConnectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use InternalConnectError::*;

        match &self.internal {
            ReadFile { error, .. } => Some(error),
            ParseCert { error, .. } => Some(error),
            InvalidAddress { error, .. } => Some(&**error),
            Verifier(error) => Some(error),
            Endpoint(error) => Some(error),
        }
    }
}
