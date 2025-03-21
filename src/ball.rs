use avian3d::prelude::*;
use bevy::color::palettes::basic;
use bevy::prelude::*;

use crate::{app, bricks, player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(OnEnter(app::AppState::Breaking), fire_ball)
        .add_systems(PostUpdate, update.run_if(in_state(app::AppState::Breaking)));
}

pub(super) const BALL_RADIUS: f32 = bricks::INNER_CUBE_SIZE / 4.0;

#[derive(Component)]
#[require(Transform)]
pub(super) struct Ball;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<app::AppState>>,
) {
    commands.spawn((
        Ball,
        RigidBody::Static,
        Restitution::new(0.8),
        Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh().ico(4).unwrap())),
        MeshMaterial3d(materials.add(Color::from(basic::RED))),
        Transform::default(),
        Collider::sphere(BALL_RADIUS),
    ));
    next_state.set(app::AppState::Ready);
}

fn fire_ball(mut commands: Commands, ball: Single<(Entity, &GlobalTransform), With<Ball>>) {
    let (ball_entity, ball_transform) = ball.into_inner();
    commands
        .entity(ball_entity)
        // This happens in PostUpdae, so we have to use GlobalTransform below
        .remove_parent_in_place()
        .insert((
            RigidBody::Dynamic,
            ExternalImpulse::new(ball_transform.forward() * 0.5).with_persistence(false),
        ));
}

fn update(
    mut commands: Commands,
    ball: Single<(Entity, &Transform), With<Ball>>,
    mut next_state: ResMut<NextState<app::AppState>>,
) {
    let (ball_entity, ball_transform) = ball.into_inner();
    if ball_transform.translation.distance(Vec3::ZERO) > player::PLAYFIELD_RADIUS {
        commands.entity(ball_entity).insert(RigidBody::Static);
        next_state.set(app::AppState::Ready);
    }
}
