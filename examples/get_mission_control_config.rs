use std::error::Error;

use lnd_grpc_tonic_client::{create_router_client, routerrpc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Usage: cargo run --example get_mission_control_config -- <uri> <lnddir>
    let uri = std::env::args().nth(1).expect("uri required");
    let lnddir = std::env::args().nth(2).expect("lnddir required");

    let cert = tokio::fs::read(format!("{}/tls.cert", lnddir)).await?;
    let macaroon_path = format!("{}/data/chain/bitcoin/regtest/admin.macaroon", lnddir);
    let macaroon = if tokio::fs::try_exists(&macaroon_path).await? {
        Some(tokio::fs::read(&macaroon_path).await?)
    } else {
        None
    };

    let mut router_client =
        create_router_client(uri.parse()?, Some(&cert), macaroon.as_deref()).await?;
    println!(
        "response: {:?}",
        router_client
            .get_mission_control_config(routerrpc::GetMissionControlConfigRequest {})
            .await,
    );

    Ok(())
}
