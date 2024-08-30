use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Weight(pub f32);

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Tags(pub Vec<String>);
