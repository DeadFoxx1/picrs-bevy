use bevy::prelude::*;

use crate::{
    CellCount,
    grid::grid_bg::Bg,
    layout::cells_layout::{CellDimensions, init_cell_dimensions},
};

pub struct GridCellsPlugin;
impl Plugin for GridCellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_board_cells.after(init_cell_dimensions));
    }
}

const CELL_FG_COLOR: [f32; 3] = [1., 1., 1.];

#[derive(Component)]
struct Cell;

fn draw_board_cells(
    cell_dimensions: Res<CellDimensions>,
    cell_count: Res<CellCount>,
    board_bg: Single<Entity, With<Bg>>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let fg_size = cell_dimensions.fg_size;
    let total_size = cell_dimensions.total_size;
    let top_of_board = cell_dimensions.top_of_board;
    let left_of_board = cell_dimensions.left_of_board;
    for x in 0..cell_count.ncol {
        for y in 0..cell_count.nrow {
            commands.spawn((
                Cell,
                Mesh2d(mesh.add(Rectangle::default())),
                MeshMaterial2d(material.add(Color::srgb_from_array(CELL_FG_COLOR))),
                Transform::from_translation(Vec3::new(
                    left_of_board + (total_size / 2.) + (total_size * x as f32),
                    top_of_board - (total_size / 2.) - (total_size * y as f32),
                    1.,
                ))
                .with_scale(Vec3::new(fg_size, fg_size, 1.)),
                ChildOf(*board_bg),
            ));
        }
    }
}
