use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    //XXX make these children of the camera, so as we move we shine light at cube
    //XXX also need a collider that covers the "near" plane
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 0.0, -17.0).looking_at(Vec3::ZERO, Dir3::Y),
        ))
        .with_child(PointLight::default());
}
