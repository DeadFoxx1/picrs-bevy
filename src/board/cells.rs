use super::{BORDER_TO_CELL_FG_RATIO, CELL_CROSSED_COLOR, CELL_FG_COLOR, CELL_FILLED_COLOR};
use bevy::prelude::*;

use crate::{
    CellCount, GameState,
    board::bg::{GridBg, draw_board_bg},
};

pub struct CellsPlugin;
impl Plugin for CellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_board_cells.after(draw_board_bg));
    }
}

enum CellState {
    Filled,
    Empty,
    Crossed,
}
#[derive(Component)]
struct Cell {
    cell_state: CellState,
    coords: (usize, usize),
}

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
    let border_size = border_size + ((n - 1) as f32 * border_size) / (n as f32 + 1.);
    let fg_size = (BORDER_TO_CELL_FG_RATIO.1
        / (BORDER_TO_CELL_FG_RATIO.0 + BORDER_TO_CELL_FG_RATIO.1))
        / n as f32;
    //+ (((n - 1) as f32 * border_size) / n as f32);

    let top_of_board = 1. / 2.;
    let left_of_board = -top_of_board;

    let white_matl = material.add(Color::WHITE);
    let crossed_matl = material.add(Color::srgb_from_array(CELL_CROSSED_COLOR));
    let filled_matl = material.add(Color::srgb_from_array(CELL_FILLED_COLOR));

    for x in 0..cell_count.ncol {
        for y in 0..cell_count.nrow {
            commands
                .spawn((
                    Cell {
                        cell_state: CellState::Empty,
                        coords: (x, y),
                    },
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
                ))
                .observe(toggle_state(
                    filled_matl.clone(),
                    white_matl.clone(),
                    crossed_matl.clone(),
                ));
        }
    }
}
#[allow(clippy::type_complexity)]
fn toggle_state(
    filled_material: Handle<ColorMaterial>,
    empty_material: Handle<ColorMaterial>,
    crossed_material: Handle<ColorMaterial>,
) -> impl Fn(On<Pointer<Press>>, Query<(&mut Cell, &mut MeshMaterial2d<ColorMaterial>)>, ResMut<GameState>)
{
    move |event, mut query, mut state| {
        if let Ok(mut query) = query.get_mut(event.entity) {
            match query.0.cell_state {
                CellState::Empty => {
                    query.0.cell_state = CellState::Filled;
                    query.1.0 = filled_material.clone();
                    state.toggle_square(query.0.coords);
                }
                CellState::Filled => {
                    query.0.cell_state = CellState::Crossed;
                    query.1.0 = crossed_material.clone();
                    state.toggle_square(query.0.coords);
                }
                CellState::Crossed => {
                    query.0.cell_state = CellState::Empty;
                    query.1.0 = empty_material.clone()
                }
            }
            if state.is_solved() {
                println!("solved :3");
            }
        }
    }
}
