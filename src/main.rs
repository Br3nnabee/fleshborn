use crate::resources::items::ItemStorage;
use crate::systems::items::{fetch_item_info, initialize_dictionary, spawn_potion};
use bevy::prelude::*;
use systems::items::generate_container_items;

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
        .add_systems(Startup, (spawn_potion, generate_container_items))
        .add_systems(PostStartup, fetch_item_info)
        .run();
}
