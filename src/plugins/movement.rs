use bevy::prelude::*;

use super::collider::Collider;

#[derive(Component)]
pub struct Acceleration {
    value: Vec3,
}

#[derive(Component, Debug)]
pub struct Velocity {
    value: Vec3,
}

pub trait Vector {
    fn new(x: f32, y: f32, z: f32) -> Self;
    fn value(&self) -> Vec3;
    fn value_mut(&mut self) -> &mut Vec3;

    fn x(&self) -> f32 {
        let value = self.value();
        value.x
    }

    fn y(&self) -> f32 {
        let value = self.value();
        value.y
    }

    fn z(&self) -> f32 {
        let value = self.value();
        value.z
    }
}

impl From<Vec3> for Velocity {
    fn from(value: Vec3) -> Self {
        Self { value }
    }
}

impl Vector for Velocity {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }

    fn value(&self) -> Vec3 {
        self.value
    }

    fn value_mut(&mut self) -> &mut Vec3 {
        &mut self.value
    }
}

impl From<Vec3> for Acceleration {
    fn from(value: Vec3) -> Self {
        Self { value }
    }
}

impl Vector for Acceleration {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }

    fn value_mut(&mut self) -> &mut Vec3 {
        &mut self.value
    }

    fn value(&self) -> Vec3 {
        self.value
    }
}

#[derive(Bundle)]
pub struct MovingObject {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
    pub model: SceneBundle,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position)
            .add_systems(Update, update_velocity);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value() * time.delta_seconds();
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        *velocity.value_mut() += acceleration.value() * time.delta_seconds();
    }
}
