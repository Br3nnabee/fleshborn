use bevy::prelude::*;
use bevy::utils::Duration;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use lightyear::prelude::{SharedPlugin, *};
use lightyear::shared::config::Mode;

use crate::network::protocol::ProtocolPlugin;

// Essentially the tickrate
pub const FIXED_TIMESTEP_HZ: f64 = 32.0;

// Rate for server to send data. Keep high enough to prevent bandwith clogging
pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

// Just grabs the server ip
pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

// Builds the shared config whenever the app is run since it's for both.
pub fn shared_config(mode: Mode) -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: (SERVER_REPLICATION_INTERVAL),
        tick: (TickConfig {
            tick_duration: (Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ)),
        }),
        mode,
    }
}

#[derive(Clone)]
pub struct SharedNetworkingPlugin;

impl Plugin for SharedNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin);
    }
}
