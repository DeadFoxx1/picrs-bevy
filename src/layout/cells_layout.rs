use bevy::{prelude::*, window::WindowResized};

use crate::{CellCount, layout::bg_layout::init_bg_dimensions};

pub struct CellsLayoutPlugin;
impl Plugin for CellsLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_cell_dimensions.after(init_bg_dimensions))
            .add_systems(Update, update_cell_dimensions);
    }
}

const BORDER_TO_CELL_FG_RATIO: (f32, f32) = (1., 9.); //1:6

#[derive(Resource)]
pub struct CellDimensions {
    pub fg_size: f32,
    pub border_size: f32,
    pub top_of_board: f32,
    pub left_of_board: f32,
}
impl CellDimensions {
    fn new(nrow: usize, ncol: usize) -> Self {
        let n = usize::max(nrow, ncol);
        let border_size = ((BORDER_TO_CELL_FG_RATIO.0
            / (BORDER_TO_CELL_FG_RATIO.1 + BORDER_TO_CELL_FG_RATIO.0))
            / n as f32)
            / 2.;
        let fg_size = (BORDER_TO_CELL_FG_RATIO.1
            / (BORDER_TO_CELL_FG_RATIO.0 + BORDER_TO_CELL_FG_RATIO.1))
            / n as f32
            + (((n - 1) as f32 * border_size) / n as f32);

        let top_of_board = 1. / 2.;
        let left_of_board = -top_of_board;

        CellDimensions {
            fg_size,
            border_size,
            top_of_board,
            left_of_board,
        }
    }
}

pub fn init_cell_dimensions(cell_count: Res<CellCount>, mut commands: Commands) {
    commands.insert_resource(CellDimensions::new(cell_count.nrow, cell_count.ncol));
}

fn update_cell_dimensions(
    mut cell_dimensions: ResMut<CellDimensions>,
    mut resized_events: MessageReader<WindowResized>,
    cell_count: Res<CellCount>,
) {
    for _event in resized_events.read() {
        *cell_dimensions = CellDimensions::new(cell_count.nrow, cell_count.ncol);
    }
}
