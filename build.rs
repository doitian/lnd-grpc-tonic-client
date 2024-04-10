use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Delete the files to regenerate
    let out_dir = Path::new("src");
    if !out_dir.join("lnrpc.rs").exists() {
        tonic_build::configure()
            .build_server(false)
            .out_dir("src")
            .compile(&["proto/lnrpc/lightning.proto"], &["proto/lnrpc"])?;
    }
    if !out_dir.join("routerrpc.rs").exists() {
        tonic_build::configure()
            .build_server(false)
            .out_dir("src")
            .compile(&["proto/lnrpc/routerrpc/router.proto"], &["proto/lnrpc"])?;
    }
    Ok(())
}
