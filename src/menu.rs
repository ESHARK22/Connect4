use crate::board::GameState;
use crate::players::CurrentPlayer;

use bevy::{prelude::*, text::Text2dBundle};

pub fn plugin_status_text(app: &mut App) {
    app.add_systems(Startup, spawn_inital_status_text);
    app.add_systems(Update, write_current_status_text);
}

#[derive(Component)]
struct StatusText;

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
        GameState::Won => String::from(format!("{} has won!", current_player.name.clone())),
        GameState::Draw => String::from("No one won. No one lost. It is a draw."),
    };

    let mut text_entity = status_text.single_mut();
    *text_entity = Text::from_section(new_text, TextStyle::default())
}
