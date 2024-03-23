mod plugins;

use bevy::prelude::*;

use plugins::assets::AssetLoaderPlugin;
use plugins::asteroid::AsteroidPlugin;
use plugins::camera::CameraPlugin;
use plugins::collider::CollisionsPlugin;
use plugins::movement::MovementPlugin;
use plugins::spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 100.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionsPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
