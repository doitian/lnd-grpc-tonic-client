pub mod lnrpc;
pub mod routerrpc;

pub mod channel;
pub use channel::connect_lnd;

pub use hyper::Uri;

pub type LightningClient = lnrpc::lightning_client::LightningClient<channel::Channel>;
pub type RouterClient = routerrpc::router_client::RouterClient<channel::Channel>;

pub async fn create_lightning_client(
    uri: Uri,
    cert: Option<&[u8]>,
    macaroon: &[u8],
) -> Result<LightningClient, channel::Error> {
    connect_lnd(uri, cert, macaroon)
        .await
        .map(LightningClient::new)
}

pub async fn create_router_client(
    uri: Uri,
    cert: Option<&[u8]>,
    macaroon: &[u8],
) -> Result<RouterClient, channel::Error> {
    connect_lnd(uri, cert, macaroon)
        .await
        .map(RouterClient::new)
}
