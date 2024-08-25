use std::{
    ops::{Div, Mul},
    path::Path,
};

use bevy::{
    app::{App, Startup},
    asset::{AssetPath, AssetServer, Assets},
    color::Color,
    log::error,
    math::{Rect, Vec3},
    prelude::{
        default, Bundle, Camera2dBundle, Circle, Commands,
        DefaultPlugins, Mesh, PluginGroup, Query, Res, ResMut, Resource, WindowPlugin,
    },
    sprite::{
        BorderRect, ColorMaterial, ImageScaleMode, MaterialMesh2dBundle, Mesh2dHandle, Sprite,
        SpriteBundle, TextureSlicer,
    },
    transform::components::Transform,
    window::{Window, WindowResolution},
};
use tiled::{LayerType, Loader, Tileset};

#[derive(Resource)]
struct Board {
    // width, height
    board_size: (f32, f32),
    tile_size: (f32, f32),
    window_resolution: (f32, f32),
}

impl Board {
    fn calc_bottom_left_coord(&self) -> (f32, f32) {
        (
            (0. - self.window_resolution.0.div(2.))
                + self.tile_size.0.mul(self.calc_scale_factor().0).div(2.),
            (0. - self.window_resolution.1.div(2.))
                + self.tile_size.1.mul(self.calc_scale_factor().1).div(2.),
        )
    }

    fn calc_scale_factor(&self) -> (f32, f32) {
        (
            self.window_resolution.0.div(self.board_size.0),
            self.window_resolution.1.div(self.board_size.1),
        )
    }

    fn calc_scaled_tile_position(&self, coords: (u32, u32)) -> (f32, f32) {
        let bottom_left = self.calc_bottom_left_coord();
        let scale = self.calc_scale_factor();

        (
            bottom_left.0 + self.tile_size.0.mul(scale.0).mul(coords.0 as f32),
            bottom_left.1 + self.tile_size.1.mul(scale.1).mul(coords.1 as f32),
        )
    }
}

#[derive(Bundle)]
struct Tile {
    sprite_bundle: SpriteBundle,
    scale: ImageScaleMode,
}

impl Tile {
    fn new(
        board: &Board,
        tileset: &Tileset,
        asset_server: &Res<AssetServer>,
        tile_id: u32,
        index: (u32, u32),
    ) -> Tile {
        let scale_fac = board.calc_scale_factor();
        let scaled_tile_coords = board.calc_scaled_tile_position(index);
        Tile {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    rect: Some(Rect::new(
                        (0 + tileset.tile_width.mul(tile_id)) as f32,
                        0.,
                        tileset.tile_width.mul(tile_id + 1) as f32,
                        tileset.tile_height as f32,
                    )),
                    ..Default::default()
                },
                texture: asset_server.load(AssetPath::from_path(Path::new(
                    tileset.image.as_ref().unwrap().source.file_name().unwrap(),
                ))),
                transform: Transform {
                    translation: Vec3::new(scaled_tile_coords.0, scaled_tile_coords.1, 0.),
                    scale: Vec3::new(scale_fac.0, scale_fac.1, 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            scale: ImageScaleMode::Sliced(TextureSlicer {
                border: BorderRect::square(1.),
                center_scale_mode: bevy::sprite::SliceScaleMode::Tile { stretch_value: 0.1 },
                sides_scale_mode: bevy::sprite::SliceScaleMode::Tile { stretch_value: 0.1 },
                max_corner_scale: 0.2,
            }),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1028., 1028.),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, initialize)
        .run();
}

fn initialize(
    mut commands: Commands,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut tiled_loader = Loader::new();

    match tiled_loader.load_tmx_map("assets/checkers_board.tmx") {
        Ok(map) => {
            let resolution = &windows.single().resolution;

            let board = Board {
                board_size: (
                    map.tile_width.mul(map.width) as f32,
                    map.tile_height.mul(map.height) as f32,
                ),
                tile_size: (map.tile_width as f32, map.tile_height as f32),
                window_resolution: (resolution.width() as f32, resolution.height() as f32),
            };

            let layer = match map.get_layer(0).unwrap().layer_type() {
                LayerType::Tiles(layer) => layer,
                _ => panic!("Layer #0 is not a tile layer"),
            };

            for x in 0..layer.width().unwrap() {
                for y in 0..layer.height().unwrap() {
                    layer.get_tile(x as i32, y as i32).map(|layer_tile| {
                        layer_tile.get_tile().map(|tile| {
                            commands.spawn(Tile::new(
                                &board,
                                tile.tileset(),
                                &asset_server,
                                layer_tile.id(),
                                (x, y),
                            ));

                            if (y < 3 || y >= layer.height().unwrap() - 3) && x % 2 == y % 2 {
                                let color = if y <= layer.height().unwrap().div(2) {
                                    (255., 0., 0.)
                                } else {
                                    (255., 255., 255.)
                                };

                                let scaled_tile_coords = board.calc_scaled_tile_position((x, y));
                                commands.spawn(MaterialMesh2dBundle {
                                    mesh: Mesh2dHandle(meshes.add(Circle {
                                        radius: board.tile_size.1.div(2.),
                                    })),
                                    material: materials.add(Color::srgb(color.0, color.1, color.2)),
                                    transform: Transform {
                                        translation: Vec3::new(
                                            scaled_tile_coords.0,
                                            scaled_tile_coords.1,
                                            1.,
                                        ),
                                        scale: Vec3::new(
                                            board.calc_scale_factor().0,
                                            board.calc_scale_factor().1,
                                            1.,
                                        ),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                            }
                        });
                    });
                }
            }
            commands.insert_resource(board);
        }
        Err(exception) => error!("Could not load map due to {}", exception),
    };
}
