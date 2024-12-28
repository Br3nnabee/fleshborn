use bevy::prelude::*;
use rustc_hash::FxHashSet;
use serde::Deserialize;

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Weight(pub f32);

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Tags(pub FxHashSet<String>);

#[macro_export]
macro_rules! fxhashset {
    ( $( $x:expr ),* ) => {
        {
            let mut set = FxHashSet::default();
            $(
                set.insert($x);
            )*
            set
        }
    };
}
