use avian3d::prelude::*;
use bevy::color::palettes::basic;
use bevy::prelude::*;

use crate::{app, bricks, player};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(BallResource::default())
        .add_systems(Startup, setup)
        // PostStartup because we need player::PaddleHolder to exist first
        .add_systems(PostStartup, setup_ball_placeholder)
        .add_systems(OnEnter(app::AppState::PlayBall), fire_ball)
        .add_systems(
            Update,
            (handle_ball_oob, handle_ball_collision).run_if(in_state(app::AppState::PlayBall)),
        );
}

pub(super) const BALL_RADIUS: f32 = bricks::INNER_CUBE_SIZE / 4.0;

#[derive(Component)]
#[require(Transform)]
pub(super) struct Ball;

#[derive(Component)]
#[require(Transform)]
pub(super) struct BallPlaceholder;

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

fn setup_ball_placeholder(
    mut commands: Commands,
    ball_resource: Res<BallResource>,
    paddle_holder: Single<Entity, With<player::PaddleHolder>>,
) {
    let ball_entity = commands
        .spawn((
            BallPlaceholder,
            Name::new("BallPlaceholder"),
            Visibility::Visible,
            Mesh3d(ball_resource.mesh.clone()),
            MeshMaterial3d(ball_resource.material.clone()),
            Transform::from_xyz(0.0, 0.0, -player::PADDLE_Z_LENGTH / 1.9),
        ))
        .id();
    let paddle_holder_entity = paddle_holder.into_inner();
    commands.entity(paddle_holder_entity).add_child(ball_entity);
}

fn fire_ball(
    mut commands: Commands,
    ball_placeholder: Single<(Entity, &GlobalTransform), With<BallPlaceholder>>,
    ball_resource: Res<BallResource>,
) {
    let (ball_placeholder_entity, ball_placeholder_transform) = ball_placeholder.into_inner();
    let ball_placeholder_transform = ball_placeholder_transform.compute_transform();
    commands.spawn((
        Ball,
        Name::new("Ball"),
        RigidBody::Dynamic,
        Restitution::new(1.0),
        MaxLinearSpeed(5.0),
        Mesh3d(ball_resource.mesh.clone()),
        MeshMaterial3d(ball_resource.material.clone()),
        Collider::sphere(BALL_RADIUS),
        CollisionLayers::new(
            app::GameLayer::Ball,
            [app::GameLayer::Brick, app::GameLayer::Paddle],
        ),
        ball_placeholder_transform,
        ExternalImpulse::new(ball_placeholder_transform.forward() * 0.3).with_persistence(false),
    ));
    commands
        .entity(ball_placeholder_entity)
        .insert(Visibility::Hidden);
}

fn handle_ball_oob(
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

fn handle_ball_collision(
    mut commands: Commands,
    mut collisions: EventReader<Collision>,
    ball: Single<(Entity, &Transform), With<Ball>>,
    paddle: Single<Entity, With<player::Paddle>>,
) {
    let (ball_entity, ball_transform) = ball.into_inner();
    let paddle_entity = paddle.into_inner();
    const ACCELERATION: f32 = 0.2;
    let mut impulse = ExternalImpulse::ZERO;
    for Collision(contact) in collisions.read() {
        if !contact.collision_started() {
            continue;
        }
        let contact_data = contact.find_deepest_contact();
        if contact.entity1 != ball_entity && contact.entity1 != paddle_entity {
            commands.entity(contact.entity1).despawn();
            if let Some(data) = contact_data {
                impulse.apply_impulse(
                    data.global_normal2(&Rotation(ball_transform.rotation)) * -ACCELERATION,
                );
            }
        }
        if contact.entity2 != ball_entity && contact.entity2 != paddle_entity {
            commands.entity(contact.entity2).despawn();
            if let Some(data) = contact_data {
                impulse.apply_impulse(
                    data.global_normal1(&Rotation(ball_transform.rotation)) * -ACCELERATION,
                );
            }
        }
    }
    if impulse != ExternalImpulse::ZERO {
        commands.entity(ball_entity).insert(impulse);
    }
}
