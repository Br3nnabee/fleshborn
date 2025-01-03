#[cfg(feature = "server")]
use bevy::{
    app::{ PreStartup, Startup },
    prelude::{ Commands, Res, ResMut, Resource, EntityCommands },
};
use bevy::app::PostStartup;
use bevy::prelude::{ Plugin, Component, Reflect, Entity, Name, App, info, Query, With };
use bevy::utils::Instant;
use serde::Deserialize;
use rustc_hash::{ FxHashMap, FxHashSet };
#[cfg(feature = "server")]
use bevy_rand::prelude::{ GlobalEntropy, WyRand, ForkableRng };
#[cfg(feature = "server")]
use rand_core::RngCore;

use crate::utils::common::{ PropertyValue, Tags, Weight, DisplayName, Icon, choose_random };
use crate::{ fxhashset, fxhashmap };

pub struct ItemsPlugin;

// Quick plugin definition, nothing special.
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "server")]
        app.insert_resource(ItemStorage {
            items: Default::default(),
        });
        #[cfg(feature = "server")]
        app.add_systems(PreStartup, initialize_item_storage);
        #[cfg(feature = "server")]
        app.add_systems(Startup, (spawn_sword, generate_container_items));
        app.add_systems(PostStartup, fetch_item_info);
    }
}

#[derive(Component, Debug, Reflect)]
#[require(Name, DisplayName, Weight, Icon, Tags, ItemProperties)]
pub struct Item;

#[derive(Component, Debug, Clone, Deserialize)]
pub struct ItemProperties(pub FxHashMap<String, PropertyValue>);

impl Default for ItemProperties {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Component, Debug, Clone)]
pub struct Inventory {
    pub weight_limit: f32,
    pub items: FxHashSet<Entity>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self { weight_limit: 0.0, items: Default::default() }
    }
}

#[derive(Component, Debug, Reflect)]
#[require(Inventory)]
pub struct Container;

#[derive(Component, Debug, Reflect)]
pub struct ParentContainer(Entity);

// This will be the spawn dictionary. Everything that can be spawned in is defined here
#[cfg(feature = "server")]
#[derive(Resource)]
pub struct ItemStorage {
    pub items: FxHashMap<Name, RawItemData>,
}

// Primarily to clean things up and deserialize from files.
#[cfg(feature = "server")]
#[derive(Debug, Clone, Deserialize)]
pub struct RawItemData {
    pub display_name: DisplayName,
    pub weight: Weight,
    pub icon: Icon,
    pub tags: Tags,
    pub properties: ItemProperties,
}

// Placeholder to just define a couple of things to insert into the dictionary. Will essentially be the same after though.
#[cfg(feature = "server")]
fn initialize_item_storage(commands: Commands, mut item_storage: ResMut<ItemStorage>) {
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
                    fxhashmap! { String::from("Damage") => PropertyValue::Int(30) }
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
                    fxhashmap! { String::from("Defense") => PropertyValue::Int(50) }
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
                    fxhashmap! { String::from("Healing") => PropertyValue::Int(30) }
                }),
            },
        )
    ];

    let start = Instant::now();
    for (name, data) in items_data {
        let item_name = Name::new(name);
        item_storage.items.insert(item_name, data);
    }

    let duration = start.elapsed();
    info!("Item dictionary initialized with {} items in {:?}.", item_storage.items.len(), duration);
}

// Will spawn an item using its id (bevy Name) and the spawn dictionary.
#[cfg(feature = "server")]
fn spawn_item(
    commands: &mut Commands,
    item_storage: &Res<ItemStorage>,
    name: &str
) -> Option<Entity> {
    let item_name = Name::new(name.to_string());
    if let Some(item) = item_storage.items.get(&item_name) {
        let entity = commands
            .spawn((
                // I really don't like having to clone stuff but I think this is the only way to handle it here.
                Item,
                item_name.clone(),
                item.display_name.clone(),
                item.weight.clone(),
                item.icon.clone(),
                item.tags.clone(),
                item.properties.clone(),
            ))
            .id();
        info!("Spawned item {} with id {}", item_name, entity);
        Some(entity)
    } else {
        None
    }
}

