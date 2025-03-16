use crate::players::*;
use array2d::Array2D;
use backends::raycast::{bevy_mod_raycast::prelude::RaycastVisibility, RaycastBackendSettings};
use bevy::{
    color::palettes::css::{DARK_GREEN, DIM_GREY, LIGHT_GREEN},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::Stopwatch,
};
use bevy_mod_picking::prelude::*;

use Annulus as Ring;

const ROW_COUNT: f32 = 6.;
const COL_COUNT: f32 = 7.;

const CIRCLE_RADIUS: f32 = 24.;
const CIRCLE_DIAMETER: f32 = CIRCLE_RADIUS * 2.;

const GAP: f32 = 5.;

const RED: Color = Color::srgb(1., 0., 0.);
const BLUE: Color = Color::srgb(0., 0., 1.);
const YELLOW: Color = Color::srgb(1.5, 1.5, 0.);
const BLACK: Color = Color::srgb(0., 0., 0.);

pub fn plugin_board(app: &mut App) {
    app.insert_resource(RaycastBackendSettings {
        require_markers: false,
        raycast_visibility: RaycastVisibility::Ignore,
    });
    app.insert_resource(Board::default());
    app.insert_resource(GameState::Playing);

    app.add_systems(Startup, spawn_board_background);
    app.add_systems(Startup, spawn_initial_chips);
    app.add_systems(Startup, spawn_initial_col_hightlights);

    app.add_systems(Update, update_col_rect_visibility);

    app.add_systems(Update, update_chip_colour);

    app.add_systems(Update, update_update_game_state);
}

#[derive(Deref, DerefMut, Resource)]
pub struct Board(Array2D<BoardState>);

impl Default for Board {
    fn default() -> Self {
        Board(Array2D::filled_with(BoardState::Empty, 6, 7))
    }
}

#[derive(Clone, PartialEq)]
pub enum BoardState {
    Empty,
    Taken(Player),
}

#[derive(Clone, PartialEq, Resource, Reflect)]
#[reflect(Resource)]
pub enum GameState {
    Playing,
    Won,
    Draw,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Chip {
    row: f32,
    col: f32,
}

#[derive(Component)]
struct ColPicker {
    col: f32,
}

pub fn is_at_bottom(board: Board, row: usize, column: usize) -> bool {
    // Check if anything exists below, and if it does, make sure it is not empty
    // If it is empty, then we are not at the bottom

    // There are none below the 0th
    if row == 0 {
        return true;
    }

    match board.get(row - 1, column) {
        Some(state) => {
            match state {
                BoardState::Taken(_) => true, // Cant go any lower
                BoardState::Empty => false,   // Could have gone lower
            }
        }
        None => panic!("Tried to access non-existing chip"),
    }
}

pub fn get_lowest_chip_row(board: Board, col: usize) -> Option<usize> {
    let mut row_index = ROW_COUNT as usize - 1;

    // If the top is taken, return none, as the column is already full
    if let Some(state) = board.get(row_index, col) {
        match *state {
            BoardState::Taken(_) => return None,
            BoardState::Empty => {}
        }
    }

    while !is_at_bottom(Board(board.clone()), row_index, col) {
        row_index -= 1
    }

    Some(row_index)
}

fn is_full(board: Board) -> bool {
    // The board is full when there are no more empty spaces
    !board
        .elements_row_major_iter()
        .any(|f| f == &BoardState::Empty)
}

fn check_horizontal_wins(board: Board) -> Option<Player> {
    // Check for 4 in a row, on all rows

    // As close to the right as we can check for 4 in a row
    // (eg at col 4, there are only 2 to the right of it, so its impossible to get 4 in a row starting from col 4)
    let max_col_index = board.num_columns() - 3;

    for row_index in 0..board.num_rows() {
        for col_index in 0..max_col_index {
            // Its fine to unwrap here, since if the item doesnt exist, something is wrong with max_col_index
            // (We should never be trying to read a non existent item here)
            let item1 = board.get(row_index, col_index).unwrap().clone();
            let item2 = board.get(row_index, col_index + 1).unwrap().clone();
            let item3 = board.get(row_index, col_index + 2).unwrap().clone();
            let item4 = board.get(row_index, col_index + 3).unwrap().clone();

            if let BoardState::Taken(player) = item1.clone() {
                if item1 == item2 && item1 == item3 && item1 == item4 {
                    // We found 4 in a row!
                    return Some(player.clone());
                }
            } else {
                // The board state is empty
                // Continue searching for a winner
                continue;
            }
        }
    }

    // No wins were found
    None
}
fn check_vertical_wins(board: Board) -> Option<Player> {
    // Check for 4 in a column, on all columns

    // How low down we can go, where the is still 4 items to check
    // (eg you cant get ma vertical win if you start from the bottom row)
    let max_row_index = board.num_columns() - 4;

    for col_index in 0..board.num_columns() {
        for row_index in 0..max_row_index {
            let item1 = board.get(row_index, col_index).unwrap().clone();
            let item2 = board.get(row_index + 1, col_index).unwrap().clone();
            let item3 = board.get(row_index + 2, col_index).unwrap().clone();
            let item4 = board.get(row_index + 3, col_index).unwrap().clone();

            if let BoardState::Taken(player) = item1.clone() {
                if item1 == item2 && item1 == item3 && item1 == item4 {
                    return Some(player.clone());
                }
            } else {
                continue;
            }
        }
    }

    None
}
fn check_diagonal_wins(board: Board) -> Option<Player> {
    // Check for 4 in a diagonal, in both direction

    // First focus on ones going from top left to bottom right
    let max_row_index = board.num_columns() - 4;
    let max_col_index = board.num_columns() - 3;

    for row_index in 0..max_row_index {
        for col_index in 0..max_col_index {
            let item1 = board.get(row_index, col_index).unwrap().clone();
            let item2 = board.get(row_index + 1, col_index + 1).unwrap().clone();
            let item3 = board.get(row_index + 2, col_index + 2).unwrap().clone();
            let item4 = board.get(row_index + 3, col_index + 3).unwrap().clone();

            if let BoardState::Taken(player) = item1.clone() {
                if item1 == item2 && item1 == item3 && item1 == item4 {
                    return Some(player.clone());
                }
            } else {
                // Empty space
                continue;
            }
        }
    }

    // Now focus on ones going from top right to bottom left
    let min_col_index = board.num_columns() - max_col_index;

    for row_index in 0..max_row_index {
        for col_index in (min_col_index..board.num_columns()).rev() {
            let item1 = board.get(row_index, col_index).unwrap().clone();
            let item2 = board.get(row_index + 1, col_index - 1).unwrap().clone();
            let item3 = board.get(row_index + 2, col_index - 2).unwrap().clone();
            let item4 = board.get(row_index + 3, col_index - 3).unwrap().clone();

            if let BoardState::Taken(player) = item1.clone() {
                if item1 == item2 && item1 == item3 && item1 == item4 {
                    return Some(player.clone());
                }
            } else {
                // Empty space
                continue;
            }
        }
    }

    None
}

fn spawn_board_background(
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
        transform: Transform::from_xyz(bg_width / 2., bg_height / 2., -1.), // Half to center it, -1 to make it behind other stuff
        ..default()
    });
}

