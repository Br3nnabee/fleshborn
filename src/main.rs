use crate::game::items::Items;
use crate::network::ProtocolPlugin;
use crate::render::{Render2d, Render3d};
use crate::ui::ui::UI;
use bevy::prelude::*;
use bevy::window::PresentMode;
use game::player::PlayerPlugin;

use std::f32::consts::*;

mod config;
mod game;
mod network;
mod render;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((Items, Render3d, Render2d, PlayerPlugin, UI))
        .run();
}
