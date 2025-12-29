pub mod grid_bg;

use bevy::prelude::*;
use grid_bg::GridBgPlugin;

pub struct GridPlugin;
impl Plugin for GridPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(GridBgPlugin);
    }
}