fn spawn_initial_chips(
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
                    transform: Transform::from_xyz(x, y, 1.),
                    ..default()
                },
                Chip { row, col },
                Pickable::IGNORE,
            ));
        }
    }
}

fn spawn_initial_col_hightlights(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rect = Mesh2dHandle(meshes.add(Rectangle::new(
        GAP + CIRCLE_DIAMETER,
        (GAP + CIRCLE_DIAMETER) * ROW_COUNT + GAP,
    )));

    let mut transparent_black = BLACK;
    transparent_black.set_alpha(0.5);

    for col in 0..COL_COUNT as u8 {
        let col = col as f32;

        let x = GAP + CIRCLE_RADIUS + (col * CIRCLE_DIAMETER) + (col * GAP);
        let y = GAP + CIRCLE_RADIUS + (2. * CIRCLE_DIAMETER) + (7. * GAP); // TODO: Acctual calculations

        command.spawn((
            MaterialMesh2dBundle {
                mesh: rect.clone(),
                material: materials.add(transparent_black),
                transform: Transform::from_xyz(x, y, 0.),
                visibility: Visibility::Visible,
                ..default()
            },
            ColPicker { col },
            PickableBundle::default(),
            On::<Pointer<Click>>::run(update_col_handle_click),
        ));
    }
}

fn update_chip_colour(
    board: Res<Board>,
    mut chip_query: Query<(&Chip, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (chip, colour_handle) in &mut chip_query.iter_mut() {
        // Get the board state from the 2d array
        let state = board.get(chip.row as usize, chip.col as usize).unwrap();

        // If its taken, display the players colour,
        let colour = match state {
            BoardState::Taken(player) => match player.num {
                PlayerNum::Player1 => RED,
                PlayerNum::Player2 => YELLOW,
            },
            BoardState::Empty => BLACK,
        };

        let chip_material = materials.get_mut(colour_handle).unwrap();

        if chip_material.color != colour {
            chip_material.color = colour
        }
    }
}

fn update_col_rect_visibility(
    mut col_query: Query<(&ColPicker, &PickingInteraction, &mut Visibility)>,
) {
    for (col, interaction, mut visibility) in &mut col_query.iter_mut() {
        match interaction {
            PickingInteraction::None => *visibility = Visibility::Hidden,
            PickingInteraction::Pressed => *visibility = Visibility::Hidden,
            PickingInteraction::Hovered => *visibility = Visibility::Visible,
        }
    }
}

fn update_col_handle_click(
    event: Listener<Pointer<Click>>,
    game_state: Res<GameState>,
    time: Res<Time>,
    mut col_query: Query<(&mut ColPicker, &Handle<ColorMaterial>)>,
    mut board: ResMut<Board>,
    mut current_player: ResMut<CurrentPlayer>,
    mut next_player_event: EventWriter<NextPlayerEvent>,
) {
    if !(*game_state == GameState::Playing) {
        return;
    }

    let Ok((mut col, colour_handle)) = col_query.get_mut(event.target) else {
        panic!("Tried to get target from query, where it doesnt exist in the query")
    };

    let column = col.col as usize;
    let Some(row) = get_lowest_chip_row(Board(board.clone()), column) else {
        return;
    };

    let Some(board_state) = board.get_mut(row, column) else {
        panic!("Tried to get the value of a non-existing hole")
    };

    // Dont update any holes which have already been taken
    if *board_state != BoardState::Empty {
        debug!("Tried to change the state of a taken board state...ignoring");
        return;
    }

    // Set the new board state
    *board_state = BoardState::Taken(current_player.clone());

    // Next player
    next_player_event.send_default();
}

fn update_update_game_state(mut game_state: ResMut<GameState>, board: Res<Board>) {
    if is_full(Board(board.clone())) {
        println!("Draw!");
        *game_state = GameState::Draw;
        return;
    }

    if check_vertical_wins(Board(board.clone())).is_some()
        || check_diagonal_wins(Board(board.clone())).is_some()
        || check_horizontal_wins(Board(board.clone())).is_some()
    {
        *game_state = GameState::Won;
    }
}
