use crate::game::items::Items;
use crate::render::render_2d::Render2d;
use crate::render::render_3d::Render3d;
use bevy::prelude::*;
use game::player::PlayerPlugin;

use std::f32::consts::*;

mod game;
mod render;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((Items, Render3d, Render2d, PlayerPlugin))
        .run();
}
