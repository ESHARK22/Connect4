#![allow(unused)]
use revy;
use array2d::Array2D;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;

mod players;
use players::*;
mod board;
use board::*;
mod menu;
use menu::*;

fn main() {
    let mut app = App::new();

    // Plugins
    app.add_plugins(DefaultPlugins);
    app.add_plugins(
        DefaultPickingPlugins
            .build()
            .disable::<DebugPickingPlugin>()
            .disable::<DefaultHighlightingPlugin>(),
    );
    // app.add_plugins(DebugPickingPlugin);
    app.add_plugins(WorldInspectorPlugin::new());

    app..add_plugins({
        let rec = revy::RecordingStreamBuilder::new("connect4.").spawn().unwrap();
        revy::RerunPlugin { rec }
    })

    app.add_plugins(plugin_board);
    app.add_plugins(plugin_players);
    app.add_plugins(plugin_status_text);

    // On startup
    app.add_systems(Startup, setup_camera);

    app.run();
}

fn setup_camera(mut command: Commands) {
    command.spawn(Camera2dBundle::default());
}
