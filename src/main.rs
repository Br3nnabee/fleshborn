use crate::game::items::Items;
use crate::render::{Render2d, Render3d};
use crate::ui::ui::UI;
use bevy::prelude::*;
use bevy::window::PresentMode;
use game::player::PlayerPlugin;

use std::f32::consts::*;

mod game;
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
