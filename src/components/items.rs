use bevy::ecs::bundle::*;
use bevy::ecs::component::*;
use bevy::ecs::entity::Entity;
use rustc_hash::FxHashMap;
use serde::Deserialize;

#[derive(Component, Debug, Clone)]
pub struct Item;

#[derive(Component, Debug, Clone, Deserialize)]
pub struct UseDelta(pub f32);

#[derive(Component, Debug, Clone, Deserialize)]
pub struct UseAmount(pub f32);

#[derive(Debug, Clone, Deserialize)]
pub enum PropertyValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    Text(String),
}

#[derive(Component, Debug, Clone, Deserialize)]
pub struct ItemProperties(pub FxHashMap<String, PropertyValue>);

#[derive(Component, Debug)]
pub struct Inventory {
    pub weight_limit: Option<f32>,
    pub items: Vec<Entity>,
}

#[derive(Component, Debug)]
pub struct Container;

#[derive(Component, Debug)]
pub struct ParentContainer(Entity);

#[derive(Bundle)]
pub struct ContainerBundle {
    pub marker: Container,
    pub inventory: Inventory,
}
