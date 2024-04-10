use std::error::Error;

use lnd_grpc_tonic_client::{create_router_client, routerrpc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Usage: cargo run --example get_mission_control_config -- <uri> <lnddir>
    let uri = std::env::args().nth(1).expect("uri required");
    let lnddir = std::env::args().nth(2).expect("lnddir required");

    let cert = tokio::fs::read(format!("{}/tls.cert", lnddir)).await?;
    let macaroon = tokio::fs::read(format!(
        "{}/data/chain/bitcoin/regtest/admin.macaroon",
        lnddir
    ))
    .await?;

    let mut router_client = create_router_client(uri.parse()?, Some(&cert), &macaroon).await?;
    println!(
        "response: {:?}",
        router_client
            .get_mission_control_config(routerrpc::GetMissionControlConfigRequest {})
            .await,
    );

    println!("hello");

    Ok(())
}
