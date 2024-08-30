use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Debug, Clone, Deserialize)]
pub struct DisplayName(pub String);

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Icon(pub String);
