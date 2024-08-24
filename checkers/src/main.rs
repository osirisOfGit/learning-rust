use std::{
    ops::{Div, Mul},
    path::Path,
};

use bevy::{
    a11y::accesskit::Vec2,
    app::{App, Startup},
    asset::{AssetPath, AssetServer, Assets},
    color::Color,
    log::error,
    math::{Rect, Vec3},
    prelude::{
        default, BuildChildren, Bundle, Camera2dBundle, Circle, Commands, Component,
        DefaultPlugins, Entity, IntoSystemConfigs, Mesh, PluginGroup, Query, Res, ResMut, Resource,
        WindowPlugin, With,
    },
    render::settings::Backends,
    sprite::{
        BorderRect, ColorMaterial, ImageScaleMode, MaterialMesh2dBundle, Sprite, SpriteBundle,
        TextureSlicer,
    },
    transform::components::Transform,
    window::{Window, WindowResolution},
};
use tiled::{LayerType, Loader, TileLayer, Tileset};

#[derive(Component)]
struct PlayableSquare;

#[derive(Resource)]
struct Board {
    // width, height
    board_size: (f32, f32),
    tile_size: (f32, f32),
    window_resolution: (f32, f32),
}

impl Board {
    fn calc_bottom_left_coord(&self) -> (f32, f32) {
        return (
            (0. - self.window_resolution.0.div(2.)) + self.tile_size.0.mul(self.calc_scale_factor().0).div(2.),
            (0. - self.window_resolution.1.div(2.)) + self.tile_size.1.mul(self.calc_scale_factor().1).div(2.),
        );
    }

    fn calc_scale_factor(&self) -> (f32, f32) {
        return (
            self.window_resolution
                .0
                .div(self.board_size.0),
            self.window_resolution
                .1
                .div(self.board_size.1),
        );
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
                    translation: Vec3::new(
                        board.calc_bottom_left_coord().0 + board.tile_size.0.mul(scale_fac.0).mul(index.0 as f32),
                        board.calc_bottom_left_coord().1 + board.tile_size.1.mul(scale_fac.1).mul(index.1 as f32),
                        0.,
                    ),
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

fn initialize(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
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
                            let mut cmd = commands.spawn(Tile::new(
                                &board,
                                tile.tileset(),
                                &asset_server,
                                layer_tile.id(),
                                (x, y),
                            ));

                            if (x % 2 == 0 && y % 2 == 0) || (x % 2 == 1 && y % 2 == 1) {
                                cmd.insert(PlayableSquare {});
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

fn spawn_pieces(
    mut commands: Commands,
    playable_squares: Query<(Entity, &Transform), With<PlayableSquare>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity_id, transform) in playable_squares.iter() {
        let new_piece = commands
            .spawn(MaterialMesh2dBundle {
                mesh: bevy::sprite::Mesh2dHandle(meshes.add(Circle { radius: 4. })),
                material: materials.add(Color::srgb(255., 255., 255.)),
                ..Default::default()
            })
            .id();

        commands.entity(entity_id).add_child(new_piece);
    }
}
