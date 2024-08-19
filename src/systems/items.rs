use crate::components::common::{DisplayName, Icon, Tags, Weight};
use crate::components::items::*;
use crate::resources::items::{ItemStorage, RawItemData};
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::time::Instant;

pub fn spawn_item(
    mut commands: &mut Commands,
    item_storage: Res<ItemStorage>,
    name: String,
) -> Option<Entity> {
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
            Option<&ParentContainer>,
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
        parentcontainer_option,
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
            for tag in &tags.0 {
                println!("      {}", tag);
            }
        }

        if let Some(itemproperties) = itemproperties_option {
            println!("    Properties:");
            for (key, value) in &itemproperties.0 {
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

        if let Some(parentcontainer) = parentcontainer_option {
            println!("    Parent Container: {:?}", parentcontainer.0);
        }
    }
}

pub fn spawn_container(commands: &mut Commands) -> Entity {
    let container = commands
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

pub fn generate_container_items(
    mut commands: Commands,
    item_storage: Res<ItemStorage>,
    mut query: Query<&Inventory>,
) {
    let container_entity: Entity = spawn_container(&mut commands);
    println!("{}", container_entity);
    let item_list: Vec<Name> = item_storage.items.keys().cloned().collect();

    let mut rng = thread_rng();

    for (inventory) in &mut query {
        println!("Inventory: {:?}", inventory)
    }
}

pub fn add_item_to_container(
    mut commands: &mut Commands,
    item: Entity,
    container: Entity,
    mut query: Query<&Inventory>,
) {
    if let Ok(inventory) = query.get_mut(container) {
        // do something with the components
    } else {
        println!("Container inventory not found")
    }
}
