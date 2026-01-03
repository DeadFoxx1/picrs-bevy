mod board;

use crate::board::BoardPlugin;
use bevy::prelude::*;
use picrs_lib::table::Table;

const START_NROW: usize = 5;
const START_NCOL: usize = 5;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, BoardPlugin, MeshPickingPlugin))
        .add_systems(Startup, init)
        .insert_resource(CellCount {
            nrow: START_NROW,
            ncol: START_NCOL,
        })
        .insert_resource(GameState(Table::new(START_NCOL, START_NROW, 20)))
        .run();
}

fn init(mut commands: Commands, game_state: Res<GameState>) {
    commands.spawn(Camera2d);
    for x in game_state.columns[0].hints.clone() {
        println!("{}", x);
    }
    for x in game_state.rows[0].hints.clone() {
        println!("{}", x);
    }
}

#[derive(Resource)]
pub struct CellCount {
    nrow: usize,
    ncol: usize,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameState(Table);
