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
}

fn spawn_sword(mut commands: Commands, item_storage: Res<ItemStorage>) {
    let item_name = Name::new("sword".to_string());
    if let Some(sword) = item_storage.items.get(&item_name) {
        let mut entity = commands.spawn((Item, item_name.clone()));

        if let Some(displayname) = &sword.displayname {
            entity.insert(displayname.clone());
        }
        if let Some(weight) = &sword.weight {
            entity.insert(weight.clone());
        }
        if let Some(use_delta) = &sword.use_delta {
            entity.insert(use_delta.clone());
            entity.insert(UseAmount(1.0));
        }
        if let Some(icon) = &sword.icon {
            entity.insert(icon.clone());
        }
        if let Some(properties) = &sword.properties {
            entity.insert(properties.clone());
        }

        println!("Successfully spawned sword.");
    } else {
        println!("Sword not found in item storage.");
    }
}

fn initialize_dictionary(mut commands: Commands, mut item_storage: ResMut<ItemStorage>) {
    let item_name = Name::new("sword");

    let mut properties = HashMap::new();
    properties.insert(String::from("Damage"), PropertyValue::Int(30));

    let item = RawItemData {
        displayname: Some(DisplayName("Sword".to_string())),
        weight: Some(Weight(1.0)),
        use_delta: None,
        icon: None,
        tags: None,
        properties: Some(ItemProperties(properties)),
    };

    item_storage.items.insert(item_name, item);
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
            println!("    Properties: {:?}", itemproperties.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ItemStorage {
            items: HashMap::new(),
        })
        .add_systems(Startup, (initialize_dictionary, spawn_sword))
        .add_systems(PostStartup, fetch_item_info)
        .run();
}
