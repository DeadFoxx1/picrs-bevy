pub mod grid;
pub mod hints;

use bevy::prelude::*;
use grid::GridPlugin;

pub struct DrawPlugin;
impl Plugin for DrawPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(GridPlugin);
    }
}
