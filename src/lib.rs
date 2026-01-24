use bevy::prelude::*;
use picrs_lib::table::Table;

use crate::app_state::StatePlugin;
mod app_state;

const START_NROW: usize = 10;
const START_NCOL: usize = 10;
const START_FILL: usize = 75;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StatePlugin)
            .insert_resource(CellCount {
                nrow: START_NROW,
                ncol: START_NCOL,
            })
            .insert_resource(GameState(Table::new(START_NCOL, START_NROW, START_FILL)))
            .add_systems(Startup, init);
    }
}
#[derive(Resource)]
pub struct CellCount {
    nrow: usize,
    ncol: usize,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameState(Table);

fn init(mut commands: Commands) {
    commands.spawn(Camera2d);
}
