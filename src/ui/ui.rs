use bevy::prelude::*;
use iyes_perf_ui::prelude::*;
use serde::Deserialize;

pub struct UI;

impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
        app.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin);
        app.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);
        app.add_plugins(PerfUiPlugin);
        app.add_systems(Startup, setup);
    }
}

#[derive(Component, Debug, Clone, Deserialize)]
pub struct DisplayName(pub String);

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Icon(pub String);

fn setup(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}
