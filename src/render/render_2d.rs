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
        app.add_systems(Startup, startup);
    }
}

const MINIMUM_SCALE: f32 = 0.1;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let map_handle: Handle<TiledMap> = asset_server.load("isometric_map.tmx");
    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        render_settings: TilemapRenderSettings {
            render_chunk_size: UVec2::new(1, 1),
            y_sort: true,
        },
        tiled_settings: TiledMapSettings {
            map_positioning: MapPositioning::Centered,
            ..default()
        },
        ..Default::default()
    });
}
