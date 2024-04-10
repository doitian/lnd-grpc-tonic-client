//! Custom implementation to replace `tonic::transport::Channel`.
//!
//! This is an adaptation of lnd self-signed tls cert and macaroon authorization.
use hyper::{
    client::{HttpConnector, ResponseFuture},
    Body, Client, Request, Response, Uri,
};
use hyper_openssl::HttpsConnector;
use openssl::{
    ssl::{SslConnector, SslMethod},
    x509::X509,
};
use std::task::Poll;
use thiserror::Error;
use tonic::{
    body::BoxBody,
    metadata::{Ascii, MetadataValue},
    service::{interceptor::InterceptedService, Interceptor},
};
use tonic_openssl::ALPN_H2_WIRE;
use tower_service::Service;

pub type Channel = InterceptedService<TlsChannel, MacaroonInterceptor>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("OpenSSL error: {0}")]
    OpensslErrorStack(#[from] openssl::error::ErrorStack),
}

pub async fn connect_lnd(
    uri: Uri,
    cert: Option<&[u8]>,
    macaroon: &[u8],
) -> Result<InterceptedService<TlsChannel, MacaroonInterceptor>, Error> {
    let tls_channel = TlsChannel::new(uri, cert).await?;
    let macaroon = hex::encode(macaroon)
        .parse()
        .expect("hex must be valid ascii");
    Ok(InterceptedService::new(
        tls_channel,
        MacaroonInterceptor(macaroon),
    ))
}

#[derive(Clone, Debug)]
pub struct TlsChannel {
    uri: Uri,
    client: Client<HttpsConnector<HttpConnector>, BoxBody>,
}

impl TlsChannel {
    async fn new(uri: Uri, cert: Option<&[u8]>) -> Result<Self, Error> {
        let mut http = HttpConnector::new();
        http.enforce_http(false);
        let mut connector = SslConnector::builder(SslMethod::tls())?;
        if let Some(pem) = cert {
            let ca = X509::from_pem(pem)?;
            connector.cert_store_mut().add_cert(ca)?;
        }
        connector.set_alpn_protos(ALPN_H2_WIRE)?;
        let mut https = HttpsConnector::with_connector(http, connector)?;
        https.set_callback(|c, _| {
            c.set_verify_hostname(false);
            Ok(())
        });
        let client = Client::builder().http2_only(true).build(https);
        Ok(Self { uri, client })
    }
}

// Check out this blog post for an introduction to Tower:
// https://tokio.rs/blog/2021-05-14-inventing-the-service-trait
impl Service<Request<BoxBody>> for TlsChannel {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, mut req: Request<BoxBody>) -> Self::Future {
        let uri = Uri::builder()
            .scheme(self.uri.scheme().unwrap().clone())
            .authority(self.uri.authority().unwrap().clone())
            .path_and_query(req.uri().path_and_query().unwrap().clone())
            .build()
            .unwrap();
        *req.uri_mut() = uri;
        self.client.request(req)
    }
}

#[derive(Clone, Debug)]
pub struct MacaroonInterceptor(MetadataValue<Ascii>);

impl Interceptor for MacaroonInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        let metadata = request.metadata_mut();
        metadata.insert("macaroon", self.0.clone());
        Ok(request)
    }
}
