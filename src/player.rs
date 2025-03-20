use avian3d::prelude::*;
use bevy::{
    input::{common_conditions::input_just_pressed, mouse::AccumulatedMouseMotion},
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
use std::f32::consts::FRAC_PI_4;

use crate::{app, arcball, ball, bricks};

const SQRT_3: f32 = 1.73205_f32;
const PADDLE_Z_LENGTH: f32 = 1.0;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((WireframePlugin, arcball::plugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                fire_ball
                    .run_if(input_just_pressed(KeyCode::Space).and(in_state(app::AppState::Ready))),
            ),
        )
        .add_systems(OnEnter(app::AppState::Ready), stage_ball);
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Paddle;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // Radius of sphere enclosing cube of side N is (N√3)/2
    let enclosing_radius = (bricks::CUBE_SIZE as f32 * SQRT_3) / 2.0;
    let projection = PerspectiveProjection::default();
    let near = projection.near;
    commands
        .spawn((
            Player,
            arcball::ArcBallController::new(enclosing_radius * 2.0),
            Camera3d::default(),
            Projection::Perspective(projection),
        ))
        .with_children(|parent| {
            parent.spawn(PointLight::default());
            parent.spawn((
                Paddle,
                RigidBody::Kinematic,
                Restitution::new(1.0),
                Collider::cuboid(ball::BALL_RADIUS, ball::BALL_RADIUS, PADDLE_Z_LENGTH),
                Wireframe,
                Mesh3d(meshes.add(Cuboid::new(
                    ball::BALL_RADIUS,
                    ball::BALL_RADIUS,
                    PADDLE_Z_LENGTH,
                ))),
                Transform::from_xyz(0.0, 0.0, -(near + ball::BALL_RADIUS * 4.0)),
            ));
        });
}

fn move_player(
    controller: Single<Mut<arcball::ArcBallController>>,
    camera: Single<&Camera, With<Player>>,
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
            let horizontal_angle = (-delta.x / viewport_size.x) * FRAC_PI_4;
            let vertical_angle = (delta.y / viewport_size.y) * FRAC_PI_4;
            controller.rotate_xy(vertical_angle, horizontal_angle);
        }
    }
}

fn stage_ball(
    mut commands: Commands,
    ball: Single<(Entity, &mut Transform), With<ball::Ball>>,
    paddle: Single<Entity, With<Paddle>>,
) {
    let (ball_entity, mut ball_transform) = ball.into_inner();
    let paddle_entity = paddle.into_inner();
    commands.entity(paddle_entity).add_child(ball_entity);
    *ball_transform = Transform::from_xyz(0.0, 0.0, -PADDLE_Z_LENGTH / 2.0);
}

fn fire_ball(mut next_state: ResMut<NextState<app::AppState>>) {
    next_state.set(app::AppState::Breaking);
}
