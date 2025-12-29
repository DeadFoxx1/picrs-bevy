pub mod bg_layout;
pub mod cells_layout;

use bevy::prelude::*;
use bg_layout::BgLayoutPlugin;

use crate::layout::cells_layout::CellsLayoutPlugin;

pub struct LayoutPlugin;
impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BgLayoutPlugin, CellsLayoutPlugin));
    }
}
