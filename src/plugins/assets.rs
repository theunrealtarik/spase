use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missile: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, assets: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid: assets.load("models/Asteroid.glb#Scene0"),
        spaceship: assets.load("models/Spaceship.glb#Scene0"),
        missile: assets.load("models/Missile.glb#Scene0"),
    }
}
