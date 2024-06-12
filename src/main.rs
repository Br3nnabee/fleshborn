use bevy::ecs::entity;
use bevy::{core::Name, ecs::query, prelude::*};
use bevy::{transform::commands, utils::HashMap};

#[derive(Component, Debug, Clone)]
struct Item;

#[derive(Component, Debug, Clone)]
struct DisplayName(String);

#[derive(Component, Debug, Clone)]
struct Weight(f32);

#[derive(Component, Debug, Clone)]
struct UseDelta(f32);

#[derive(Component, Debug, Clone)]
struct UseAmount(f32);

#[derive(Component, Debug, Clone)]
struct Icon(String);

#[derive(Component, Debug, Clone)]
struct Tags(Vec<String>);

//TODO: Find a more memory efficient structure for the below

#[derive(Debug, Clone)]
enum PropertyValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    Text(String),
}

#[derive(Component, Debug, Clone)]
struct ItemProperties(HashMap<String, PropertyValue>);

#[derive(Debug)]
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
    items: HashMap<Name, RawItemData>,
} //TODO: Test loading to and from disk

fn spawn_item(mut commands: Commands, item_storage: Res<ItemStorage>, name: String) {
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
        if let Some(properties) = &item.properties {
            entity.insert(properties.clone());
        }

        println!("Successfully spawned {}.", &item_name);
    } else {
        println!("Item '{}' not found in item storage.", &item_name);
    }
}

fn spawn_sword(mut commands: Commands, item_storage: Res<ItemStorage>) {
    let name = "potion".to_string();
    spawn_item(commands, item_storage, name)
}

fn initialize_dictionary(mut commands: Commands, mut item_storage: ResMut<ItemStorage>) {
    //TODO: Add deserialization and reading from msgpack

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
                    let mut props = HashMap::new();
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
                    let mut props = HashMap::new();
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
                tags: Some(Tags(vec!["Healing".to_string(), "Consumable".to_string()])),
                properties: None,
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

fn test_spawn_nonexistent_item(mut commands: Commands, item_storage: Res<ItemStorage>) {
    let name = "nonexistent_item".to_string();
    spawn_item(commands, item_storage, name)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ItemStorage {
            items: HashMap::new(),
        })
        .add_systems(Startup, (initialize_dictionary, spawn_sword))
        .add_systems(PostStartup, (fetch_item_info, test_spawn_nonexistent_item))
        .run();
}
