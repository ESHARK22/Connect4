#![allow(unused)]

use array2d::Array2D;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use Annulus as Ring;

const ROW_COUNT: f32 = 6.0;
const COL_COUNT: f32 = 7.0;

const CIRCLE_RADIUS: f32 = 24.0;
const CIRCLE_DIAMETER: f32 = CIRCLE_RADIUS * 2.0;

const GAP: f32 = 5.0;

const RED: Color = Color::srgb(1.0, 0.0, 0.0);
const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
const YELLOW: Color = Color::srgb(1.5, 1.5, 0.0);
const BLACK: Color = Color::srgb(0.0, 0.0, 0.0);

#[derive(Deref, DerefMut, Resource)]
struct Board(Array2D<BoardState>);

#[derive(Clone)]
enum BoardState {
    Empty,
    Taken(Player),
}

#[derive(Component)]
struct Chip {
    row: f32,
    col: f32,
}

#[derive(Clone, Component)]
struct Player {
    name: String,
    num: PlayerNum,
}

#[derive(Clone, Component)]
enum PlayerNum {
    Player1,
    Player2,
}

fn main() {
    let mut app = App::new();

    // Plugins
    app.add_plugins((DefaultPlugins));
    app.add_plugins(WorldInspectorPlugin::new());

    // On startup
    app.add_systems(Startup, setup_camera);
    app.add_systems(Startup, create_board_resource);
    app.add_systems(Startup, create_board_background);
    app.add_systems(Startup, draw_initial_chips);
    app.add_systems(PostStartup, test_update_board_state);

    // Normal updates
    app.add_systems(Update, update_chip_colour);

    app.run();
}

fn setup_camera(mut command: Commands) {
    command.spawn(Camera2dBundle::default());
}

fn create_board_background(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // A Blue backgorund for the board
    let bg_width = GAP + (COL_COUNT * CIRCLE_DIAMETER) + (COL_COUNT * GAP);
    let bg_height = GAP + (ROW_COUNT * CIRCLE_DIAMETER) + (ROW_COUNT * GAP);

    // Th bg shape
    let bg_rectangle = Mesh2dHandle(meshes.add(Rectangle::new(bg_width, bg_height)));

    command.spawn(MaterialMesh2dBundle {
        mesh: bg_rectangle.clone(),
        material: materials.add(BLUE),
        transform: Transform::from_xyz(bg_width / 2.0, bg_height / 2.0, -1.0), // Half to center it, -1 to make it behind other stuff
        ..default()
    });
}

fn create_board_resource(mut command: Commands) {
    command.insert_resource(Board(Array2D::filled_with(BoardState::Empty, 6, 7)));
}

fn create_players(mut command: Commands) {
    command.spawn(Player {
        name: "Player 1".into(),
        num: PlayerNum::Player1,
    });
    command.spawn(Player {
        name: "Player 2".into(),
        num: PlayerNum::Player2,
    });
}

fn draw_initial_chips(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Circle test
    let circle = Mesh2dHandle(meshes.add(Circle::new(CIRCLE_RADIUS)));

    // Draw the holes
    for row in 0..ROW_COUNT as u8 {
        for col in 0..COL_COUNT as u8 {
            let row = row as f32;
            let col = col as f32;

            let x = GAP + CIRCLE_RADIUS + (col * CIRCLE_DIAMETER) + (col * GAP);
            let y = GAP + CIRCLE_RADIUS + (row * CIRCLE_DIAMETER) + (row * GAP);

            command.spawn((
                MaterialMesh2dBundle {
                    mesh: circle.clone(),
                    material: materials.add(BLACK),
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                Chip { row, col },
            ));
        }
    }
}

fn update_chip_colour(
    board: Res<Board>,
    mut chip_query: Query<(&Chip, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (chip, colour_handle) in &mut chip_query.iter_mut() {
        match board.get(chip.row as usize, chip.col as usize) {
            None => panic!("Tried to check a space that doesnt exist?!"),
            Some(state) => {
                let colour = match state {
                    BoardState::Empty => BLACK,
                    BoardState::Taken(player) => match player.num {
                        PlayerNum::Player1 => RED,
                        PlayerNum::Player2 => YELLOW,
                    },
                };

                let chip_material = materials.get_mut(colour_handle).unwrap();
                if chip_material.color != colour {
                    chip_material.color = colour
                }
            }
        }
    }
}

fn test_update_board_state(mut board: ResMut<Board>) {
    board[(2, 3)] = BoardState::Taken(player.as_ref().clone());
}
