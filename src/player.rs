use std::f32::consts::FRAC_PI_4;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::arcball;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(arcball::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player);
}

fn setup(mut commands: Commands) {
    //XXX make these children of the camera, so as we move we shine light at cube
    //XXX also need a collider that covers the "near" plane

    commands
        .spawn((arcball::ArcBallController::new(17.0), Camera3d::default()))
        .with_child(PointLight::default());
}

fn move_player(
    controller: Single<&mut arcball::ArcBallController>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        return;
    }

    let delta = mouse_motion.delta;
    if delta != Vec2::ZERO {
        let mut controller = controller.into_inner();
        controller.yaw += delta.x * 0.01;
        controller.pitch += delta.y * 0.01;
    }
}
