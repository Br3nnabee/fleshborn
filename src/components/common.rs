use bevy::ecs::component::*;
use serde::Deserialize;
use smallvec::SmallVec;

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Icon(pub String);

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Tags(pub SmallVec<[String; 4]>);

#[derive(Component, Debug, Clone, Deserialize)]
pub struct DisplayName(pub String);

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Weight(pub f32);
