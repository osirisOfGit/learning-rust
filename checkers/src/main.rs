use std::{
    ops::{Div, Mul},
    path::Path,
};

use bevy::{
    app::{App, Startup, Update},
    asset::{AssetPath, AssetServer, Assets},
    color::Color,
    input::common_conditions::input_just_pressed,
    log::error,
    math::{Rect, Vec2, Vec3},
    prelude::{
        default, Bundle, Camera, Camera2dBundle, Circle, Commands, Component, DefaultPlugins, Entity, GlobalTransform, IntoSystemConfigs, Mesh, MouseButton, PluginGroup, Query, Res, ResMut, Resource, WindowPlugin, With, Without
    },
    sprite::{
        BorderRect, ColorMaterial, ImageScaleMode, MaterialMesh2dBundle, Mesh2dHandle, Sprite,
        SpriteBundle, TextureSlicer,
    },
    transform::components::Transform,
    window::{PrimaryWindow, Window, WindowResolution},
};
use tiled::{LayerType, Loader, Tileset};

/// The projected 2D world coordinates of the cursor (if it's within primary window bounds).
#[derive(Resource)]
struct CursorWorldPos(Option<Vec2>);
 
#[derive(Component)]
struct ClickedPiece();

// #[derive(Resource)]
// struct SelectedPiece(Option<&)

#[derive(Resource)]
struct Board {
    // width, height
    board_size: (f32, f32),
    tile_size: (f32, f32),
    window_resolution: (f32, f32),
}

impl Board {
    fn calc_scale_factor(&self) -> Vec2 {
        Vec2::new(
            self.window_resolution.0.div(self.board_size.0),
            self.window_resolution.1.div(self.board_size.1),
        )
    }

    fn calc_scaled_tile_size(&self) -> (f32, f32) {
        let scale = self.calc_scale_factor();

        (self.tile_size.0.mul(scale.x), self.tile_size.1.mul(scale.y))
    }

    fn calc_bottom_left_coord(&self) -> Vec2 {
        let scaled_tile_size = self.calc_scaled_tile_size();
        Vec2::new(
            (0. - self.window_resolution.0.div(2.)) + scaled_tile_size.0.div(2.),
            (0. - self.window_resolution.1.div(2.)) + scaled_tile_size.1.div(2.),
        )
    }

    fn calc_scaled_tile_position(&self, coords: (u32, u32)) -> Vec2 {
        let bottom_left = self.calc_bottom_left_coord();
        let scaled_tile_size = self.calc_scaled_tile_size();


        Vec2::new(
            bottom_left.x + scaled_tile_size.0.mul(coords.0 as f32),
            bottom_left.y + scaled_tile_size.1.mul(coords.1 as f32),
        )
    }
}

#[derive(Component)]
struct Tile;

#[derive(Bundle)]
struct TileBundle {
    sprite_bundle: SpriteBundle,
    scale: ImageScaleMode,
    tile: Tile
}

impl TileBundle {
    fn new(
        board: &Board,
        tileset: &Tileset,
        asset_server: &Res<AssetServer>,
        tile_id: u32,
        scaled_tile_coords: Vec2,
    ) -> TileBundle {
        TileBundle {
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
                    translation: scaled_tile_coords.extend(0.),
                    scale: board.calc_scale_factor().extend(0.),
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
            tile: Tile
        }
    }
}

#[derive(Component)]
struct Piece;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1028., 1028.),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(CursorWorldPos(None))
        .add_systems(Startup, initialize)
        .add_systems(
            Update,
            (
                get_cursor_world_pos,
                click_piece.run_if(input_just_pressed(MouseButton::Left)),
            )
                .chain(),
        )
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
                            let scaled_tile_coords = board.calc_scaled_tile_position((x, y));

                            commands.spawn(TileBundle::new(
                                &board,
                                tile.tileset(),
                                &asset_server,
                                layer_tile.id(),
                                scaled_tile_coords,
                            ));

                            if (y < 3 || y >= layer.height().unwrap() - 3) && x % 2 == y % 2 {
                                let color = if y <= layer.height().unwrap().div(2) {
                                    (255., 0., 0.)
                                } else {
                                    (255., 255., 255.)
                                };

                                commands.spawn((
                                    MaterialMesh2dBundle {
                                        mesh: Mesh2dHandle(meshes.add(Circle {
                                            radius: board.tile_size.1.div(2.),
                                        })),
                                        material: materials
                                            .add(Color::srgb(color.0, color.1, color.2)),
                                        transform: Transform {
                                            translation: scaled_tile_coords.extend(1.),
                                            scale: board.calc_scale_factor().extend(0.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    Piece {},
                                ));
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

fn get_cursor_world_pos(
    mut cursor_world_pos: ResMut<CursorWorldPos>,
    q_primary_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let primary_window = q_primary_window.single();
    let (main_camera, main_camera_transform) = q_camera.single();

    cursor_world_pos.0 = primary_window
        .cursor_position()
        .and_then(|cursor_pos| main_camera.viewport_to_world_2d(main_camera_transform, cursor_pos));
}

fn click_piece(
    mut commands: Commands,
    cursor_world_pos: Res<CursorWorldPos>,
    board: Res<Board>,
    pieces: Query<(Entity, &Transform), With<Piece>>,
) {
    // If the cursor is not within the primary window skip this system
    let Some(cursor_world_pos) = cursor_world_pos.0 else {
        return;
    };

    let clicked_piece = pieces
        .iter()
        .find(|transform| {
            transform.1.translation.truncate().distance(cursor_world_pos)
                < board.calc_scaled_tile_size().0
        });

    if clicked_piece.is_some() {
        commands.get_entity(clicked_piece.unwrap().0).unwrap().insert(ClickedPiece());
    };
}

fn highlight_valid_moves(
    mut commands: Commands,
    tiles: Query<&Transform, With<Tile>>,
    pieces: Query<&Transform, (With<Piece>, Without<ClickedPiece>)>,
    clicked_piece: Query<&Transform, With<ClickedPiece>>
) {

}
