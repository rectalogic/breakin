// use bevy::ecs::change_detection::Mut;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, arcball);
}

// Based on this converted to Quat
// https://github.com/roy-t/roy-t.nl/blob/master/_posts/2010-02-21-xna-simple-arcballcamera.md
#[derive(Component, Default, Debug)]
#[require(Transform)]
pub(super) struct ArcBallController {
    pub look_at: Vec3,
    pub distance: f32,
    rotation: Quat,
}

impl ArcBallController {
    pub fn new(distance: f32) -> Self {
        Self {
            distance,
            ..default()
        }
    }

    pub fn rotate_xy(&mut self, x: f32, y: f32) {
        self.rotation *= Quat::from_rotation_x(x) * Quat::from_rotation_y(y);
    }
}

fn arcball(mut controller_query: Query<(Ref<ArcBallController>, &mut Transform)>) {
    let Ok((controller, mut transform)) = controller_query.get_single_mut() else {
        return;
    };

    if !controller.is_changed() {
        return;
    }
    // Calculate position based on quaternion orientation
    let forward = controller.rotation * (Vec3::Z * -controller.distance);

    transform.translation = controller.look_at + forward;

    // Set rotation so we look at the target point
    let up = controller.rotation * Vec3::Y;
    transform.look_at(controller.look_at, up);
}
