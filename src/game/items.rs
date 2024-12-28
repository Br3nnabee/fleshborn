use crate::fxhashset;
use crate::game::common::*;
use crate::ui::ui::*;
use bevy::prelude::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::Deserialize;
use std::time::Instant;

pub struct Items;

impl Plugin for Items {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemStorage {
            items: Default::default(),
        });
        app.add_systems(PreStartup, initialize_dictionary);
        app.add_systems(Startup, (generate_container_items));
        app.add_systems(PostStartup, fetch_item_info);
    }
}

#[derive(Component, Debug, Clone)]
pub struct Item;

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
    pub weight_limit: f32,
    pub items: FxHashSet<Entity>,
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

#[derive(Resource)]
pub struct ItemStorage {
    pub items: FxHashMap<Name, RawItemData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawItemData {
    pub display_name: DisplayName,
    pub weight: Weight,
    pub icon: Icon,
    pub tags: Tags,
    pub properties: ItemProperties,
}

pub fn spawn_item(
    commands: &mut Commands,
    item_storage: &Res<ItemStorage>,
    name: &str,
) -> Option<Entity> {
    let item_name = Name::new(name.to_string());  
    if let Some(item) = item_storage.items.get(&item_name) {
        let mut entity = commands.spawn((Item, item_name.clone()));
        entity.insert(item.display_name.clone());
        entity.insert(item.weight.clone());
        entity.insert(item.icon.clone());
        entity.insert(item.tags.clone());
        entity.insert(item.properties.clone());
        println!("Successfully spawned {}.", &item_name);
        Some(entity.id())
    } else {
        println!("Item '{}' not found in item storage.", &item_name);
        None
    }
}

pub fn spawn_potion(mut commands: Commands, item_storage: Res<ItemStorage>) {
    let name = "potion".to_string();
    let _ = spawn_item(&mut commands, &item_storage, name.as_str());
}

pub fn initialize_dictionary(mut commands: Commands, mut item_storage: ResMut<ItemStorage>) {
    // TODO: Add deserialization and reading from JSON

    let items_data = vec![
        (
            "sword",
            RawItemData {
                display_name: DisplayName("Sword".to_string()),
                weight: Weight(1.0),
                icon: Icon("Icon_Sword".to_string()),
                tags: Tags(fxhashset!["Weapon".to_string()]),
                properties: ItemProperties({
                    let mut props = FxHashMap::default();
                    props.insert(String::from("Damage"), PropertyValue::Int(30));
                    props
                }),
            },
        ),
        (
            "shield",
            RawItemData {
                display_name: DisplayName("Shield".to_string()),
                weight: Weight(5.0),
                icon: Icon("Icon_Shield".to_string()),
                tags: Tags(fxhashset!["Armor".to_string()]),
                properties: ItemProperties({
                    let mut props = FxHashMap::default();
                    props.insert(String::from("Defense"), PropertyValue::Int(50));
                    props
                }),
            },
        ),
        (
            "potion",
            RawItemData {
                display_name: DisplayName("Potion".to_string()),
                weight: Weight(0.25),
                icon: Icon("Icon_Potion".to_string()),
                tags: Tags(fxhashset!["Healing".to_string(), "Consumable".to_string()]),
                properties: ItemProperties({
                    let mut props = FxHashMap::default();
                    props.insert(String::from("Healing"), PropertyValue::Int(30));
                    props
                }),
            },
        ),
    ];

    let start = Instant::now();
    for (name, data) in items_data {
        let item_name = Name::new(name);
        item_storage.items.insert(item_name, data);
    }

    let duration = start.elapsed();
    println!(
        "Item dictionary initialized with {} items in {:?}.",
        item_storage.items.len(),
        duration
    );
}

pub fn fetch_item_info(
    query: Query<(&Name, &DisplayName, &Weight, &Icon, &Tags, &ItemProperties), With<Item>>,
) {
    println!("Items:");

    for (name, display_name, weight, icon, tags, item_properties) in query.iter() {
        println!("  {}", name.as_str());
        println!("    DisplayName: {}", display_name.0);
        println!("    Weight: {}", weight.0);
        println!("    Icon: {}", icon.0);
        println!("    Tags:");
        for tag in &tags.0 {
            println!("      {}", tag);
        }
        println!("    Properties:");
        for (key, value) in &item_properties.0 {
            println!(
                "      {}: {}",
                key,
                match value {
                    PropertyValue::Bool(val) => format!("{}", val),
                    PropertyValue::Int(val) => format!("{}", val),
                    PropertyValue::Float(val) => format!("{}", val),
                    PropertyValue::Text(val) => val.clone(),
                }
            );
        }
    }
}

pub fn spawn_container(commands: &mut Commands) -> Entity {
    let container: Entity = commands
        .spawn(ContainerBundle {
            marker: Container,
            inventory: Inventory {
                weight_limit: 10.0,
                items: FxHashSet::default(),
            },
        })
        .id();
    println!("Container spawned with id: {}", container);
    container
}

pub fn generate_container_items(mut commands: Commands, item_storage: Res<ItemStorage>) {
    for _ in 0..2 {
        let container_entity: Entity = commands.spawn(()).id();
        let item_list: Vec<Name> = item_storage.items.keys().cloned().collect();

        let mut rng = thread_rng();
        if let Some(name) = item_list.choose(&mut rng) {
            let item: Entity =
                spawn_item(&mut commands, &item_storage, name.as_str()).expect("Boo");
            let mut inventory = FxHashSet::default();
            inventory.insert(item);
            commands
                .entity(container_entity)
                .insert(ContainerBundle {
                    marker: Container,
                    inventory: Inventory {
                        weight_limit: 10.0,
                        items: inventory,
                    },
                })
                .insert(Name::new("Container"));
            commands
                .entity(item)
                .insert(ParentContainer(container_entity));
        };
    }
}



