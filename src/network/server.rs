use bevy::prelude::{ Commands, Startup, App, Plugin, default };
use bevy::state::app::StatesPlugin;
use server::{ NetConfig, NetcodeConfig };
use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use bevy_rand::prelude::WyRand;
use bevy_rand::prelude::EntropyPlugin;
use rand_core::RngCore;

use lightyear::prelude::server::*;
pub use lightyear::prelude::*;
use lightyear::shared::config::Mode;

use crate::network::shared::{
    shared_config,
    SharedNetworkingPlugin,
    SERVER_ADDR,
    SERVER_REPLICATION_INTERVAL,
};
use crate::game::items::ItemsPlugin;

pub struct ServerNetworkingPlugin;

// Builds the server plugin for when app is run as server
fn build_server_plugin() -> ServerPlugins {
    // Specifies type of transport to use during communication
    let io = IoConfig {
        transport: ServerTransport::UdpSocket(SERVER_ADDR),
        ..default()
    };

    // Sets how the connection is actually established.
    // Only needs to really be changed if we want to change netcode,
    // or use steam networking.
    let net_config = NetConfig::Netcode {
        io,
        config: NetcodeConfig::default(),
    };

    // Sets the final config. Duh.
    // Will need to be changed in the future once we start planning out
    // things like prediction or interpolation.
    let config = ServerConfig {
        shared: shared_config(Mode::Separate),
        net: vec![net_config], // Multiple net configs can be added here
        replication: ReplicationConfig {
            send_interval: SERVER_REPLICATION_INTERVAL,
            ..default()
        },
        ..default()
    };
    ServerPlugins::new(config)
}

// Plugin implementation. Puts it all together.
impl Plugin for ServerNetworkingPlugin {
    fn build(&self, app: &mut App) {
        // Add server-specific systems/plugins
        app.add_systems(Startup, start_server);
        app.add_plugins(ItemsPlugin);
        app.add_plugins(EntropyPlugin::<WyRand>::default());
    }
}

// Quick fn to start the server using lightyear commands
pub fn start_server(mut commands: Commands) {
    commands.start_server();
}
