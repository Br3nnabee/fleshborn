use crate::resources::items::ItemStorage;
use crate::systems::items::{fetch_item_info, initialize_dictionary, spawn_potion};
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ItemStorage {
            items: Default::default(),
        })
        .add_systems(PreStartup, initialize_dictionary)
        .add_systems(Startup, spawn_potion)
        .add_systems(PostStartup, fetch_item_info)
        .run();
}
