use std::f32::consts::PI;

use bevy::math::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
struct AnalogDirection(f32);

impl AnalogDirection {
    fn turn_left(&self) -> AnalogDirection {
        AnalogDirection(self.0 + PI / 4.)
    }
    fn turn_right(&self) -> AnalogDirection {
        AnalogDirection(self.0 - PI / 4.)
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ant-2.png"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(5.)),
            ..default()
        },
        AnalogDirection(PI / 2.),
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut AnalogDirection, &mut Transform)>,
) {
    for (mut direction, mut transform) in &mut sprite_position {
        let dir = direction.0;

        transform.rotation = Quat::from_rotation_z(dir);

        let speed = 150.;
        let delta_x = speed * dir.cos() * time.delta_seconds();
        let delta_y = speed * dir.sin() * time.delta_seconds();

        transform.translation.x += delta_x;
        transform.translation.y += delta_y;

        *direction = AnalogDirection(dir - PI / 1000.);
    }
}
