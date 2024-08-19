use crate::components::common::{DisplayName, Icon, Tags, Weight};
use crate::components::items::{Item, ItemProperties, PropertyValue, UseAmount, UseDelta};
use bevy::core::Name;
use bevy::ecs::prelude::Resource;
use rustc_hash::FxHashMap;
use serde::Deserialize;

#[derive(Resource)]
pub struct ItemStorage {
    pub items: FxHashMap<Name, RawItemData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawItemData {
    pub displayname: Option<DisplayName>,
    pub weight: Option<Weight>,
    pub use_delta: Option<UseDelta>,
    pub icon: Option<Icon>,
    pub tags: Option<Tags>,
    pub properties: Option<ItemProperties>,
}
