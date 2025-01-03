#![allow(unused_imports)]
#![allow(unused_variables)]
use std::net::{ Ipv4Addr, SocketAddr };

use bevy::asset::ron;
use bevy::prelude::{ Resource, info, default };
use bevy::utils::Duration;

use async_compat::Compat;
use bevy::tasks::IoTaskPool;

use lightyear::connection::netcode::PRIVATE_KEY_BYTES;
use lightyear::prelude::client::{ Authentication, ClientTransport };
use lightyear::prelude::{ CompressionConfig, LinkConditionerConfig };

use lightyear::prelude::{ client, server };

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClientTransports {
    Udp,
    WebTransport,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServerTransports {
    Udp {
        local_port: u16,
    },
    WebTransport {
        local_port: u16,
        certificate: WebTransportCertificateSettings,
    },
}

#[derive(Clone, Debug)]
pub struct Conditioner {
    /// One way latency in milliseconds
    pub latency_ms: u16,
    /// One way jitter in milliseconds
    pub jitter_ms: u16,
    /// Percentage of packet loss
    pub packet_loss: f32,
}

impl Conditioner {
    pub fn build(&self) -> LinkConditionerConfig {
        LinkConditionerConfig {
            incoming_latency: Duration::from_millis(self.latency_ms as u64),
            incoming_jitter: Duration::from_millis(self.jitter_ms as u64),
            incoming_loss: self.packet_loss,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerSettings {
    /// If true, disable any rendering-related plugins
    pub headless: bool,

    /// If true, enable bevy_inspector_egui
    pub inspector: bool,

    /// Possibly add a conditioner to simulate network conditions
    pub conditioner: Option<Conditioner>,

    /// Which transport to use
    pub transport: Vec<ServerTransports>,
}

#[derive(Clone, Debug)]
pub struct ClientSettings {
    /// If true, enable bevy_inspector_egui
    pub inspector: bool,

    /// The client id
    pub client_id: u64,

    /// The client port to listen on
    pub client_port: u16,

    /// The ip address of the server
    pub server_addr: Ipv4Addr,

    /// The port of the server
    pub server_port: u16,

    /// Which transport to use
    pub transport: ClientTransports,

    /// Possibly add a conditioner to simulate network conditions
    pub conditioner: Option<Conditioner>,
}

#[derive(Copy, Clone, Debug)]
pub struct SharedSettings {
    /// An id to identify the protocol version
    pub protocol_id: u64,

    /// a 32-byte array to authenticate via the Netcode.io protocol
    pub private_key: [u8; 32],

    /// compression options
    pub compression: CompressionConfig,
}

#[derive(Resource, Debug, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub client: ClientSettings,
    pub shared: SharedSettings,
}

#[cfg(feature = "server")]
pub(crate) fn build_server_netcode_config(
    conditioner: Option<&Conditioner>,
    shared: &SharedSettings,
    transport_config: server::ServerTransport
) -> server::NetConfig {
    let conditioner = conditioner.map(|c| LinkConditionerConfig {
        incoming_latency: Duration::from_millis(c.latency_ms as u64),
        incoming_jitter: Duration::from_millis(c.jitter_ms as u64),
        incoming_loss: c.packet_loss,
    });
    // Use private key from environment variable, if set. Otherwise from settings file.
    let privkey = if let Some(key) = parse_private_key_from_env() {
        info!("Using private key from LIGHTYEAR_PRIVATE_KEY env var");
        key
    } else {
        shared.private_key
    };

    let netcode_config = server::NetcodeConfig
        ::default()
        .with_protocol_id(shared.protocol_id)
        .with_key(privkey);
    let io_config = server::IoConfig {
        transport: transport_config,
        conditioner,
        compression: shared.compression,
    };
    server::NetConfig::Netcode {
        config: netcode_config,
        io: io_config,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WebTransportCertificateSettings {
    /// Generate a self-signed certificate, with given SANs list to add to the certifictate
    /// eg: ["example.com", "*.gameserver.example.org", "10.1.2.3", "::1"]
    AutoSelfSigned(Vec<String>),
    /// Load certificate pem files from disk
    FromFile {
        /// Path to cert .pem file
        cert: String,
        /// Path to private key .pem file
        key: String,
    },
}

impl Default for WebTransportCertificateSettings {
    fn default() -> Self {
        let sans = vec!["localhost".to_string(), "127.0.0.1".to_string(), "::1".to_string()];
        WebTransportCertificateSettings::AutoSelfSigned(sans)
    }
}

#[cfg(feature = "server")]
impl From<&WebTransportCertificateSettings> for server::Identity {
    fn from(wt: &WebTransportCertificateSettings) -> server::Identity {
        match wt {
            WebTransportCertificateSettings::AutoSelfSigned(sans) => {
                let mut sans = sans.clone();
                // generic env to add domains and ips to SAN list:
                // SELF_SIGNED_SANS="example.org,example.com,127.1.1.1"
                if let Ok(san) = std::env::var("SELF_SIGNED_SANS") {
                    println!("🔐 SAN += SELF_SIGNED_SANS: {}", san);
                    sans.extend(san.split(',').map(|s| s.to_string()));
                }
                println!("🔐 Generating self-signed certificate with SANs: {sans:?}");
                let identity = server::Identity::self_signed(sans).unwrap();
                let digest = identity.certificate_chain().as_slice()[0].hash();
                info!("🔐 Certificate digest: {digest}");
                identity
            }
            WebTransportCertificateSettings::FromFile {
                cert: cert_pem_path,
                key: private_key_pem_path,
            } => {
                info!(
                    "Reading certificate PEM files:\n * cert: {}\n * key: {}",
                    cert_pem_path,
                    private_key_pem_path
                );
                // this is async because we need to load the certificate from io
                // we need async_compat because wtransport expects a tokio reactor
                let identity = IoTaskPool::get()
                    .scope(|s| {
                        s.spawn(
                            Compat::new(async {
                                server::Identity
                                    ::load_pemfiles(cert_pem_path, private_key_pem_path).await
                                    .unwrap()
                            })
                        );
                    })
                    .pop()
                    .unwrap();
                let digest = identity.certificate_chain().as_slice()[0].hash();
                println!("🔐 Certificate digest: {digest}");
                identity
            }
        }
    }
}

/// Parse the settings into a list of `NetConfig` that are used to configure how the lightyear server
/// listens for incoming client connections
#[cfg(feature = "server")]
pub(crate) fn get_server_net_configs(settings: &Settings) -> Vec<server::NetConfig> {
    settings.server.transport
        .iter()
        .map(|t| {
            match t {
                ServerTransports::Udp { local_port } =>
                    build_server_netcode_config(
                        settings.server.conditioner.as_ref(),
                        &settings.shared,
                        server::ServerTransport::UdpSocket(
                            SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), *local_port)
                        )
                    ),
                ServerTransports::WebTransport { local_port, certificate } => {
                    let transport_config = server::ServerTransport::WebTransportServer {
                        server_addr: SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), *local_port),
                        certificate: certificate.into(),
                    };
                    build_server_netcode_config(
                        settings.server.conditioner.as_ref(),
                        &settings.shared,
                        transport_config
                    )
                }
            }
        })
        .collect()
}

/// Build a netcode config for the client
pub(crate) fn build_client_netcode_config(
    client_id: u64,
    server_addr: SocketAddr,
    conditioner: Option<&Conditioner>,
    shared: &SharedSettings,
    transport_config: client::ClientTransport
) -> client::NetConfig {
    let conditioner = conditioner.map(|c| c.build());
    // TODO no point having the private key in shared settings. client's shouldn't know it.
    // use dummy zeroed key explicitly here.
    let auth = Authentication::Manual {
        server_addr,
        client_id,
        private_key: shared.private_key,
        protocol_id: shared.protocol_id,
    };
    info!("Auth: {auth:?}");
    info!("TransportConfig: {transport_config:?}");
    let netcode_config = client::NetcodeConfig {
        // Make sure that the server times out clients when their connection is closed
        client_timeout_secs: 3,
        ..default()
    };
    let io_config = client::IoConfig {
        transport: transport_config,
        conditioner,
        compression: shared.compression,
    };
    client::NetConfig::Netcode {
        auth,
        config: netcode_config,
        io: io_config,
    }
}

/// Parse the settings into a `NetConfig` that is used to configure how the lightyear client
/// connects to the server
pub(crate) fn get_client_net_config(settings: &Settings, client_id: u64) -> client::NetConfig {
    let server_addr = SocketAddr::new(
        settings.client.server_addr.into(),
        settings.client.server_port
    );
    let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), settings.client.client_port);
    match &settings.client.transport {
        ClientTransports::Udp =>
            build_client_netcode_config(
                client_id,
                server_addr,
                settings.client.conditioner.as_ref(),
                &settings.shared,
                client::ClientTransport::UdpSocket(client_addr)
            ),
        ClientTransports::WebTransport =>
            build_client_netcode_config(
                client_id,
                server_addr,
                settings.client.conditioner.as_ref(),
                &settings.shared,
                client::ClientTransport::WebTransportClient {
                    client_addr,
                    server_addr,
                }
            ),
    }
}

