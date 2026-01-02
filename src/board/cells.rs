use super::{BORDER_TO_CELL_FG_RATIO, CELL_FG_COLOR};
use bevy::prelude::*;

use crate::{
    CellCount,
    board::bg::{GridBg, draw_board_bg},
};

pub struct CellsPlugin;
impl Plugin for CellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_board_cells.after(draw_board_bg));
    }
}

#[derive(Component)]
struct Cell;

fn draw_board_cells(
    cell_count: Res<CellCount>,
    board_bg: Single<Entity, With<GridBg>>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let n = usize::max(cell_count.nrow, cell_count.ncol);
    let border_size = ((BORDER_TO_CELL_FG_RATIO.0
        / (BORDER_TO_CELL_FG_RATIO.1 + BORDER_TO_CELL_FG_RATIO.0))
        / n as f32)
        / 2.;
    let border_size = border_size + ((n - 2) as f32 * border_size) / n as f32;
    let fg_size = (BORDER_TO_CELL_FG_RATIO.1
        / (BORDER_TO_CELL_FG_RATIO.0 + BORDER_TO_CELL_FG_RATIO.1))
        / n as f32;
    //+ (((n - 1) as f32 * border_size) / n as f32);

    let top_of_board = 1. / 2.;
    let left_of_board = -top_of_board;

    for x in 0..cell_count.ncol {
        for y in 0..cell_count.nrow {
            commands.spawn((
                Cell,
                Mesh2d(mesh.add(Rectangle::default())),
                MeshMaterial2d(material.add(Color::srgb_from_array(CELL_FG_COLOR))),
                Transform::from_translation(Vec3::new(
                    left_of_board
                        + (border_size + fg_size / 2.)
                        + (border_size + fg_size) * x as f32,
                    top_of_board
                        - (border_size + fg_size / 2.)
                        - (border_size + fg_size) * y as f32,
                    1.,
                ))
                .with_scale(Vec3::new(fg_size, fg_size, 1.)),
                ChildOf(*board_bg),
            ));
        }
    }
}
