use bevy::prelude::*;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use smallvec::SmallVec;
use std::time::Instant;

#[derive(Component, Debug, Clone)]
struct Item;

#[derive(Component, Debug, Clone, Deserialize)]
struct DisplayName(String);

#[derive(Component, Debug, Clone, Deserialize)]
struct Weight(f32);

#[derive(Component, Debug, Clone, Deserialize)]
struct UseDelta(f32);

#[derive(Component, Debug, Clone, Deserialize)]
struct UseAmount(f32);

#[derive(Component, Debug, Clone, Deserialize)]
struct Icon(String);

#[derive(Component, Debug, Clone, Deserialize)]
struct Tags(SmallVec<[String; 4]>);

#[derive(Debug, Clone, Deserialize)]
enum PropertyValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    Text(String),
}

#[derive(Component, Debug, Clone, Deserialize)]
struct ItemProperties(FxHashMap<String, PropertyValue>);

#[derive(Debug, Clone, Deserialize)]
struct RawItemData {
    displayname: Option<DisplayName>,
    weight: Option<Weight>,
    use_delta: Option<UseDelta>,
    icon: Option<Icon>,
    tags: Option<Tags>,
    properties: Option<ItemProperties>,
}

#[derive(Resource)]
struct ItemStorage {
    items: FxHashMap<Name, RawItemData>,
}

fn spawn_item(mut commands: Commands, item_storage: Res<ItemStorage>, name: String) {
    let start = Instant::now();
    let item_name = Name::new(name);
    if let Some(item) = item_storage.items.get(&item_name) {
        let mut entity = commands.spawn((Item, item_name.clone()));

        if let Some(displayname) = &item.displayname {
            entity.insert(displayname.clone());
        }
        if let Some(weight) = &item.weight {
            entity.insert(weight.clone());
        }
        if let Some(use_delta) = &item.use_delta {
            entity.insert(use_delta.clone());
            entity.insert(UseAmount(1.0));
        }
        if let Some(icon) = &item.icon {
            entity.insert(icon.clone());
        }
        if let Some(tags) = &item.tags {
            entity.insert(tags.clone());
        }
        if let Some(properties) = &item.properties {
            entity.insert(properties.clone());
        }

        let duration = start.elapsed();
        println!("Successfully spawned {} in {:?}.", &item_name, duration);
    } else {
        println!("Item '{}' not found in item storage.", &item_name);
    }
}

fn spawn_potion(mut commands: Commands, item_storage: Res<ItemStorage>) {
    let name = "potion".to_string();
    spawn_item(commands, item_storage, name)
}

fn initialize_dictionary(mut commands: Commands, mut item_storage: ResMut<ItemStorage>) {
    //TODO: Add deserialization and reading from json

    let items_data = vec![
        (
            "sword",
            RawItemData {
                displayname: Some(DisplayName("Sword".to_string())),
                weight: Some(Weight(1.0)),
                use_delta: None,
                icon: None,
                tags: None,
                properties: Some(ItemProperties({
                    let mut props = FxHashMap::default();
                    props.insert(String::from("Damage"), PropertyValue::Int(30));
                    props
                })),
            },
        ),
        (
            "shield",
            RawItemData {
                displayname: Some(DisplayName("Shield".to_string())),
                weight: Some(Weight(5.0)),
                use_delta: None,
                icon: None,
                tags: None,
                properties: Some(ItemProperties({
                    let mut props = FxHashMap::default();
                    props.insert(String::from("Defense"), PropertyValue::Int(50));
                    props
                })),
            },
        ),
        (
            "potion",
            RawItemData {
                displayname: Some(DisplayName("Potion".to_string())),
                weight: Some(Weight(0.25)),
                use_delta: Some(UseDelta(0.125)),
                icon: Some(Icon("Icon".to_string())),
                tags: Some(Tags(SmallVec::from_vec(vec![
                    "Healing".to_string(),
                    "Consumable".to_string(),
                ]))),
                properties: Some(ItemProperties({
                    let mut props = FxHashMap::default();
                    props.insert(String::from("Healing"), PropertyValue::Int(30));
                    props
                })),
            },
        ),
    ];

    for (name, data) in items_data {
        let item_name = Name::new(name);
        item_storage.items.insert(item_name, data);
    }

    println!(
        "Item dictionary initialized with {} items.",
        item_storage.items.len()
    );
}

fn fetch_item_info(
    query: Query<
        (
            &Name,
            Option<&DisplayName>,
            Option<&Weight>,
            Option<&UseDelta>,
            Option<&UseAmount>,
            Option<&Icon>,
            Option<&Tags>,
            Option<&ItemProperties>,
        ),
        With<Item>,
    >,
) {
    println!("Items:");
    for (
        name,
        displayname_option,
        weight_option,
        usedelta_option,
        useamount_option,
        icon_option,
        tags_option,
        itemproperties_option,
    ) in query.iter()
    {
        println!("  {}", name.as_str());

        if let Some(displayname) = displayname_option {
            println!("    DisplayName: {}", displayname.0);
        }

        if let Some(weight) = weight_option {
            println!("    Weight: {}", weight.0);
        }

        if let Some(usedelta) = usedelta_option {
            println!("    UseDelta: {}", usedelta.0);
        }

        if let Some(useamount) = useamount_option {
            println!("    UseAmount: {}", useamount.0);
        }

        if let Some(icon) = icon_option {
            println!("    Icon: {}", icon.0);
        }

        if let Some(tags) = tags_option {
            println!("    Tags:");
            for (tag) in &tags.0 {
                println!("      {:?}", tag)
            }
        }

        if let Some(itemproperties) = itemproperties_option {
            println!("    Properties:");
            for (key, value) in &itemproperties.0 {
                match value {
                    PropertyValue::Bool(val) => println!("      {}: {}", key, val),
                    PropertyValue::Int(val) => println!("      {}: {}", key, val),
                    PropertyValue::Float(val) => println!("      {}: {}", key, val),
                    PropertyValue::Text(val) => println!("      {}: {}", key, val),
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ItemStorage {
            items: FxHashMap::default(),
        })
        .add_systems(PreStartup, initialize_dictionary)
        .add_systems(Startup, (spawn_potion))
        .add_systems(PostStartup, (fetch_item_info))
        .run();
}
