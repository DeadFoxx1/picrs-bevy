use bevy::prelude::*;
use picrs_lib::table::Table;

use crate::{
    app_state::{
        game::board::cells::{Cell, CellMatl, CellState},
        AppState,
    },
    CellCount, GameState,
};

pub struct GameEventsPlugin;
impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(game_win).add_observer(game_reset);
    }
}

#[derive(Event)]
pub struct GameWin;

pub fn game_win(
    _: On<GameWin>,
    mut query: Query<(&Cell, &mut MeshMaterial2d<ColorMaterial>)>,
    cell_matl: Res<CellMatl>,
) {
    for (cell, mut matl) in query.iter_mut() {
        if cell.cell_state == CellState::Filled {
            **matl = cell_matl.green.clone();
        }
    }
}

#[derive(Event)]
pub struct GameReset;

pub fn game_reset(
    _: On<GameReset>,
    mut query: Query<(&mut Cell, &mut MeshMaterial2d<ColorMaterial>)>,
    cell_matl: Res<CellMatl>,
    mut game_state: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
    cell_count: Res<CellCount>,
) {
    for (mut cell, mut matl) in query.iter_mut() {
        cell.cell_state = CellState::Empty;
        **matl = cell_matl.empty.clone();
        game_state.0 = Table::new(cell_count.ncol, cell_count.nrow, cell_count.nfilled);
    }
    next_state.set(AppState::MainMenu);
    next_state.set(AppState::InGame);
}
