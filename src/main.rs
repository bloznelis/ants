use std::f32::consts::PI;

use bevy::math::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, ant_behavior)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
struct Ant {
    direction: f32,
    speed: f32,
}

fn ant_behavior(mut query: Query<&mut Ant>) {
    let mut ant = query.single_mut();

    ant.direction += - PI / 1000.;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ant-2.png"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(5.)),
            ..default()
        },
        Ant{direction: PI / 2., speed: 150.},
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&Ant, &mut Transform)>,
) {
    for (ant, mut transform) in &mut sprite_position {
        let dir = ant.direction;

        transform.rotation = Quat::from_rotation_z(dir);

        let speed = ant.speed;
        let delta_x = speed * dir.cos() * time.delta_seconds();
        let delta_y = speed * dir.sin() * time.delta_seconds();

        transform.translation.x += delta_x;
        transform.translation.y += delta_y;
    }
}