// Really basic, just spawns an entity and gives it the container component.
#[cfg(feature = "server")]
fn spawn_container(commands: &mut Commands) -> Entity {
    let entity = commands.spawn(Container).id();
    info!("Spawned container with id {}", entity);
    entity
}

// Still unfinished, transfers an item. Same function works for moving it into a container,
// out of a container, or from one container to another. Certainly feels suboptimal atm.
// TODO: Optimize, improve error handling, and clean up.
#[cfg(feature = "server")]
fn move_item(
    commands: &mut Commands,
    item: Entity,
    current_container: Option<Entity>,
    target_container: Option<Entity>,
    mut query_inventory: Query<&mut Inventory>,
    mut query_parent: Query<&mut ParentContainer>
) {
    match (current_container, target_container) {
        (Some(current), Some(target)) => {
            // TODO: Replace these clones and handle things in a more rusty/optimal way.
            let mut curr = query_inventory.get_mut(current).expect("Err").clone();
            let mut tar = query_inventory.get_mut(target).expect("Err").clone();
            if let Ok(mut parent) = query_parent.get_mut(item) {
                *parent = ParentContainer(target);
            }
            curr.items.remove(&item);
            tar.items.insert(item);
            info!(
                "Transferred item with id {:?} from container with id {:?} into container with id {:?}",
                item,
                current,
                target
            )
        }
        (Some(current), None) => {
            if let Ok(curr) = query_inventory.get_mut(current) {
                if let Ok(parent) = query_parent.get_mut(item) {
                    commands.entity(item).remove::<ParentContainer>();
                }
            }
            info!("Removed item with id {:?} from container with id {:?}", item, current);
        }
        (None, Some(target)) => {
            if let Ok(mut tar) = query_inventory.get_mut(target) {
                commands.entity(item).insert(ParentContainer(target));
                tar.items.insert(item);
            }
            info!("Added item with id {:?} to container with id {:?}", item, target);
        }
        (None, None) => {
            warn!("No containers specified for move operation");
        }
    }
}

#[cfg(feature = "server")]
fn spawn_sword(mut commands: Commands, item_storage: Res<ItemStorage>) {
    spawn_item(&mut commands, &item_storage, "sword");
}

// Queries all items's data atm, more of a debugging tool. Will be shifted to be able to query specific items.
fn fetch_item_info(
    query: Query<(&Name, &DisplayName, &Weight, &Icon, &Tags, &ItemProperties), With<Item>>
) {
    info!("Queried items:");

    for (name, display_name, weight, icon, tags, item_properties) in query.iter() {
        println!(
            "  {}\n    DisplayName: {}\n    Weight: {}\n    Icon: {}\n    Tags: {:?}\n    Properties:",
            name.as_str(),
            display_name.0,
            weight.0,
            icon.0,
            tags.0
        );

        for (key, value) in &item_properties.0 {
            let value_str = match value {
                PropertyValue::Bool(val) => val.to_string(),
                PropertyValue::Int(val) => val.to_string(),
                PropertyValue::Float(val) => val.to_string(),
                PropertyValue::Text(val) => val.clone(),
            };
            println!("      {}: {}", key, value_str);
        }
    }
}

// Placeholder/testing function to spanw a container and a random entity and insert it.
#[cfg(feature = "server")]
fn generate_container_items(
    mut commands: Commands,
    item_storage: Res<ItemStorage>,
    mut global_entropy: GlobalEntropy<WyRand>
) {
    let container_entity: Entity = commands.spawn(()).id();
    let item_keys = item_storage.items.keys();
    let mut rng = global_entropy.fork_rng();

    if let Some(name) = choose_random(item_keys, &mut rng) {
        let item = spawn_item(&mut commands, &item_storage, name.as_str()).expect(
            "Failed to spawn item"
        );

        let inventory = Inventory { items: fxhashset![item], ..Default::default() };

        commands.entity(container_entity).insert((Container, inventory, Name::new("Container")));

        commands.entity(item).insert(ParentContainer(container_entity));
        info!("Spawned container with id {} and inserted item with id {}", container_entity, item)
    }
}
