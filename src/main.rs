mod board;
mod cursor;

use crate::board::BoardPlugin;
use crate::cursor::CursorPlugin;
use bevy::prelude::*;
use picrs_lib::table::Table;

const START_NROW: usize = 5;
const START_NCOL: usize = 5;
const START_FILL: usize = 15;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, BoardPlugin, MeshPickingPlugin, CursorPlugin))
        .add_systems(Startup, init)
        .insert_resource(CellCount {
            nrow: START_NROW,
            ncol: START_NCOL,
        })
        .insert_resource(GameState(Table::new(START_NCOL, START_NROW, START_FILL)))
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource)]
pub struct CellCount {
    nrow: usize,
    ncol: usize,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameState(Table);
