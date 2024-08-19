use crate::components::common::DisplayName;
use crate::components::items::Inventory;
use crate::components::player::*;
use bevy::prelude::*;

fn init_player(mut commands: Commands) -> Entity {
    let playername = "Joe";
    let display_name = "Yes".to_string();

    commands
        .spawn((
            PlayerBundle {
                marker: Player,
                name: Name::new(playername),
                display_name: DisplayName(display_name),
                inventory: Inventory {
                    weight_limit: Some(10.0),
                    items: Vec::new(),
                },
            },
            SpatialBundle {
                transform: Transform::from_scale(Vec3::splat(3.0)),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        ))
        .id()
}
