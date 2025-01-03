#[macro_export]
macro_rules! fxhashset {
    ($($x:expr),*) => {
        {
            let mut set = rustc_hash::FxHashSet::default();
            $(
                set.insert($x);
            )*
            set
        }
    };
}

#[macro_export]
macro_rules! fxhashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = rustc_hash::FxHashMap::default();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}
