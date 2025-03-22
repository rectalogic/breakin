use avian3d::prelude::*;
use bevy::{
    input::{common_conditions::input_just_pressed, mouse::AccumulatedMouseMotion},
    prelude::*,
    render::camera::CameraProjection,
    window::WindowResized,
};
use std::f32::consts::FRAC_PI_4;

use crate::{app, arcball, ball, bricks};

const SQRT_3: f32 = 1.73205_f32;
const PADDLE_Z_LENGTH: f32 = 1.0;
// Radius of sphere enclosing cube of side N is (N√3)/2
const ENCLOSING_RADIUS: f32 = (bricks::CUBE_SIZE as f32 * SQRT_3) / 2.0;
pub(super) const PLAYFIELD_RADIUS: f32 = ENCLOSING_RADIUS * 2.0;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(arcball::plugin)
        .add_systems(Startup, setup)
        .add_systems(
            OnTransition {
                exited: app::AppState::Init,
                entered: app::AppState::ReadyBall,
            },
            create_ball_placeholder,
        )
        .add_systems(
            Update,
            (
                on_resize,
                move_player,
                fire_ball.run_if(
                    input_just_pressed(KeyCode::Space).and(in_state(app::AppState::ReadyBall)),
                ),
            ),
        )
        .add_systems(OnEnter(app::AppState::ReadyBall), stage_ball);
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
#[require(Transform)]
pub(super) struct BallPlaceholder;

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Player,
            RigidBody::Kinematic,
            arcball::ArcBallController::new(PLAYFIELD_RADIUS),
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection::default()),
        ))
        .with_children(|parent| {
            parent.spawn(PointLight::default());
            parent.spawn((
                Paddle,
                Restitution::new(1.0),
                Collider::cuboid(1.0, 1.0, PADDLE_Z_LENGTH),
                CollisionLayers::new(app::GameLayer::Paddle, [app::GameLayer::Ball]),
                Transform::default(),
            ));
        });
}

fn on_resize(
    mut commands: Commands,
    projection: Single<&Projection, With<Player>>,
    paddle: Single<(Entity, &mut Transform), With<Paddle>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    if resize_reader.read().last().is_some() {
        let projection = projection.into_inner();
        let near = match projection {
            Projection::Perspective(p) => p.near,
            Projection::Orthographic(p) => p.near,
        } + ball::BALL_RADIUS * 4.0;
        let corners = projection.get_frustum_corners(near, near + PADDLE_Z_LENGTH);
        // bottom right - bottom left
        let x_length = corners[0].x - corners[3].x;
        // top right - bottom left
        let y_length = corners[1].y - corners[3].y;
        let (paddle_entity, mut paddle_transform) = paddle.into_inner();
        paddle_transform.translation = Vec3::new(0.0, 0.0, -(near + PADDLE_Z_LENGTH / 2.0));
        commands.entity(paddle_entity).insert(Collider::cuboid(
            x_length,
            y_length,
            PADDLE_Z_LENGTH,
        ));
    }
}

fn create_ball_placeholder(
    mut commands: Commands,
    ball_resource: Res<ball::BallResource>,
    paddle: Single<Entity, With<Paddle>>,
) {
    let ball_entity = commands
        .spawn((
            BallPlaceholder,
            Visibility::Visible,
            Mesh3d(ball_resource.mesh.clone()),
            MeshMaterial3d(ball_resource.material.clone()),
            Transform::from_xyz(0.0, 0.0, -PADDLE_Z_LENGTH / 1.9),
        ))
        .id();
    let paddle_entity = paddle.into_inner();
    commands.entity(paddle_entity).add_child(ball_entity);
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

fn stage_ball(mut commands: Commands, ball: Single<Entity, With<BallPlaceholder>>) {
    let ball_entity = ball.into_inner();
    commands.entity(ball_entity).insert(Visibility::Visible);
}

fn fire_ball(mut next_state: ResMut<NextState<app::AppState>>) {
    next_state.set(app::AppState::PlayBall);
}
