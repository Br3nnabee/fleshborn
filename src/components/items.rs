use bevy::ecs::component::*;
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
}

#[derive(Component, Debug)]
pub struct Container;
