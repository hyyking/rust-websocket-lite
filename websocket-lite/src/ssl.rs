use std::fmt::Debug;
use std::io::{Read, Write};

use native_tls::{HandshakeError, TlsConnector};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{Error, Result};

pub async fn async_wrap<S: AsyncRead + AsyncWrite + Unpin>(
    domain: String,
    stream: S,
) -> Result<::tokio_tls::TlsStream<S>> {
    let builder = TlsConnector::builder();
    let cx = builder.build()?;
    Ok(tokio_tls::TlsConnector::from(cx).connect(&domain, stream).await?)
}

pub fn wrap<S: Read + Write + Debug + 'static>(domain: &str, stream: S) -> Result<::native_tls::TlsStream<S>> {
    let builder = TlsConnector::builder();
    let cx = builder.build()?;
    Ok(cx.connect(domain, stream).map_err(|e| {
        if let HandshakeError::Failure(e) = e {
            Error::from(e)
        } else {
            Error::from(e.to_string())
        }
    })?)
}
