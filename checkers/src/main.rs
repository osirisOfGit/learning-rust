use std::path::Path;

use bevy::{
    app::{App, Startup},
    asset::{AssetPath, AssetServer},
    log::error,
    math::Rect,
    prelude::{Camera2dBundle, Commands, Query, Res},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    window::Window,
    DefaultPlugins,
};
use tiled::{LayerType, Loader};

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
            let layer = match map.get_layer(0).unwrap().layer_type() {
                LayerType::Tiles(layer) => layer,
                _ => panic!("Layer #0 is not a tile layer"),
            };

            for x in 0..layer.width().unwrap() {
                for y in 0..layer.height().unwrap() {
                    layer.get_tile(x as i32, y as i32).map(|layer_tile| {
                        layer_tile.get_tile().map(|tile| {
                            let image_path =
                                tile.tileset().image.as_ref().unwrap().source.file_name().unwrap();
                            commands.spawn(SpriteBundle {
                                sprite: Sprite {
                                    rect: Some(Rect::new(
                                        (layer_tile.id() * layer_tile.get_tileset().tile_width) as f32,
                                        0.,
                                        ((layer_tile.id() + 1) * layer_tile.get_tileset().tile_width) as f32,
                                        layer_tile.get_tileset().tile_height as f32,
                                    )),
                                    ..Default::default()
                                },
                                texture: asset_server.load(AssetPath::from_path(Path::new(image_path))),
                                transform: Transform::from_xyz(
                                    (x * map.tile_width) as f32,
                                    (y * map.tile_height) as f32,
                                    0.,
                                ),
                                ..Default::default()
                            })
                        });
                    });
                }
            }
        }
        Err(exception) => error!("Could not load map due to {}", exception),
    };
}
