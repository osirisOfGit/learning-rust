use std::{
    ops::{Div, Mul},
    path::Path,
};

use bevy::{
    app::{App, Startup, Update},
    asset::{AssetServer, Assets},
    color::Color,
    ecs::system::Single,
    input::common_conditions::input_just_pressed,
    log::error,
    math::Vec2,
    mesh::Mesh2d,
    prelude::{
        Camera, Camera2d, Circle, Commands, Component, DefaultPlugins, Entity, GlobalTransform,
        IntoScheduleConfigs, Mesh, MouseButton, PluginGroup, Query, Res, ResMut, Resource,
        WindowPlugin, With, Without, default,
    },
    sprite::Sprite,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
    window::{PrimaryWindow, Window, WindowResolution},
};
use tiled::{LayerType, Loader, Map};

/// The projected 2D world coordinates of the cursor (if it's within primary window bounds).
#[derive(Resource)]
struct CursorWorldPos(Option<Vec2>);

#[derive(Resource, Clone)]
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

    fn new(map: &Map, resolution: &WindowResolution) -> Board {
        Board {
            board_size: (
                map.tile_width.mul(map.width) as f32,
                map.tile_height.mul(map.height) as f32,
            ),
            tile_size: (map.tile_width as f32, map.tile_height as f32),
            window_resolution: (resolution.width() as f32, resolution.height() as f32),
        }
    }
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct PlayableTile;

#[derive(PartialEq)]
enum PlayerColor {
    RED,
    BLACK,
}

#[derive(Component)]
struct Piece(PlayerColor);

#[derive(Component)]
struct ClickedPiece();

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1028, 1028),
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
    windows: Single<&Window>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());

    let mut tiled_loader = Loader::new();

    let map = tiled_loader
        .load_tmx_map("assets/checkers_board.tmx")
        .expect("Board should have been loaded");

    let board = Board::new(&map, &windows.resolution);

    let layer = map
        .get_layer(0)
        .expect("Map should have a layer 0")
        .clone()
        .as_tile_layer()
        .expect("Layer #0 should be a TileLayer");

    for x in 0..layer.width().unwrap() {
        for y in 0..layer.height().unwrap() {
            let scaled_tile_coords = board.calc_scaled_tile_position((x, y));

            let mut tile_bund = commands.spawn((
                Sprite::from_image(asset_server.load("board.png")),
                Transform::default()
                    .with_translation(scaled_tile_coords.extend(0.))
                    .with_scale(board.calc_scale_factor().extend(0.)),
            ));

            if x % 2 == y % 2 {
                tile_bund.insert(PlayableTile);

                if y < 3 || y >= layer.height().unwrap() - 3 {
                    let (color, piece) = if y <= layer.height().unwrap().div(2) {
                        ((255., 0., 0.), Piece(PlayerColor::RED))
                    } else {
                        ((255., 255., 255.), Piece(PlayerColor::BLACK))
                    };

                    commands.spawn((
                        Mesh2d(meshes.add(Circle {
                            radius: board.tile_size.1.div(2.),
                        })),
                        MeshMaterial2d(materials.add(Color::srgb(color.0, color.1, color.2))),
                        Transform::default()
                            .with_translation(scaled_tile_coords.extend(1.))
                            .with_scale(board.calc_scale_factor().extend(0.)),
                        piece,
                    ));
                }
            }
        }
    }
    commands.insert_resource(board);
}

fn get_cursor_world_pos(
    mut cursor_world_pos: ResMut<CursorWorldPos>,
    q_primary_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = *q_camera;

    if let Some(cursor_position) = q_primary_window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        cursor_world_pos.0 = Some(world_pos)
    }
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

    let clicked_piece = pieces.iter().find(|transform| {
        transform
            .1
            .translation
            .truncate()
            .distance(cursor_world_pos)
            < board.calc_scaled_tile_size().0
    });

    if clicked_piece.is_some() {
        commands
            .get_entity(clicked_piece.unwrap().0)
            .unwrap()
            .insert(ClickedPiece());
    };
}

fn highlight_valid_moves(
    mut commands: Commands,
    board: Res<Board>,
    tiles: Query<&Transform, With<PlayableTile>>,
    pieces: Query<(&Piece, &Transform), (With<Piece>, Without<ClickedPiece>)>,
    clicked_piece: Query<(&Piece, &Transform), With<ClickedPiece>>,
) {
    let clicked_piece = clicked_piece.single();

    // tiles
    //     .iter()
    //     .filter(|tile_pos| {
    //         let clicked_pos = clicked_piece.1.translation;
    //         let tile_translation = tile_pos.translation;

    //         let is_correct_direction = if clicked_piece.0.0 == PlayerColor::BLACK {
    //             clicked_pos.y < tile_translation.y
    //         } else {
    //             clicked_pos.y > tile_translation.y
    //         };

    //         is_correct_direction
    //             && tile_translation.distance(clicked_pos)
    //                 <= board.calc_scaled_tile_size().1.mul(1.5)
    //     })
    //     .filter_map(|viable_tile| {
    //         let piece_on_tile = pieces.iter().find(|piece| {
    //             piece
    //                 .1
    //                 .translation
    //                 .truncate()
    //                 .distance(viable_tile.translation.truncate())
    //                 < board.calc_scaled_tile_size().0
    //         });

    //         piece_on_tile
    //             .filter(|piece| piece.0.0 != clicked_piece.0.0)
    //             .map(|piece| tiles.iter().find(|tile| {}))
    //     });
}
