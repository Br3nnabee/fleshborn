use bevy::prelude::{
    Commands,
    Startup,
    App,
    Plugin,
    default,
    Entity,
    Resource,
    ResMut,
    EventReader,
    Update,
    info,
};
use bevy::state::app::StatesPlugin;
use server::{ NetConfig, NetcodeConfig };
use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use bevy_rand::prelude::WyRand;
use bevy_rand::prelude::EntropyPlugin;
use rand_core::RngCore;
use rustc_hash::FxHashMap;

use lightyear::prelude::server::*;
pub use lightyear::prelude::*;
use lightyear::shared::config::Mode;
use lightyear::shared::replication::components::NetworkRelevanceMode::InterestManagement;

use crate::network::shared::{
    shared_config,
    SharedNetworkingPlugin,
    SERVER_ADDR,
    SERVER_REPLICATION_INTERVAL,
};
use crate::game::items::ItemsPlugin;
use crate::game::player::Player;
use crate::network::protocol::PlayerId;

// Defines a room. This is a (likely temporary) way to define when a player is spawned and should be replicated
const PLAYER_ROOM: RoomId = RoomId(0);

// Will be used frequently, eg interest management. Defines basic data.
#[derive(Resource, Default)]
pub struct Global {
    pub client_id_to_entity_id: FxHashMap<ClientId, Entity>,
    pub client_id_to_room_id: FxHashMap<ClientId, RoomId>,
}

// Super important function.
// Defines what to do when a connection is made. Currently includes only defining the client and stuff.
fn handle_connections(
    mut global: ResMut<Global>,
    mut room_manager: ResMut<RoomManager>,
    mut connections: EventReader<ConnectEvent>,
    mut commands: Commands
) {
    for connection in connections.read() {
        let client_id = connection.client_id;

        let replicate = Replicate {
            sync: SyncTarget {
                prediction: NetworkTarget::Single(client_id),
                interpolation: NetworkTarget::AllExceptSingle(client_id),
            },
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(client_id),
                ..default()
            },
            relevance_mode: InterestManagement,
            ..default()
        };

        let entity = commands.spawn((Player, PlayerId(client_id), replicate));
        info!("Player Entity Spawned");
        global.client_id_to_entity_id.insert(client_id, entity.id());
        global.client_id_to_room_id.insert(client_id, PLAYER_ROOM);
        room_manager.add_client(client_id, PLAYER_ROOM);
        room_manager.add_entity(entity.id(), PLAYER_ROOM);
    }
}


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
        app.add_systems(Update, handle_connections);
        app.insert_resource(Global::default());   
        app.add_plugins(ItemsPlugin);
        app.add_plugins(EntropyPlugin::<WyRand>::default());
    }
}

// Quick fn to start the server using lightyear commands
pub fn start_server(mut commands: Commands) {
    commands.start_server();
}
