use std::error::Error;

use lnd_grpc_tonic_client::{create_invoices_client, invoicesrpc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Usage: cargo run --example get_mission_control_config -- <uri> <lnddir>
    let uri = std::env::args().nth(1).expect("uri required");
    let lnddir = std::env::args().nth(2).expect("lnddir required");
    let payment_hash = hex::decode(std::env::args().nth(3).expect("payment hash required"))?;

    let cert = tokio::fs::read(format!("{}/tls.cert", lnddir)).await?;
    let macaroon = tokio::fs::read(format!(
        "{}/data/chain/bitcoin/regtest/admin.macaroon",
        lnddir
    ))
    .await?;

    let mut invoices_client =
        create_invoices_client(uri.parse()?, Some(&cert), Some(&macaroon)).await?;
    println!(
        "response: {:?}",
        invoices_client
            .add_hold_invoice(invoicesrpc::AddHoldInvoiceRequest {
                hash: payment_hash,
                value_msat: 500,
                ..Default::default()
            })
            .await,
    );

    println!("hello");

    Ok(())
}
