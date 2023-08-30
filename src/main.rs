use std::f32::consts::PI;

use bevy::math::*;
use bevy::prelude::*;
use rand::rngs::ThreadRng;
use rand::Rng;

const TIME_STEP: f32 = 0.1; //secs

const BOX_WIDTH: f32 = 1000.;
const BOX_HEIGHT: f32 = BOX_WIDTH / 2.;

const ANT_SPEED_MAX: f32 = 200.;
const ANT_TURN_STR: f32 = PI / 10.;
const ANT_AMOUNT: usize = 10000;

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
            speed: rng.gen_range((3.)..ANT_SPEED_MAX),
        }
    }

    fn batch_gen(amount: usize, rng: &mut ThreadRng) -> Vec<Ant> {
        let mut vec: Vec<Ant> = vec![];
        vec.resize_with(amount, || Ant::gen(rng));
        vec
    }
}

fn ant_behavior(mut query: Query<(&mut Ant, &Transform)>) {
    for (mut ant, transorm) in &mut query {
        let is_positive: bool = rand::random();
        let turn_strength = if !is_inside_box(transorm.translation.x, transorm.translation.y) {
            PI
        } else {
            ANT_TURN_STR
        };

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
    let ants = Ant::batch_gen(ANT_AMOUNT, &mut rng);
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

        transform.translation.x = clamp(-BOX_WIDTH, BOX_WIDTH, transform.translation.x + delta_x);
        transform.translation.y = clamp(-BOX_HEIGHT, BOX_HEIGHT, transform.translation.y + delta_y);
    }
}

fn clamp(min: f32, max: f32, value: f32) -> f32 {
    min.max(value).min(max)
}

fn is_inside_box(x: f32, y: f32) -> bool {
    x > -BOX_WIDTH && x < BOX_WIDTH && y > -BOX_HEIGHT && y < BOX_HEIGHT
}
