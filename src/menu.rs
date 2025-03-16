use crate::board::{Board, BoardState, GameState};
use crate::players::CurrentPlayer;

use array2d::Array2D;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::{prelude::*, text::Text2dBundle};
use bevy_mod_picking::events::{Click, Pointer};
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::PickableBundle;

pub fn plugin_status_text(app: &mut App) {
    app.register_type::<StatusText>();
    app.register_type::<ResetButton>();

    app.add_systems(Startup, spawn_inital_status_text);
    app.add_systems(Update, write_current_status_text);
    app.add_systems(Update, show_hide_reset_button);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct StatusText;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ResetButton;

fn spawn_inital_status_text(mut command: Commands) {
    command.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Starting...",
            TextStyle::default(),
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            top: Val::Px(5.0),
            ..default()
        }),
        StatusText,
    ));
}

fn write_current_status_text(
    current_player: Res<CurrentPlayer>,
    game_state: Res<GameState>,
    mut status_text: Query<&mut Text, With<StatusText>>,
) {
    let new_text = match *game_state {
        GameState::Playing => current_player.name.clone(),
        GameState::Won => format!("{} has won!", current_player.name.clone()),
        GameState::Draw => String::from("No one won. No one lost. It is a draw."),
    };

    let mut text_entity = status_text.single_mut();
    *text_entity = Text::from_section(new_text, TextStyle::default())
}

fn spawn_click_to_reset(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let text_bg_rect = Mesh2dHandle(meshes.add(Rectangle::new(8., 15.)));
    command.spawn((
        MaterialMesh2dBundle {
            mesh: text_bg_rect,
            material: materials.add(Color::BLACK),
            transform: Transform::from_xyz(0., 0., 0.),
            visibility: Visibility::Hidden,
            ..default()
        },
        PickableBundle::default(),
        On::<Pointer<Click>>::run(reset_game),
        ResetButton,
    ));
}

fn show_hide_reset_button(
    game_state: Res<GameState>,
    mut button: Query<&mut Visibility, With<ResetButton>>,
) {
    let mut button_visibility: Mut<'_, Visibility> = button.get_single_mut().unwrap();
    match *game_state {
        GameState::Playing => *button_visibility = Visibility::Hidden,
        GameState::Won => *button_visibility = Visibility::Visible,
        GameState::Draw => *button_visibility = Visibility::Visible,
    };
}

fn reset_game(mut command: Commands) {
    command.insert_resource(GameState::Playing);
    command.insert_resource(Board::default());
}
