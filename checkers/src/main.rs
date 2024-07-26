use std::{
    ops::{Div, Mul},
    path::Path,
};

use bevy::{
    app::{App, Startup},
    asset::{AssetPath, AssetServer},
    log::error,
    math::{Rect, Vec3},
    prelude::{
        default, Camera2dBundle, Commands, DefaultPlugins, PluginGroup, Query, Res, WindowPlugin,
    },
    sprite::{BorderRect, ImageScaleMode, Sprite, SpriteBundle, TextureSlicer},
    transform::components::Transform,
    window::{Window, WindowResolution},
};
use tiled::{LayerType, Loader};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1028., 1028.),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let mut tiled_loader = Loader::new();

    match tiled_loader.load_tmx_map("assets/checkers_board.tmx") {
        Ok(map) => {
            let resolution = &windows.single().resolution;
            let scaled = (resolution.width().div(map.tile_width.mul(map.width) as f32), resolution.height().div(map.tile_height.mul(map.height) as f32));
            let tile_sizes = (map.tile_width as f32 * scaled.0, map.tile_height as f32 * scaled.0);

            let layer = match map.get_layer(0).unwrap().layer_type() {
                LayerType::Tiles(layer) => layer,
                _ => panic!("Layer #0 is not a tile layer"),
            };

            let top_left_coord = (
                (0. - resolution.width().div(2.)) + tile_sizes.0.div(2.),
                (0. - resolution.height().div(2.)) + tile_sizes.1.div(2.),
            );

            for x in 0..layer.width().unwrap() {
                for y in 0..layer.height().unwrap() {
                    layer.get_tile(x as i32, y as i32).map(|layer_tile| {
                        layer_tile.get_tile().map(|tile| {
                            let mut cmd = commands.spawn(SpriteBundle {
                                sprite: Sprite {
                                    rect: Some(Rect::new(
                                        (0 + tile.tileset().tile_width.mul(layer_tile.id())) as f32,
                                        0.,
                                        tile.tileset().tile_width.mul(layer_tile.id() + 1) as f32,
                                        tile.tileset().tile_height as f32,
                                    )),
                                    ..Default::default()
                                },
                                texture: asset_server.load(AssetPath::from_path(Path::new(
                                    tile.tileset()
                                        .image
                                        .as_ref()
                                        .unwrap()
                                        .source
                                        .file_name()
                                        .unwrap(),
                                ))),
                                transform: Transform {
                                    translation: Vec3::new(
                                        top_left_coord.0 + tile_sizes.0.mul(x as f32),
                                        top_left_coord.1 + tile_sizes.1.mul(y as f32),
                                        0.,
                                    ),
                                    scale: Vec3::new(scaled.0, scaled.1, 0.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            });

                            cmd.insert(ImageScaleMode::Sliced(TextureSlicer {
                                border: BorderRect::square(1.),
                                center_scale_mode: bevy::sprite::SliceScaleMode::Tile { stretch_value: 0.1 },
                                sides_scale_mode: bevy::sprite::SliceScaleMode::Tile { stretch_value: 0.1 },
                                max_corner_scale: 0.2
                            }));
                        });
                    });
                }
            }
        }
        Err(exception) => error!("Could not load map due to {}", exception),
    };
}
