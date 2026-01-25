use super::{BORDER_TO_CELL_FG_RATIO, CELL_CROSSED_COLOR, CELL_EMPTY_COLOR, CELL_FILLED_COLOR};
use bevy::prelude::*;

use crate::{
    CellCount,
    app_state::{
        AppState,
        game::{
            board::{
                CELL_SOLVED_COLOR,
                bg::{GridBg, draw_board_bg},
            },
            events::{paint_cell, toggle_cursor},
        },
    },
};

pub struct CellsPlugin;
impl Plugin for CellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (draw_board_cells.after(draw_board_bg), CellMatl::init),
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Filled,
    Empty,
    Crossed,
}
#[derive(Component)]
pub struct Cell {
    pub cell_state: CellState,
    pub coords: (usize, usize),
}
#[derive(Resource)]
pub struct CellMatl {
    pub empty: Handle<ColorMaterial>,
    pub crossed: Handle<ColorMaterial>,
    pub filled: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
}
impl CellMatl {
    pub fn init(mut commands: Commands, mut material: ResMut<Assets<ColorMaterial>>) {
        let empty = material.add(Color::srgb_from_array(CELL_EMPTY_COLOR));
        let crossed = material.add(Color::srgb_from_array(CELL_CROSSED_COLOR));
        let filled = material.add(Color::srgb_from_array(CELL_FILLED_COLOR));
        let green = material.add(Color::srgb_from_array(CELL_SOLVED_COLOR));
        let cell_matl = CellMatl {
            empty,
            crossed,
            filled,
            green,
        };
        commands.insert_resource(cell_matl);
    }
}

fn draw_board_cells(
    cell_count: Res<CellCount>,
    board_bg: Single<Entity, With<GridBg>>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    cell_matl: Res<CellMatl>,
) {
    let n = usize::max(cell_count.nrow, cell_count.ncol);
    let border_size = ((BORDER_TO_CELL_FG_RATIO.0
        / (BORDER_TO_CELL_FG_RATIO.1 + BORDER_TO_CELL_FG_RATIO.0))
        / n as f32)
        / 2.;
    let border_size = border_size + ((n - 1) as f32 * border_size) / (n as f32 + 1.);
    let fg_size = (BORDER_TO_CELL_FG_RATIO.1
        / (BORDER_TO_CELL_FG_RATIO.0 + BORDER_TO_CELL_FG_RATIO.1))
        / n as f32;
    //+ (((n - 1) as f32 * border_size) / n as f32);

    let top_of_board = 1. / 2.;
    let left_of_board = -top_of_board;

    for x in 0..cell_count.ncol {
        for y in 0..cell_count.nrow {
            commands
                .spawn((
                    Cell {
                        cell_state: CellState::Empty,
                        coords: (x, y),
                    },
                    Mesh2d(mesh.add(Rectangle::default())),
                    MeshMaterial2d(cell_matl.empty.clone()),
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
                ))
                .observe(toggle_cursor)
                .observe(paint_cell);
        }
    }
}
