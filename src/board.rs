pub mod grid;
pub mod hints;

use bevy::prelude::*;
use grid::GridPlugin;

pub struct BoardPlugin;
impl Plugin for BoardPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(GridPlugin);
    }
}
