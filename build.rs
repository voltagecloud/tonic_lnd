use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-env-changed=LND_REPO_DIR");
    let lnd_dir = match std::env::var_os("LND_REPO_DIR") {
        Some(lnd_repo_path) => PathBuf::from(lnd_repo_path).join("lnrpc"),
        None => PathBuf::from("vendor"),
    };

    let proto_file = lnd_dir.join("lightning.proto");
    println!("cargo:rerun-if-changed={}", proto_file.display());

    let protos = [
        "invoicesrpc/invoices.proto",
        "lightning.proto",
        "peersrpc/peers.proto",
        "routerrpc/router.proto",
        "signrpc/signer.proto",
        "stateservice.proto",
        "verrpc/verrpc.proto",
        "walletrpc/walletkit.proto",
    ];

    let lnd_proto_paths: Vec<_> = protos.iter().map(|proto| lnd_dir.join(proto)).collect();

    println!("cargo:rerun-if-env-changed=TAPROOT_ASSETS_REPO_DIR");
    let tap_dir = match std::env::var_os("TAPROOT_ASSETS_REPO_DIR") {
        Some(taproot_assets_repo_path) => PathBuf::from(taproot_assets_repo_path).join("taprpc"),
        None => PathBuf::from("vendor"),
    };

    let proto_file = tap_dir.join("taprootassets.proto");
    println!("cargo:rerun-if-changed={}", proto_file.display());

    let protos = [
        "assetwalletrpc/assetwallet.proto",
        "mintrpc/mint.proto",
        "priceoraclerpc/price_oracle.proto",
        "rfqrpc/rfq.proto",
        "tapchannelrpc/tapchannel.proto",
        "tapdevrpc/tapdev.proto",
        "taprootassets.proto",
        "universerpc/universe.proto",
    ];

    let tap_proto_paths: Vec<_> = protos.iter().map(|proto| tap_dir.join(proto)).collect();
    let all_protos: Vec<_> = lnd_proto_paths.into_iter().chain(tap_proto_paths).collect();
    let include_dirs = &[lnd_dir, tap_dir];

    #[cfg(feature = "serde")]
    {
        let descriptor_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
            .join("proto_descriptor.bin");

        tonic_prost_build::configure()
            .build_client(true)
            .build_server(false)
            .file_descriptor_set_path(&descriptor_path)
            .compile_protos(&all_protos, include_dirs)?;

        let descriptor_set = std::fs::read(&descriptor_path)?;
        pbjson_build::Builder::new().register_descriptors(&descriptor_set)?.build(&["."])?;
    }

    #[cfg(not(feature = "serde"))]
    {
        tonic_prost_build::configure()
            .build_client(true)
            .build_server(false)
            .compile_protos(&all_protos, include_dirs)?;
    }

    Ok(())
}
