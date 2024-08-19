use crate::components::common::DisplayName;
use crate::components::items::Inventory;
use bevy::core::Name;
use bevy::ecs::bundle::*;
use bevy::ecs::component::*;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub marker: Player,
    pub name: Name,
    pub display_name: DisplayName,
    pub inventory: Inventory,
}
