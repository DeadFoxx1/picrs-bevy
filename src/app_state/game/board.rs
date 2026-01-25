pub mod bg;
pub mod cells;
pub mod hints;
pub mod divider;

use bevy::prelude::*;
use bg::BgPlugin;
use cells::CellsPlugin;
use hints::HintsPlugin;

use crate::app_state::game::board::divider::DividerPlugin;

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BgPlugin, CellsPlugin, HintsPlugin, DividerPlugin));
    }
}
pub const GRID_BG_COLOR: [f32; 3] = [0.5, 0.51, 0.59];

pub const HINTS_PERCENT: f32 = 0.25;

pub const VERT_MARGIN_PERCENT: f32 = 0.05; //5% of the screen's height per margin
pub const HORI_MARGIN_PERCENT: f32 = 0.05;
pub const HORI_MARGIN_RATIO: (f32, f32) = (1., 1.); //9:1
pub const VERT_MARGIN_RATIO: (f32, f32) = (1., 1.); //9:1

pub const CELL_EMPTY_COLOR: [f32; 3] = [1., 1., 1.];
pub const CELL_FILLED_COLOR: [f32; 3] = [0.12, 0.12, 0.18];
pub const CELL_CROSSED_COLOR: [f32; 3] = [1., 0.22, 0.26];
pub const CELL_SOLVED_COLOR: [f32; 3] = [0.4, 1., 0.5];
pub const BORDER_TO_CELL_FG_RATIO: (f32, f32) = (1., 20.); //1:20

pub const HINTS_FG_COLOR: [f32; 3] = [1., 1., 1.];
pub const BORDER_TO_HINTS_FG_RATIO: (f32, f32) = (1., 20.); //1:20

pub const HINT_BG_COLOR: [f32; 3] = [0.5, 0.51, 0.59];

pub const DIVIDER_COLOR: [f32; 3] = [0., 0., 0.];
