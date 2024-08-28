use crate::components::common::{DisplayName, Icon, Tags, Weight};
use crate::components::items::*;
use crate::resources::items::{ItemStorage, RawItemData};
use bevy::prelude::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::time::Instant;

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
    pub items: Vec<Entity>,
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

pub fn spawn_item(
    mut commands: &mut Commands,
    item_storage: Res<ItemStorage>,
    name: String,
) -> Option<Entity> {
    let start = Instant::now();
    let item_name = Name::new(name);
    if let Some(item) = item_storage.items.get(&item_name) {
        let mut entity = commands.spawn((Item, item_name.clone()));

        if let Some(display_name) = &item.display_name {
            entity.insert(display_name.clone());
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
        Some(entity.id())
    } else {
        println!("Item '{}' not found in item storage.", &item_name);
        None
    }
}

pub fn spawn_potion(mut commands: Commands, item_storage: Res<ItemStorage>) {
    let name = "potion".to_string();
    let _ = spawn_item(&mut commands, item_storage, name);
}

pub fn initialize_dictionary(mut commands: Commands, mut item_storage: ResMut<ItemStorage>) {
    // TODO: Add deserialization and reading from JSON

    let items_data = vec![
        (
            "sword",
            RawItemData {
                display_name: Some(DisplayName("Sword".to_string())),
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
                display_name: Some(DisplayName("Shield".to_string())),
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
                display_name: Some(DisplayName("Potion".to_string())),
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
        display_name_option,
        weight_option,
        use_delta_option,
        use_amount_option,
        icon_option,
        tags_option,
        item_properties_option,
    ) in query.iter()
    {
        println!("  {}", name.as_str());

        if let Some(display_name) = display_name_option {
            println!("    DisplayName: {}", display_name.0);
        }

        if let Some(weight) = weight_option {
            println!("    Weight: {}", weight.0);
        }

        if let Some(use_delta) = use_delta_option {
            println!("    UseDelta: {}", use_delta.0);
        }

        if let Some(use_amount) = use_amount_option {
            println!("    UseAmount: {}", use_amount.0);
        }

        if let Some(icon) = icon_option {
            println!("    Icon: {}", icon.0);
        }

        if let Some(tags) = tags_option {
            println!("    Tags:");
            for tag in &tags.0 {
                println!("      {}", tag);
            }
        }

        if let Some(item_properties) = item_properties_option {
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
}

pub fn spawn_container(commands: &mut Commands) -> Entity {
    let container: Entity = commands
        .spawn(
            (ContainerBundle {
                marker: Container,
                inventory: Inventory {
                    weight_limit: Some(10.0),
                    items: Vec::new(),
                },
            }),
        )
        .id();
    println!("Container spawned with id: {}", container);
    container
}

pub fn generate_container_items(mut commands: Commands, item_storage: Res<ItemStorage>) {
    let container_entity: Entity = commands.spawn(()).id();
    let item_list: Vec<Name> = item_storage.items.keys().cloned().collect();

    let mut rng = thread_rng();
    if let Some(name) = item_list.choose(&mut rng) {
        let item: Entity =
            spawn_item(&mut commands, item_storage, name.as_str().to_string()).expect("Boo");
        let mut inventory = Vec::new();
        inventory.push(item);
        commands.entity(container_entity).insert(ContainerBundle {
            marker: Container,
            inventory: Inventory {
                weight_limit: Some(10.0),
                items: (inventory),
            },
        });
    }
}
