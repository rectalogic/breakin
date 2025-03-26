use crate::{ball, bricks, player};
use avian3d::prelude::*;
use bevy::{DefaultPlugins, app::App, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BreakIn".into(),
                ..default()
            }),
            ..default()
        }),
        PhysicsPlugins::default(),
        bricks::plugin,
        player::plugin,
        ball::plugin,
    ))
    .insert_resource(Gravity(Vec3::ZERO))
    .insert_state(AppState::Init);
}

#[derive(States, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) enum AppState {
    Init,
    ReadyBall,
    PlayBall,
}

#[derive(PhysicsLayer, Default)]
pub(super) enum GameLayer {
    #[default]
    Default,
    Brick,
    Ball,
    Paddle,
}
