// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// The program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!
//
// Example run: `cargo run --features=lightningrpc --example get_info <address> [tls.cert] <file.macaroon>`
use fedimint_tonic_lnd::Client;

#[tokio::main]
#[cfg(feature = "lightningrpc")]
async fn main() {
    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");

    let address = args.next().expect("missing arguments: address, cert file, macaroon file");
    let mut macaroon_file = args.next().expect("missing arguments: cert file or macaroon file");

    let mut cert_file = None;
    if let Some(path) = args.next() {
        // if we have three arguments, then the cert file was passed.
        cert_file = Some(macaroon_file);
        macaroon_file = path;
    }

    let address = address.into_string().expect("address is not UTF-8");

    let mut client = Client::builder().address(address).macaroon_path(macaroon_file);
    if let Some(cert_file) = cert_file {
        client = client.cert_path(cert_file);
    }

    let mut client = client.build().await.expect("failed to build client");

    let info = client
        .lightning()
        // All calls require at least empty parameter
        .get_info(fedimint_tonic_lnd::lnrpc::GetInfoRequest {})
        .await
        .expect("failed to get info");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
