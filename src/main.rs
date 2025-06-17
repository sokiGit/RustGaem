use bevy::prelude::*;
use bevy_rapier3d::math::Vect;
use bevy_rapier3d::plugin::{PhysicsSet, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::Collider;
use bevy_rapier3d::rapier::dynamics::RigidBodyType;
use bevy_rapier3d::rapier::prelude::DebugRenderObject::RigidBody;
use bevy_rapier3d::rapier::prelude::{ColliderBuilder, PhysicsPipeline, RigidBodyBuilder};
/* Components */
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

/* Systems */
fn setup(
    mut commands: Commands,                             // Commands are used to spawn/despawn entities and add/remove components
    mut meshes: ResMut<Assets<Mesh>>,                   // Resources for managing meshes (shapes)
    mut materials: ResMut<Assets<StandardMaterial>>,    // Resources for managing materials (colors/textures)
) {
    // Spawn baseplate
    commands
        .spawn((
            Mesh3d::from(meshes.add(Plane3d::new(Vec3::new(0.0, 1.0,0.0), Vec2::new(128.0, 128.0)))),
            MeshMaterial3d::from(materials.add(StandardMaterial::from(Color::WHITE))),
            Transform::from_xyz(0.0, -1.0, 0.0),
        ));
        

    // Spawn player
        // An entity is created by calling `command.spawn()`.
        // We add a `Player` marker component so we can identify it later.
        // We also add a PbrBundle, which adds common components for 3D rendering:
            // - Mesh:      Defines the shape (e.g., cube)
            // - Material:  Defines appearance (e.g., red color)
            // - Transform: Defines the position, rotation, and scale in 3D space
    commands.spawn((
        Player,
        Mesh3d::from(meshes.add(Capsule3d::new(1.0, 2.0))),
        MeshMaterial3d::from(materials.add(StandardMaterial::from(Color::WHITE))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Velocity(Vec3::ZERO),
    ));
    // Spawn a light source
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            color: Color::Hsla(Hsla::new(1.0, 1.0, 0.5, 1.0).into()),
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Add a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(-2.5, 4.5, 9.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn player_movement(
    mut player_query: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    const WALK_SPEED: f32 = 10.0;
    const JUMP_FORCE: f32 = 5.0;
    const GRAVITATIONAL_CONSTANT: f32 = 9.80665;
    
    for mut velocity in player_query.iter_mut() {
        let move_forward = keyboard_input.pressed(KeyCode::KeyW);
        let move_backward = keyboard_input.pressed(KeyCode::KeyS);
        let move_left = keyboard_input.pressed(KeyCode::KeyA);
        let move_right = keyboard_input.pressed(KeyCode::KeyD);

        // Move Forward/Backward
        if move_forward && !move_backward {
            velocity.0.z = -WALK_SPEED;
        } else if move_backward && !move_forward {
            velocity.0.z = WALK_SPEED;
        } else {
            velocity.0.z = 0.0;
        }

        // Move Left/Right
        if move_left && !move_right {
            velocity.0.x = -WALK_SPEED;
        } else if move_right && !move_left {
            velocity.0.x = WALK_SPEED;
        } else {
            velocity.0.x = 0.0;
        }

        // Jump
        if keyboard_input.just_pressed(KeyCode::Space) { velocity.0.y = JUMP_FORCE.max(velocity.0.y + JUMP_FORCE); }
        
        velocity.0.y -=  GRAVITATIONAL_CONSTANT * time.delta_secs();
    }
}

fn velocity_update(
    time: Res<Time>,
    mut velocity_query: Query<(&mut Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in velocity_query.iter_mut() {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

/* Main */
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, velocity_update)
        .run();
}
