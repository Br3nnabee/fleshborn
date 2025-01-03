use bevy::{math::Vec3A, prelude::*};
use client::ComponentSyncMode;
use leafwing_input_manager::Actionlike;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};

#[derive(Clone)]
pub struct ProtocolPlugin;

// Components protocol.
// Defines the kinds of components that can be sent over network

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(ClientId);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct PlayerPosition(Vec3A); // Use Vec3A for better performance

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerColor(Color);

// Below implementations needed for the linear interpolation

impl Add for PlayerPosition {
    type Output = PlayerPosition;
    #[inline]
    fn add(self, rhs: PlayerPosition) -> PlayerPosition {
        PlayerPosition(self.0.add(rhs.0))
    }
}

impl Mul<f32> for &PlayerPosition {
    type Output = PlayerPosition;

    fn mul(self, rhs: f32) -> Self::Output {
        PlayerPosition(self.0 * rhs)
    }
}

// Message protocol.
// Defines generic messages that can be sent over network.

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize); // Generic usize used as a placeholder

// Input protocol
// Defines all inputs that can be sent over network.
// Currently not following tutorial but docs/example on leafwing

// Defines an enum of ALL actions a player can do.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect, Actionlike)]
pub enum PlayerActions {
    Up,
    Down,
    Left,
    Right,
}

// Channel protocol
// Currently unnecesary, defines properties of message sending.
// e.g. reliability, ordering, priority

#[derive(Channel)]
pub struct Channel1;

// Plugin Implementation. Puts it all together.
impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Messages
        app.register_message::<Message1>(ChannelDirection::Bidirectional);
        // Inputs
        app.add_plugins(LeafwingInputPlugin::<PlayerActions>::default());
        // Components
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<PlayerPosition>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_linear_interpolation_fn();

        app.register_component::<PlayerColor>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
        // Channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
