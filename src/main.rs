use std::f32::consts::PI;
use std::time::Duration;

use bevy::math::*;
use bevy::prelude::*;
use rand::rngs::ThreadRng;
use rand::Rng;

const TIME_STEP: f32 = 0.1; //secs

const BOX_WIDTH: f32 = 1000.;
const BOX_HEIGHT: f32 = BOX_WIDTH / 2.;

const ANT_AMOUNT: usize = 2000;
const ANT_SPEED_MIN: f32 = 30.;
const ANT_SPEED_MAX: f32 = 100.;
const ANT_TURN_STR: f32 = PI / 10.;

const PHEROMONE_SPAWN_RATE_MS: u64 = 100;
const PHEROMONE_STR: f32 = 1.;
const PHEROMONE_FADE_STR: f32 = 0.005;

// todo:
// * Add food support
// * Rework pheromone detection
//      * Several kind of pheromones depening on ant's state
// * Add nest
// * Several ant states
// * Add mouse support for interactivity (like adding food) -> https://github.com/bevyengine/bevy/blob/main/examples/ui/relative_cursor_position.rs

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.25, 0.3, 0.25)))
        .add_plugins(DefaultPlugins)
        // .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, ant_behavior_fixed)
        .add_systems(Update, (sprite_movement, ant_behavior, pheromone_behavior))
        .run();
}

#[derive(Component)]
struct Ant {
    direction: f32,
    speed: f32,
    pheromone_spawn_timer: Timer,
}

impl Ant {
    fn gen(rng: &mut ThreadRng) -> Self {
        let mut timer = Timer::new(
            Duration::from_millis(PHEROMONE_SPAWN_RATE_MS),
            TimerMode::Once,
        );
        timer.set_elapsed(Duration::from_millis(
            rng.gen_range(0..PHEROMONE_SPAWN_RATE_MS),
        ));
        Ant {
            direction: rng.gen_range((0.)..(PI * 2.)),
            speed: rng.gen_range(ANT_SPEED_MIN..ANT_SPEED_MAX),
            pheromone_spawn_timer: timer,
        }
    }

    fn batch_gen(amount: usize, rng: &mut ThreadRng) -> Vec<Ant> {
        let mut vec: Vec<Ant> = vec![];
        vec.resize_with(amount, || Ant::gen(rng));
        vec
    }
}

#[derive(Component)]
struct Pheromone {
    owner: Entity,
    strength: f32,
}

fn pheromone_behavior(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut Pheromone)>,
) {
    for (entity, mut sprite, mut pheromone) in &mut query {
        sprite.color.set_a(pheromone.strength);
        pheromone.strength -= PHEROMONE_FADE_STR;
        if pheromone.strength <= 0. {
            commands.entity(entity).despawn();
            sprite.color = Color::rgb(0., 0., 0.);
        }
    }
}

fn ant_behavior(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut Ant, &Transform)>,
) {
    for (entity, mut ant, transorm) in &mut query {
        ant.pheromone_spawn_timer.tick(time.delta());

        let mut translation = transorm.translation.clone();
        translation.z -= 1.;
        if ant.pheromone_spawn_timer.finished() {
            //this is very stupid, should load up the sprite only one time
            let pheromone_sprite = SpriteBundle {
                texture: asset_server.load("sprites/pheromone-2.png"),
                transform: transorm
                    .with_translation(translation)
                    .clone()
                    .with_scale(Vec3::splat(1.2)),
                ..default()
            };

            commands.spawn((
                pheromone_sprite,
                Pheromone {
                    owner: entity,
                    strength: PHEROMONE_STR,
                },
            ));
            ant.pheromone_spawn_timer.reset();
        }
    }
}

fn ant_behavior_fixed(
    mut query: Query<(Entity, &mut Ant, &Transform)>,
    pheromones: Query<(&Pheromone, &Transform)>,
) {
    let mut rng = rand::thread_rng();
    for (entity, mut ant, ant_pos) in &mut query {
        // let follow_pheromone: bool = rand::random();
        let follow_pheromone: bool = false;
        let turn_str = if !is_inside_box(ant_pos.translation.x, ant_pos.translation.y) {
            PI
        } else if follow_pheromone {
            let max_pheromone_dist = 50.;
            let mut closest_pheromone_pos: Option<&Transform> = None;
            let mut closest_distance = 1000000.0; //inf

            for (pheromone, pos) in pheromones.iter() {
                let dist = ((pos.translation.x - ant_pos.translation.x).powi(2)
                    + (pos.translation.y - ant_pos.translation.y).powi(2))
                .sqrt();
                if closest_distance > dist
                    && max_pheromone_dist > dist
                    && entity.index() != pheromone.owner.index()
                {
                    closest_distance = dist;
                    closest_pheromone_pos = Some(pos);
                }
            }
            match closest_pheromone_pos {
                Some(pheromone_pos) => {
                    let dx = pheromone_pos.translation.x - ant_pos.translation.x;
                    let dy = pheromone_pos.translation.y - ant_pos.translation.y;
                    let target_angle = dy.atan2(dx);

                    target_angle / 2.
                }
                None => rng.gen_range(-ANT_TURN_STR..ANT_TURN_STR),
            }
        } else {
            rng.gen_range(-ANT_TURN_STR..ANT_TURN_STR)
        };

        ant.direction += turn_str;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let ant_sprite = SpriteBundle {
        texture: asset_server.load("sprites/ant-3.png"),
        transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(2.)),
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
