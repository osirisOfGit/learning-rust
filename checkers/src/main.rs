use bevy::{
    app::{App, Startup}, asset::AssetServer, log::error, prelude::{Camera2dBundle, Commands, Component, Query, Res}, window::Window, DefaultPlugins
};
use tiled::Loader;

#[derive(Component)]
struct Tile {
    
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let mut tiled_loader = Loader::new();
    
    match tiled_loader.load_tmx_map("assets/checkers_board.tmx") {
        Ok(map) => {
            map.
        },
        Err(exception) => error!("Could not load map due to {}", exception)
    };

}
