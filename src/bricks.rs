use avian3d::prelude::*;
use bevy::color::palettes::basic;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const HALF_SIZE: usize = 5;
    // Color of each "shell"
    let colors: [Handle<StandardMaterial>; HALF_SIZE] = [
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
        // materials.add(Color::from(basic::RED)),
        // materials.add(Color::from(basic::SILVER)),
        // materials.add(Color::from(basic::TEAL)),
        // materials.add(Color::from(basic::WHITE)),
        // materials.add(Color::from(basic::YELLOW)),
    ];
    const SIDE: f32 = 0.99;
    let size = -(HALF_SIZE as i32)..=HALF_SIZE as i32;
    let mesh = meshes.add(Cuboid::new(SIDE, SIDE, SIDE));
    for x in size.clone() {
        for y in size.clone() {
            for z in size.clone() {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                let shell_level = [
                    x + 1,                  // distance from left face
                    y + 1,                  // distance from bottom face
                    z + 1,                  // distance from back face
                    (HALF_SIZE as i32) - x, // distance from right face
                    (HALF_SIZE as i32) - y, // distance from top face
                    (HALF_SIZE as i32) - z, // distance from front face
                ]
                .into_iter()
                .min()
                .expect("Not empty")
                    - 1;
                let material = colors[(shell_level as usize) % colors.len()].clone();
                commands.spawn((
                    RigidBody::Static,
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material),
                    Transform::from_xyz(x as f32, y as f32, z as f32),
                    Collider::cuboid(SIDE, SIDE, SIDE),
                ));
            }
        }
    }
}
