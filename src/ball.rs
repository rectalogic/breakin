use avian3d::prelude::*;
use bevy::color::palettes::basic;
use bevy::prelude::*;

use crate::{app, bricks, player};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(BallResource::default())
        .add_systems(Startup, setup)
        .add_systems(OnEnter(app::AppState::PlayBall), fire_ball)
        .add_systems(Update, update.run_if(in_state(app::AppState::PlayBall)));
}

pub(super) const BALL_RADIUS: f32 = bricks::INNER_CUBE_SIZE / 4.0;

#[derive(Component)]
#[require(Transform)]
pub(super) struct Ball;

#[derive(Resource, Default)]
pub(super) struct BallResource {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ball_resource: ResMut<BallResource>,
    mut next_state: ResMut<NextState<app::AppState>>,
) {
    ball_resource.mesh = meshes.add(Sphere::new(BALL_RADIUS).mesh().ico(4).unwrap());
    ball_resource.material = materials.add(Color::from(basic::RED));

    next_state.set(app::AppState::ReadyBall);
}

fn fire_ball(
    mut commands: Commands,
    ball_placeholder: Single<(Entity, &GlobalTransform), With<player::BallPlaceholder>>,
    ball_resource: Res<BallResource>,
) {
    let (ball_placeholder_entity, ball_placeholder_transform) = ball_placeholder.into_inner();
    let ball_placeholder_transform = ball_placeholder_transform.compute_transform();
    commands.spawn((
        Ball,
        RigidBody::Dynamic,
        Restitution::new(1.0),
        Mesh3d(ball_resource.mesh.clone()),
        MeshMaterial3d(ball_resource.material.clone()),
        Collider::sphere(BALL_RADIUS),
        CollisionLayers::new(
            app::GameLayer::Ball,
            [app::GameLayer::Brick, app::GameLayer::Paddle],
        ),
        ball_placeholder_transform,
        ExternalImpulse::new(ball_placeholder_transform.forward() * 0.5).with_persistence(false),
    ));
    commands
        .entity(ball_placeholder_entity)
        .insert(Visibility::Hidden);
}

fn update(
    mut commands: Commands,
    ball: Single<(Entity, &Transform), With<Ball>>,
    mut next_state: ResMut<NextState<app::AppState>>,
) {
    let (ball_entity, ball_transform) = ball.into_inner();
    if ball_transform.translation.distance(Vec3::ZERO) > player::PLAYFIELD_RADIUS {
        commands.entity(ball_entity).despawn();
        next_state.set(app::AppState::ReadyBall);
    }
}
