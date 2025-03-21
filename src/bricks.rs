use avian3d::prelude::*;
use bevy::color::palettes::basic;
use bevy::prelude::*;

use crate::GameLayer;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

const CUBE_HALF_SIZE: usize = 5;
pub(super) const CUBE_SIZE: usize = CUBE_HALF_SIZE * 2 + 1;
pub(super) const INNER_CUBE_SIZE: f32 = 1.0;

#[derive(Component)]
struct Brick;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Color of each "shell"
    let colors: [Handle<StandardMaterial>; CUBE_HALF_SIZE] = [
        materials.add(Color::from(basic::AQUA)),
        materials.add(Color::from(basic::LIME)),
        materials.add(Color::from(basic::BLUE)),
        materials.add(Color::from(basic::FUCHSIA)),
        materials.add(Color::from(basic::GRAY)),
        // materials.add(Color::from(basic::GREEN)),
        // materials.add(Color::from(basic::BLACK)),
        // materials.add(Color::from(basic::MAROON)),
        // materials.add(Color::from(basic::NAVY)),
        // materials.add(Color::from(basic::OLIVE)),
        // materials.add(Color::from(basic::PURPLE)),
        // XXX materials.add(Color::from(basic::RED)),
        // materials.add(Color::from(basic::SILVER)),
        // materials.add(Color::from(basic::TEAL)),
        // materials.add(Color::from(basic::WHITE)),
        // materials.add(Color::from(basic::YELLOW)),
    ];
    const SIDE: f32 = INNER_CUBE_SIZE - 0.01;
    let size = -(CUBE_HALF_SIZE as i32)..=CUBE_HALF_SIZE as i32;
    let mesh = meshes.add(Cuboid::new(SIDE, SIDE, SIDE));
    for x in size.clone() {
        for y in size.clone() {
            for z in size.clone() {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                let shell_level = CUBE_HALF_SIZE as i32
                    - [
                        x.abs(), // Distance from x=0 plane
                        y.abs(), // Distance from y=0 plane
                        z.abs(), // Distance from z=0 plane
                    ]
                    .into_iter()
                    .max()
                    .expect("Not empty");
                let material = colors[(shell_level as usize) % colors.len()].clone();
                commands.spawn((
                    Brick,
                    RigidBody::Static,
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material),
                    Transform::from_xyz(x as f32, y as f32, z as f32),
                    Collider::cuboid(SIDE, SIDE, SIDE),
                    Restitution::new(1.0),
                    CollisionLayers::new(GameLayer::Brick, [GameLayer::Ball]),
                ));
            }
        }
    }
}
