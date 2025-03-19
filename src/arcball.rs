// use bevy::ecs::change_detection::Mut;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, arcball);
}

// https://github.com/roy-t/roy-t.nl/blob/master/_posts/2010-02-21-xna-simple-arcballcamera.md
#[derive(Component, Default, Debug)]
#[require(Transform)]
pub(super) struct ArcBallController {
    pub look_at: Vec3,
    pub distance: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl ArcBallController {
    pub fn new(distance: f32) -> Self {
        Self {
            distance,
            ..default()
        }
    }
}

fn arcball(mut controller_query: Query<(Ref<ArcBallController>, &mut Transform)>) {
    let Ok((controller, mut transform)) = controller_query.get_single_mut() else {
        return;
    };

    if !controller.is_changed() {
        return;
    }
    // Calculate the relative position of the camera
    let mut position = Transform::from_rotation(Quat::from_euler(
        EulerRot::YXZ,
        controller.yaw,
        controller.pitch,
        0.0,
    ))
    .back()
    .as_vec3();

    // Convert the relative position to the absolute position
    position *= -controller.distance;
    position += controller.look_at;

    transform.translation = position;
    transform.look_at(controller.look_at, Vec3::Y);
}
