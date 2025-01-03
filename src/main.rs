/* Fleshborn

This is my take on an isometric zombie survival game, heavily inspired by project zomboid, dayz, 
and many other similar games. It is also a learning project, as it is not only my first bevy project,
but also my first larger rust project. My goals are currently rather far fetched considering the
scale of the project right now, but they include, in order of priority:

- 90% or more of the game written in rust.
- Joint 2d/3d rendering for tiles and moving entities respectively, similar to Project Zomboid.
- Highly optimized code. Aiming for minimum 60fps with 100 mods on potatoes.
- Fully networked multiplayer using QUIC. Should be able to handle 100+ player servers.
- Complex systems in every facet of the game, from medical to weather.
- Built-in modding support with optimized API and lua.
- Fully scripted AI for zombies, NPCs, and animals.
- 6th to 7th gen modern classic aesthetics and design.
- Completely genAI-free in assets and code.

*/

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[cfg(feature = "client")]
use crate::network::client::ClientNetworkingPlugin;
#[cfg(feature = "server")]
use crate::network::server::ServerNetworkingPlugin;
use crate::network::protocol::ProtocolPlugin;
#[cfg(feature = "gui")]
use crate::render::ui::UiRenderPlugin;
use bevy::prelude::*;
use bevy::utils::Duration;
use clap::{ Arg, Command };
use game::app::cli;
#[cfg(feature = "client")]
use lightyear::prelude::client::*;
#[cfg(feature = "server")]
use lightyear::prelude::server::*;
use lightyear::prelude::*;

use crate::utils::settings::*;
use crate::game::app::{ Cli, Apps };

mod utils;
mod network;
mod game;
#[cfg(feature = "gui")]
mod render;

fn main() {
    let cli = cli();
    #[allow(unused_mut)]
    let mut settings = get_settings();
    let mut apps = Apps::new(settings, cli, env!("CARGO_PKG_NAME").to_string()).unwrap();
    apps.add_lightyear_plugins();
    apps.add_user_shared_plugin(ProtocolPlugin);
    #[cfg(feature = "client")]
    apps.add_user_client_plugin(ClientNetworkingPlugin);
    #[cfg(feature = "server")]
    apps.add_user_server_plugin(ServerNetworkingPlugin);
    #[cfg(feature = "gui")]
    apps.add_user_renderer_plugin(UiRenderPlugin {
        name: env!("CARGO_PKG_NAME").to_string(),
    });
    apps.run();
}
