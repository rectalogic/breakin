use std::f32::consts::{FRAC_PI_4, PI};

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::arcball;

const PI_2: f32 = 2.0 * PI;

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
    controller: Single<Mut<arcball::ArcBallController>>,
    camera: Single<&Camera>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        return;
    }

    let delta = mouse_motion.delta;
    if delta != Vec2::ZERO {
        if let Some(viewport_size) = camera.logical_viewport_size() {
            let viewport_size = viewport_size / 2.0;
            let mut controller = controller.into_inner();
            controller.yaw = (controller.yaw + (-delta.x / viewport_size.x) * FRAC_PI_4) % PI_2;
            controller.pitch = (controller.pitch + (delta.y / viewport_size.y) * FRAC_PI_4) % PI_2;
        }
    }
}
