use bevy::prelude::*;
use rand::prelude::*;
use std::ops::Range;

use super::{
    assets::SceneAssets,
    collider::Collider,
    movement::{Acceleration, MovingObject, Velocity},
};

const ACCELERATION_SCALAR: f32 = 1.0;
const VELOCITY_SCALAR: f32 = 5.0;

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Y: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;

const ASTEROID_RADIUS: f32 = 2.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (spawn_asteroid, rotate_asteroids, handle_asteroid_collisions),
        );
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Y),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let acceleration = ACCELERATION_SCALAR * random_unit_vector();
    let velocity = VELOCITY_SCALAR * random_unit_vector();

    commands.spawn(MovingObject {
        acceleration: Acceleration::from(acceleration),
        velocity: Velocity::from(velocity),
        collider: Collider::new(ASTEROID_RADIUS),
        model: SceneBundle {
            scene: assets.asteroid.clone(),
            transform: Transform::from_translation(translation),
            ..default()
        },
    });
}

fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(time.delta_seconds() * 25.0);
    }
}

fn handle_asteroid_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}
