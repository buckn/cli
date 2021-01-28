use hecs::*;
use quinn::{ClientConfig, ClientConfigBuilder, Endpoint};
use rustls::ServerCertVerified;
use std::sync::mpsc::Sender;
use std::{
    error::Error,
    fs,
    io::{self, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs},
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use webpki::*;

#[tokio::main]
pub async fn start(send: Sender<hecs::World>) {
    let client_cfg = configure_client();
    let mut endpoint_builder = Endpoint::builder();
    endpoint_builder.default_client_config(client_cfg);

    let (endpoint, _) = endpoint_builder
        .bind(&"127.0.0.1:0".parse().unwrap())
        .unwrap();

    // connect to server
    let quinn::NewConnection { connection, .. } = endpoint
        .connect(
            &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000),
            "localhost",
        )
        .unwrap()
        .await
        .unwrap();
    println!("[client] connected: addr={}", connection.remote_address());
    // Dropping handles allows the corresponding objects to automatically shut down
    drop(connection);
    // Make sure the server has a chance to clean up
    endpoint.wait_idle().await;
}

struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _roots: &rustls::RootCertStore,
        _presented_certs: &[rustls::Certificate],
        _dns_name: webpki::DNSNameRef,
        _ocsp_response: &[u8],
    ) -> Result<rustls::ServerCertVerified, rustls::TLSError> {
        Ok(rustls::ServerCertVerified::assertion())
    }
}

fn configure_client() -> ClientConfig {
    let mut cfg = ClientConfigBuilder::default().build();
    let tls_cfg: &mut rustls::ClientConfig = Arc::get_mut(&mut cfg.crypto).unwrap();
    // this is only available when compiled with "dangerous_configuration" feature
    tls_cfg
        .dangerous()
        .set_certificate_verifier(SkipServerVerification::new());
    cfg
}

fn duration_secs(x: &Duration) -> f32 {
    x.as_secs() as f32 + x.subsec_nanos() as f32 * 1e-9
}
