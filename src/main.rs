use std::f32::consts::PI;

use bevy::math::*;
use bevy::prelude::*;
use rand::rngs::ThreadRng;
use rand::Rng;

const TIME_STEP: f32 = 0.1; //secs

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.25, 0.3, 0.25)))
        .add_plugins(DefaultPlugins)
        // .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
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

impl Ant {
    fn gen(rng: &mut ThreadRng) -> Self {
        Ant {
            direction: rng.gen_range((0.)..(PI * 2.)),
            speed: rng.gen_range((3.)..(75.)),
        }
    }

    fn batch_gen(amount: usize, rng: &mut ThreadRng) -> Vec<Ant> {
        let mut vec: Vec<Ant> = vec![];
        vec.resize_with(amount, || Ant::gen(rng));
        vec
    }
}

fn ant_behavior(mut query: Query<&mut Ant>) {
    for mut ant in &mut query {
        let is_positive: bool = rand::random();
        let turn_strength = PI / 10.;

        if is_positive {
            ant.direction += turn_strength;
        } else {
            ant.direction -= turn_strength;
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let ant_sprite = SpriteBundle {
        texture: asset_server.load("sprites/ant-3.png"),
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(2.)),
        ..default()
    };
    let ants = Ant::batch_gen(15000, &mut rng);
    let ant_entities: Vec<(SpriteBundle, Ant)> = ants
        .into_iter()
        .map(|ant| (ant_sprite.clone(), ant))
        .collect();

    commands.spawn(Camera2dBundle::default());
    commands.spawn_batch(ant_entities);
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&Ant, &mut Transform)>) {
    for (ant, mut transform) in &mut sprite_position {
        let dir = ant.direction;

        transform.rotation = Quat::from_rotation_z(dir);

        let delta_x = ant.speed * dir.cos() * time.delta_seconds();
        let delta_y = ant.speed * dir.sin() * time.delta_seconds();

        transform.translation.x += delta_x;
        transform.translation.y += delta_y;
    }
}
