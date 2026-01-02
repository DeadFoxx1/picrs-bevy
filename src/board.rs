pub mod bg;
pub mod cells;
pub mod hints;

use bevy::prelude::*;
use bg::BgPlugin;
use cells::CellsPlugin;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BgPlugin, CellsPlugin));
    }
}
pub const GRID_BG_COLOR: [f32; 3] = [0., 0., 0.];

pub const TOP_HINTS_PERCENT: f32 = 0.25;
pub const TOP_MARGIN_PERCENT: f32 = 0.05; //5% of the screen's height per margin
pub const SIDE_MARGIN_RATIO: (f32, f32) = (9., 1.); //9:1

//in the case that screen width < screen height
pub const MIN_SIZE_OF_SQUARE_PERCENT: f32 = 0.70; //70% of the screens width

pub const CELL_FG_COLOR: [f32; 3] = [1., 1., 1.];
pub const BORDER_TO_CELL_FG_RATIO: (f32, f32) = (1., 20.); //1:20

pub const HINT_BG_COLOR: [f32; 3] = [1., 1., 0.];
