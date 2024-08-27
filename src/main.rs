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

#[derive(Clone, Resource)]
struct Player {
    name: String,
    num: PlayerNum,
    colour: Color,
}

#[derive(Clone)]
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
    app.add_systems(Startup, setup);

    // Normal updates
    app.add_systems(Update, keyboard_system);

    app.run();
}

fn setup_camera(mut command: Commands) {
    command.spawn(Camera2dBundle::default());
}

fn setup_game_resources(mut command: Commands) {
    // Spawn a player
    command.insert_resource(Player {
        name: "Player 1".into(),
        num: PlayerNum::Player1,
        colour: RED,
    });

    // Spawn another player
    command.insert_resource(Player {
        name: "Player 2".into(),
        num: PlayerNum::Player2,
        colour: YELLOW,
    });

    // Spawn a board
    command.insert_resource(Board(Array2D::filled_with(BoardState::Empty, 6, 7)));
}

fn setup(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Circle test
    let circle = Mesh2dHandle(meshes.add(Circle::new(CIRCLE_RADIUS)));

    // Red circle
    command.spawn(MaterialMesh2dBundle {
        mesh: circle.clone(),
        material: materials.add(RED),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    });

    // Yellow circle
    command.spawn(MaterialMesh2dBundle {
        mesh: circle.clone(),
        material: materials.add(YELLOW),
        transform: Transform::from_xyz(60.0, 0.0, 1.0),
        ..default()
    });

    // A Blue backgorund for the board
    let bg_width = GAP + (COL_COUNT * CIRCLE_DIAMETER) + (COL_COUNT * GAP);
    let bg_height = GAP + (ROW_COUNT * CIRCLE_DIAMETER) + (ROW_COUNT * GAP);

    let bg_rectangle = Mesh2dHandle(meshes.add(Rectangle::new(bg_width, bg_height)));
    command.spawn(MaterialMesh2dBundle {
        mesh: bg_rectangle.clone(),
        material: materials.add(BLUE),
        transform: Transform::from_xyz(bg_width / 2.0, bg_height / 2.0, -1.0), // Half to center it
        ..default()
    });

    for row in 0..ROW_COUNT as u8 {
        for col in 0..COL_COUNT as u8 {
            let row = row as f32;
            let col = col as f32;

            let x = GAP + CIRCLE_RADIUS + (col * CIRCLE_DIAMETER) + (col * GAP);
            let y = GAP + CIRCLE_RADIUS + (row * CIRCLE_DIAMETER) + (row * GAP);

            command.spawn(MaterialMesh2dBundle {
                mesh: circle.clone(),
                material: materials.add(BLACK),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            });
        }
    }
}

fn keyboard_system(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        info!("Space Pressed!")
    }
}
