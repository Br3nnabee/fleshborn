use crate::game::items::Items;
use bevy::prelude::*;

mod game;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((Items))
        .run();
}
