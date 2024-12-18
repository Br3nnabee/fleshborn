use crate::game::common::Weight;
use crate::game::items::Inventory;
use crate::ui::ui::DisplayName;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
        app.add_systems(Update, player_movement);
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        app.add_event::<PlayerWalk>();
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub input_manager: InputManagerBundle<PlayerAction>,
    pub name: Name,
    pub display_name: DisplayName,
    pub inventory: Inventory,
    pub stats: StatsBundle,
    pub traits: PlayerTraits,
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Down, KeyCode::KeyR);
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Right, KeyCode::KeyS);

        input_map
    }
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

pub fn init_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                input_manager: InputManagerBundle::with_map(PlayerBundle::default_input_map()),
            },
            SceneBundle {
                scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("BasePlayer.glb")),
                ..default()
            },
        ))
        .id();
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
}

impl PlayerAction {
    const DIRECTIONS: [(Self, Vec3); 4] = [
        (PlayerAction::Up, Vec3::new(-1.0, 0.0, -1.0)),
        (PlayerAction::Down, Vec3::new(1.0, 0.0, 1.0)),
        (PlayerAction::Left, Vec3::new(-1.0, 0.0, 1.0)),
        (PlayerAction::Right, Vec3::new(1.0, 0.0, -1.0)),
    ];

    fn direction(self) -> Option<Vec3> {
        PlayerAction::DIRECTIONS
            .iter()
            .find(|&&(action, _)| action == self)
            .map(|&(_, dir)| dir)
    }
}

#[derive(Event)]
pub struct PlayerWalk {
    pub direction: Vec3,
}

fn player_movement(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut transforms: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    mut event_writer: EventWriter<PlayerWalk>,
) {
    let action_state = query.single();
    let mut direction = Vec3::ZERO;

    for &(input_action, dir) in PlayerAction::DIRECTIONS.iter() {
        if action_state.pressed(&input_action) {
            direction += dir;
        }
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    let mut player_transform = transforms.single_mut();
    let movement = direction * 2.0 * time.delta_seconds();
    player_transform.translation += movement;

    if direction.length_squared() > 0.0 {
        event_writer.send(PlayerWalk { direction });
    }
}
