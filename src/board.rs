pub mod bg;
pub mod cells;
pub mod hints;

use bevy::prelude::*;
use bg::BgPlugin;
use cells::CellsPlugin;
use hints::HintsPlugin;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BgPlugin, CellsPlugin, HintsPlugin));
    }
}
pub const GRID_BG_COLOR: [f32; 3] = [0., 0., 0.];

pub const HINTS_PERCENT: f32 = 0.25;
pub const VERT_MARGIN_PERCENT: f32 = 0.05; //5% of the screen's height per margin
pub const HORI_MARGIN_PERCENT: f32 = 0.30;
pub const SIDE_MARGIN_RATIO: (f32, f32) = (9., 1.); //9:1

pub const CELL_FG_COLOR: [f32; 3] = [1., 1., 1.];
pub const CELL_FILLED_COLOR: [f32; 3] = [0., 0., 0.];
pub const CELL_CROSSED_COLOR: [f32; 3] = [1., 0., 0.];
pub const BORDER_TO_CELL_FG_RATIO: (f32, f32) = (1., 10.); //1:20

pub const HINTS_FG_COLOR: [f32; 3] = [1., 1., 1.];
pub const BORDER_TO_HINTS_FG_RATIO: (f32, f32) = (1., 10.); //1:20

pub const HINT_BG_COLOR: [f32; 3] = [0., 0., 0.];
