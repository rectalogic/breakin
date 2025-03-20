use avian3d::prelude::*;
use bevy::color::palettes::basic;
use bevy::prelude::*;

use crate::{app, bricks};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(OnEnter(app::AppState::Breaking), fire_ball)
        .add_systems(Update, update.run_if(in_state(app::AppState::Breaking)));
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

fn fire_ball(mut commands: Commands, ball: Single<(Entity, &Transform), With<Ball>>) {
    let (ball_entity, ball_transform) = ball.into_inner();
    commands
        .entity(ball_entity)
        .remove_parent_in_place()
        .insert((
            RigidBody::Dynamic,
            ExternalForce::new(ball_transform.rotation * Vec3::Z).with_persistence(false),
        ));
}

fn update(
    mut commands: Commands,
    ball: Single<(Entity, &Transform), With<Ball>>,
    mut next_state: ResMut<NextState<app::AppState>>,
) {
    let (ball_entity, ball_transform) = ball.into_inner();
    dbg!(ball_transform.translation); //XXX
    if false {
        commands
            .entity(ball_entity)
            .insert(RigidBody::Static)
            .remove::<ExternalForce>();
        next_state.set(app::AppState::Ready);
    }
}

//XXX run Update if in state Breaking - if we go out of bounds, set state back to Ready
//XXX in ball, we should switch back to Static and no Velocity
//XXX in player, we should reparent to paddle (already handled)
