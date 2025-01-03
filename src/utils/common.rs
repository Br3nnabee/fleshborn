use bevy::prelude::{ Component, Reflect };
use serde::Deserialize;
use rustc_hash::FxHashSet;
use bevy_rand::prelude::{ WyRand, Entropy };
use rand_core::RngCore;

// Can be used for lots, but currently only using for item properties
#[derive(Debug, Clone, Deserialize)]
pub enum PropertyValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    Text(String),
}

#[derive(Component, Debug, Clone, Deserialize, Reflect)]
pub struct Weight(pub f32);

impl Default for Weight {
    fn default() -> Self {
        Self(0.0)
    }
}

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Tags(pub FxHashSet<String>);

impl Default for Tags {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Component, Clone, Debug, Deserialize, Reflect)]
pub struct DisplayName(pub String);

impl Default for DisplayName {
    fn default() -> Self {
        Self(String::from("No DisplayName Defined"))
    }
}

#[derive(Component, Clone, Debug, Deserialize, Reflect)]
pub struct Icon(pub String);

impl Default for Icon {
    fn default() -> Self {
        Self(String::from("No Icon"))
    }
}

// Uses the bevy_rand crate to choose randoms from a hashset.
pub fn choose_random<T, I>(iter: I, rng: &mut Entropy<WyRand>) -> Option<T>
    where I: IntoIterator<Item = T>
{
    let mut iter = iter.into_iter();
    let mut chosen = None;
    let mut count = 0;

    while let Some(item) = iter.next() {
        count += 1;
        if (rng.next_u32() as usize) % count == 0 {
            chosen = Some(item);
        }
    }

    chosen
}
