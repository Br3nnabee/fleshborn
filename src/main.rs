use crate::game::items::Items;
use crate::network::ProtocolPlugin;
use crate::render::{Render2d, Render3d};
use bevy::prelude::*;
use game::player::PlayerPlugin;

use std::f32::consts::*;

mod config;
mod game;
mod network;
mod render;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((Items, Render3d, Render2d, PlayerPlugin))
        .run();
}
