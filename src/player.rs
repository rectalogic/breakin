use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::arcball;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(arcball::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, arcball);
}

fn setup(mut commands: Commands) {
    //XXX make these children of the camera, so as we move we shine light at cube
    //XXX also need a collider that covers the "near" plane
    info!("Setting up player"); //XXX
    commands
        .spawn((
            arcball::ArcBallController {
                distance: 17.0,
                ..default()
            },
            Camera3d::default(),
        ))
        .with_child(PointLight::default());
}

//XXX need ArcBall camera
// https://github.com/roy-t/roy-t.nl/blob/caaf939b776b673b47a5bb5af89233c3adabce79/_posts/2010-02-21-xna-simple-arcballcamera.md#L4
fn arcball(
    // mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        return;
    }

    let delta = mouse_motion.delta;
    dbg!(delta);
}
//XXX need orbit system - use mouse_motion: Res<AccumulatedMouseMotion>, and map 180 degrees to half width/height of viewport?
