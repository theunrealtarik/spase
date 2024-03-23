use bevy::prelude::*;

use super::movement::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position)
            .add_systems(Update, print_velocity);
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in &query {
        info!("ENTITY {:#?} VELOCITY {:#?}", entity, transform.translation);
    }
}

fn print_velocity(query: Query<(Entity, &Velocity)>) {
    for (entity, velocity) in &query {
        info!("ENTITY {:#?} VELOCITY {:#?}", entity, velocity);
    }
}
