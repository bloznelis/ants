use std::f32::consts::PI;
use std::time::Duration;

use bevy::math::*;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::Material2d;
use bevy::sprite::MaterialMesh2dBundle;
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
const PHEROMONE_FADE_STR: f32 = 0.001;

// todo:
// * Add food support
// * Rework pheromone detection
//      * Several kind of pheromones depening on ant's state
// * Add nest
// * Several ant states
// * Add mouse support for interactivity (like adding food) -> https://github.com/bevyengine/bevy/blob/main/examples/ui/relative_cursor_position.rs
// * Load pheromone sprite only once -> https://bevyengine.org/learn/book/getting-started/resources/

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

#[derive(Component, Clone, Copy)]
struct AntFov {}

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
                    strength: PHEROMONE_STR,
                },
            ));
            ant.pheromone_spawn_timer.reset();
        }
    }
}

fn ant_behavior_fixed(
    mut query: Query<(&mut Ant, &Transform, &Children)>,
    fovs: Query<&Transform>,
    pheromones: Query<(&Pheromone, &Transform)>,
) {
    let mut rng = rand::thread_rng();
    for (mut ant, ant_pos, children) in &mut query {
        for &child in children.iter() {
            if let Ok(fov_pos) = fovs.get(child) {
                // if collide(, a_size, b_pos, b_size)
            }
        }

        let turn_str = if !is_inside_box(ant_pos.translation.x, ant_pos.translation.y) {
            PI
        } else {
            rng.gen_range(-ANT_TURN_STR..ANT_TURN_STR)
        };

        ant.direction += turn_str;
    }
}

#[derive(Component, Clone, Copy)]
struct Food {}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    commands.spawn(Camera2dBundle::default());

    let foods: Vec<Food> = vec![Food {}; 100];
    let foods: Vec<(Food, SpriteBundle)> = foods
        .iter()
        .map(|food| {
            (
                *food,
                SpriteBundle {
                    texture: asset_server.load("sprites/food-1.png"),
                    transform: Transform::from_xyz(
                        rng.gen_range(-BOX_WIDTH..BOX_WIDTH),
                        rng.gen_range(-BOX_HEIGHT..BOX_HEIGHT),
                        1.,
                    ),
                    ..default()
                },
            )
        })
        .collect();
    commands.spawn_batch(foods);

    let ant_sprite = SpriteBundle {
        texture: asset_server.load("sprites/ant-3.png"),
        transform: Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(2.)),
        ..default()
    };

    // let triangle_mesh = MaterialMesh2dBundle {
    //                 mesh: meshes.add(shape::RegularPolygon::new(30., 3).into()).into(),
    //                 material: materials.add(ColorMaterial::from(Color::rgba(0., 0., 0., 0.3))),
    //                 transform: Transform::from_translation(Vec3::new(25., 0., 0.)).with_rotation(Quat::from_rotation_z(PI / 6. + PI)),
    //                 ..default()
    //             };
    //

    let detection_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(shape::Box::new(60., 70., 0.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgba(0., 0., 0., 0.3))),
        transform: Transform::from_translation(Vec3::new(30., 0., 0.)),
        ..default()
    };
    let fov = AntFov {};

    let ant = Ant::gen(&mut rng);
    let parent = commands.spawn((ant_sprite, ant)).id();

    let child = commands.spawn((detection_mesh, fov)).id();

    commands.entity(parent).push_children(&[child]);

    // let ants = Ant::batch_gen(ANT_AMOUNT, &mut rng);
    // let ant_entities: Vec<(SpriteBundle, Ant)> = ants
    //     .into_iter()
    //     .map(|ant| {
    //         (
    //             ant_sprite.clone(),
    //             ant,
    //         )
    //     })
    //     .collect();
    //
    // commands.spawn_batch(ant_entities);
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
