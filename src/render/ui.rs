#[cfg(feature = "client")]
use bevy::prelude::*;
use lightyear::prelude::*;
use lightyear::prelude::client::ClientCommands;
use lightyear::prelude::client::*;
use serde::Deserialize;

#[derive(Resource)]
struct GameName(String);

pub(crate) struct UiRenderPlugin {
    pub name: String,
}

impl UiRenderPlugin {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Plugin for UiRenderPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "gui")]
        app.insert_resource(GameName(self.name.clone()));
        #[cfg(feature = "gui")]
        app.add_systems(Startup, set_window_title);
        #[cfg(feature = "client")]
        spawn_connect_button(app)
    }
}

fn set_window_title(mut window: Query<&mut Window>, game_name: Res<GameName>) {
    let mut window = window.get_single_mut().unwrap();
    window.title = format!("Lightyear Example: {}", game_name.0);
}

#[cfg(feature = "client")]
#[derive(Component)]
struct StatusMessageMarker;

#[cfg(feature = "client")]
/// Create a button that allow you to connect/disconnect to a server
pub(crate) fn spawn_connect_button(app: &mut App) {
    app.world_mut()
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexEnd,
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text("Click to connect/Disconnect".to_string()),
                TextColor(Color::srgb(0.9, 0.9, 0.9).with_alpha(0.4)),
                TextFont::from_font_size(18.0),
                StatusMessageMarker,
                Node {
                    padding: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ));
            parent
                .spawn((
                    Text("Connect".to_string()),
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextFont::from_font_size(20.0),
                    BorderColor(Color::BLACK),
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                ))
                .observe(
                    |
                        _: Trigger<Pointer<Click>>,
                        mut commands: Commands,
                        state: Res<State<NetworkingState>>
                    | {
                        match state.get() {
                            NetworkingState::Disconnected => {
                                commands.connect_client();
                            }
                            NetworkingState::Connecting | NetworkingState::Connected => {
                                commands.disconnect_client();
                            }
                        };
                    }
                );
        });
}
