use bevy::{input::ButtonInput, math::Vec3, prelude::*, render::camera::Camera};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct Render2d;

impl Plugin for Render2d {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);
        app.add_plugins(TiledMapPlugin);
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_systems(Update, movement);
        app.add_systems(Startup, startup);
    }
}

const MINIMUM_SCALE: f32 = 0.1;

// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyR) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < MINIMUM_SCALE {
            ortho.scale = MINIMUM_SCALE;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let map_handle: Handle<TiledMap> = asset_server.load("isometric_map.tmx");
    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        render_settings: TilemapRenderSettings {
            // bevy_ecs_tilemap provide the 'y_sort' parameter to
            // sort chunks using their y-axis position during rendering.
            // However, it applies to whole chunks, not individual tile,
            // so we have to force the chunk size to be exactly one tile
            render_chunk_size: UVec2::new(1, 1),
            y_sort: true,
        },
        tiled_settings: TiledMapSettings {
            // Not related to current example, but center the map
            map_positioning: MapPositioning::Centered,
            ..default()
        },
        ..Default::default()
    });
}
