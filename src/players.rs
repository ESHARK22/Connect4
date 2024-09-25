use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Clone, Component, PartialEq)]
pub struct Player {
    pub name: String,
    pub num: PlayerNum,
}

#[derive(Clone, PartialEq, Component)]
pub enum PlayerNum {
    Player1,
    Player2,
}
pub use PlayerNum::Player1;
pub use PlayerNum::Player2;

#[derive(Deref, DerefMut, Resource)]
pub struct CurrentPlayer(pub Player);

#[derive(Event, Default)]
pub struct NextPlayerEvent;

pub fn plugin_players(app: &mut App) {
    app.add_event::<NextPlayerEvent>();
    app.add_systems(Startup, create_players);
    app.add_systems(Startup, create_current_player.after(create_players));
    app.add_systems(Update, next_player_event_handler);
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

fn create_current_player(mut command: Commands, players: Query<&Player>) {
    command.insert_resource(CurrentPlayer(get_player1(&players)))
}

pub fn get_player1(players: &Query<&Player>) -> Player {
    match players.iter().find(|p| p.num == Player1) {
        Some(player) => player.clone(),
        None => panic!("Failed to fetch player 1"),
    }
}

pub fn get_player2(players: &Query<&Player>) -> Player {
    match players.iter().find(|p| p.num == Player2) {
        Some(player) => player.clone(),
        None => panic!("Failed to fetch player 2"),
    }
}

pub fn next_player_event_handler(
    mut events: EventReader<NextPlayerEvent>,
    players: Query<&Player>,
    mut current_player: ResMut<CurrentPlayer>,
) {
    for event in events.read() {
        match current_player.num {
            Player1 => **current_player = get_player2(&players),
            Player2 => **current_player = get_player1(&players),
        }
    }
}
