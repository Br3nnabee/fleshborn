use crate::game::common::Weight;
use crate::game::items::Inventory;
use crate::ui::ui::DisplayName;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub marker: Player,
    pub name: Name,
    pub display_name: DisplayName,
    pub inventory: Inventory,
    pub stats: StatsBundle,
    pub traits: PlayerTraits,
}

#[derive(Bundle, Debug)]
pub struct StatsBundle {
    pub hunger: Hunger,
    pub thirst: Thirst,
    pub weight: Weight,
    pub calories: Calories,
    pub nutrients: Nutrients,
    pub metabolism: Metabolism,
    pub fatigue: Fatigue,
    pub endurance: Endurance,
    pub sanity: Sanity,
    pub unhappiness: Unhappiness,
    pub boredom: Boredom,
    pub stress: Stress,
    pub panic: Panic,
    pub drunkenness: Drunkenness,
}

impl Default for StatsBundle {
    fn default() -> Self {
        Self {
            hunger: Hunger(1.0),
            thirst: Thirst(1.0),
            weight: Weight(80.0),
            calories: Calories(2500.0),
            nutrients: Nutrients {
                proteins: 50.0,
                carbohydrates: 250.0,
                fats: 40.0,
                vitamin_c: 80.0,
                minerals: 20.0,
            },
            metabolism: Metabolism(1.0),
            fatigue: Fatigue(1.0),
            endurance: Endurance(100.0),
            sanity: Sanity(100.0),
            unhappiness: Unhappiness(0.0),
            boredom: Boredom(0.0),
            stress: Stress(0.0),
            panic: Panic(0.0),
            drunkenness: Drunkenness(0.0),
        }
    }
}

#[derive(Component, Debug)]
pub struct Hunger(pub f32);

#[derive(Component, Debug)]
pub struct Thirst(pub f32);

#[derive(Component, Debug)]
pub struct Calories(pub f32);

#[derive(Component, Debug, Default)]
pub struct Nutrients {
    pub proteins: f32,
    pub carbohydrates: f32,
    pub fats: f32,
    pub vitamin_c: f32,
    pub minerals: f32,
}

#[derive(Component, Debug)]
pub struct Metabolism(pub f32);

#[derive(Component, Debug)]
pub struct Fatigue(pub f32);

#[derive(Component, Debug)]
pub struct Endurance(pub f32);

#[derive(Component, Debug)]
pub struct Sanity(pub f32);

#[derive(Component, Debug)]
pub struct Unhappiness(pub f32);

#[derive(Component, Debug)]
pub struct Boredom(pub f32);

#[derive(Component, Debug)]
pub struct Stress(pub f32);

#[derive(Component, Debug)]
pub struct Panic(pub f32);

#[derive(Component, Debug)]
pub struct Drunkenness(pub f32);

#[derive(Component, Debug)]
pub struct PlayerTraits(pub Vec<String>);

fn init_player(mut commands: Commands) -> Entity {
    let player_name = "Joe";
    let display_name = "Yes".to_string();

    commands
        .spawn((
            PlayerBundle {
                marker: Player,
                name: Name::new(player_name),
                display_name: DisplayName(display_name),
                inventory: Inventory {
                    weight_limit: Some(10.0),
                    items: Vec::new(),
                },
                stats: StatsBundle::default(),
                traits: PlayerTraits(Vec::new()),
            },
            SpatialBundle {
                transform: Transform::from_scale(Vec3::splat(3.0)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        ))
        .id()
}