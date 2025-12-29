pub mod grid_bg;
pub mod grid_cells;

use bevy::prelude::*;
use grid_bg::GridBgPlugin;

use crate::grid::grid_cells::GridCellsPlugin;

pub struct GridPlugin;
impl Plugin for GridPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((GridBgPlugin, GridCellsPlugin));
    }
}
