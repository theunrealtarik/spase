use bevy::prelude::*;

use super::{assets::SceneAssets, collider::Collider, movement::*};

const SPACESHIP_STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);

const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_YAW_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 5.0;

const MISSILE_SPEED: f32 = 45.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 1.0;

#[derive(Component)]
pub struct Spaceship;

pub struct SpaceshipPlugin;
impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, spaceship_controller)
            .add_plugins(MissilePlugin);
    }
}

fn spawn_spaceship(mut commands: Commands, assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObject {
            acceleration: Acceleration::from(Vec3::ZERO),
            velocity: Velocity::from(SPACESHIP_STARTING_VELOCITY),
            collider: Collider::new(SPACESHIP_RADIUS),
            model: SceneBundle {
                scene: assets.spaceship.clone(),
                transform: Transform::from_translation(SPACESHIP_STARTING_TRANSLATION),
                ..SceneBundle::default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_controller(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut yaw = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard.pressed(KeyCode::KeyD) {
        yaw = -SPACESHIP_YAW_SPEED * time.delta_seconds();
    } else if keyboard.pressed(KeyCode::KeyA) {
        yaw = SPACESHIP_YAW_SPEED * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;
    }

    transform.rotate_y(yaw);
    transform.rotate_local_z(roll);

    *velocity.value_mut() = -transform.forward() * movement;
}

pub struct MissilePlugin;

impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_missile);
    }
}

#[derive(Component)]
pub struct SpaceshipMissile;

fn spawn_missile(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    assets: Res<SceneAssets>,
) {
    let transform = query.single();

    if keyboard.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObject {
                velocity: Velocity::from(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::from(Vec3::ZERO),
                collider: Collider::new(MISSILE_RADIUS),
                model: SceneBundle {
                    scene: assets.missile.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}
