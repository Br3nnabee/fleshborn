use bevy::prelude::{ Commands, Camera2d, Startup, App, Plugin, default };
use client::{ Authentication, ClientTransport, NetConfig, NetcodeConfig };
use std::net::{ IpAddr, Ipv4Addr, SocketAddr };

use lightyear::client::plugin::ClientPlugins;
use lightyear::prelude::client::*;
pub use lightyear::prelude::*;
use lightyear::shared::config::Mode;

use crate::network::shared::{ shared_config, SharedNetworkingPlugin, SERVER_ADDR };

pub struct ClientNetworkingPlugin;

// Simply fetch client address
const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);

// Builds the client plugin for when app is run as client
fn build_client_plugin() -> ClientPlugins {
    // Specifies how the client should connect to server
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 0,
        private_key: Key::default(),
        protocol_id: 0,
    };

    // Specifies transport type to use during communication
    let io = IoConfig {
        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
        ..default()
    };

    // Sets how the connection is actually established.
    // Only needs to really be changed if we want to change netcode,
    // or use steam networking.
    let net_config = NetConfig::Netcode {
        auth,
        io,
        config: NetcodeConfig::default(),
    };

    // Sets the final config. Duh.
    // Will need to be changed in the future once we start planning out
    // things like prediction or interpolation.
    let config = ClientConfig {
        shared: shared_config(Mode::Separate),
        net: net_config,
        ..default()
    };

    ClientPlugins::new(config)
}

// Plugin implementation. Puts it all together.
impl Plugin for ClientNetworkingPlugin {
    fn build(&self, app: &mut App) {
        // Add client-specific systems/plugins
        app.add_systems(Startup, connect_client);
    }
}

// Quick startup fn to connect the client using lightyear commands
fn connect_client(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.connect_client();
}