/// Reads and parses the LIGHTYEAR_PRIVATE_KEY environment variable into a private key.
#[cfg(feature = "server")]
fn parse_private_key_from_env() -> Option<[u8; PRIVATE_KEY_BYTES]> {
    let Ok(key_str) = std::env::var("LIGHTYEAR_PRIVATE_KEY") else {
        return None;
    };
    let private_key: Vec<u8> = key_str
        .chars()
        .filter(|c| (c.is_ascii_digit() || *c == ','))
        .collect::<String>()
        .split(',')
        .map(|s| { s.parse::<u8>().expect("Failed to parse number in private key") })
        .collect();

    if private_key.len() != PRIVATE_KEY_BYTES {
        panic!("Private key must contain exactly {} numbers", PRIVATE_KEY_BYTES);
    }

    let mut bytes = [0u8; PRIVATE_KEY_BYTES];
    bytes.copy_from_slice(&private_key);
    Some(bytes)
}

pub(crate) fn get_settings() -> Settings {
    Settings {
        server: ServerSettings {
            headless: false,
            inspector: true,
            conditioner: Some(Conditioner {
                latency_ms: 200,
                jitter_ms: 20,
                packet_loss: 0.05,
            }),
            transport: vec![
                ServerTransports::WebTransport {
                    local_port: 5000,
                    certificate: WebTransportCertificateSettings::FromFile {
                        cert: "assets/certificates/cert.pem".to_string(),
                        key: "assets/certificates/key.pem".to_string(),
                    },
                },
                ServerTransports::Udp { local_port: 5001 }
            ],
        },
        client: ClientSettings {
            inspector: true,
            client_id: 0,
            client_port: 0, // 0 means that the OS will assign a random port
            server_addr: Ipv4Addr::LOCALHOST,
            server_port: 5000, // change the port depending on the transport used
            transport: ClientTransports::WebTransport,
            conditioner: None,
        },
        shared: SharedSettings {
            protocol_id: 0,
            private_key: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            compression: CompressionConfig::None,
        },
    }
}
