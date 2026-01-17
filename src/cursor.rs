use bevy::prelude::*;

use crate::{
    GameState,
    board::cells::{Cell, CellMatl, CellState},
};

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorState {
            cell_state: CellState::Empty,
        });
    }
}

#[derive(Resource)]
pub struct CursorState {
    //state cursor is changing other cells to
    pub cell_state: CellState,
}

#[allow(clippy::type_complexity)]
pub fn toggle_cursor() -> impl Fn(
    On<Pointer<Press>>,
    Query<(&mut Cell, &mut MeshMaterial2d<ColorMaterial>)>,
    ResMut<CursorState>,
    ResMut<GameState>,
    Res<CellMatl>,
) {
    move |event, mut query, mut cursor_state, mut game_state, cell_matl| {
        if let Ok(mut query) = query.get_mut(event.entity) {
            match query.0.cell_state {
                CellState::Empty => {
                    *cursor_state = CursorState {
                        cell_state: CellState::Filled,
                    };
                }
                CellState::Filled => {
                    *cursor_state = CursorState {
                        cell_state: CellState::Crossed,
                    };
                }
                CellState::Crossed => {
                    *cursor_state = CursorState {
                        cell_state: CellState::Empty,
                    };
                }
            }
            update_cell(
                &mut query.0,
                &mut query.1,
                cell_matl,
                &cursor_state,
                &mut game_state,
            );
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn paint_cell() -> impl Fn(
    On<Pointer<DragEnter>>,
    Query<(&mut Cell, &mut MeshMaterial2d<ColorMaterial>)>,
    Res<CellMatl>,
    ResMut<CursorState>,
    ResMut<GameState>,
) {
    move |event, mut query, cell_matl, cursor_state, mut game_state| {
        if let Ok(mut query) = query.get_mut(event.entity) {
            update_cell(
                &mut query.0,
                &mut query.1,
                cell_matl,
                &cursor_state,
                &mut game_state,
            )
        }
    }
}

fn update_cell(
    cell: &mut Cell,
    current_matl: &mut Handle<ColorMaterial>,
    cell_matl: Res<CellMatl>,
    cursor_state: &CursorState,
    game_state: &mut ResMut<GameState>,
) {
    //dont need to change it if its already the same state as the cursor
    if cell.cell_state == cursor_state.cell_state {
        return;
    }

    //update game state
    if cell.cell_state == CellState::Filled
        && (cursor_state.cell_state == CellState::Empty
            || cursor_state.cell_state == CellState::Crossed)
    {
        game_state.toggle_square(cell.coords);
    }

    if (cell.cell_state == CellState::Empty || cell.cell_state == CellState::Crossed)
        && cursor_state.cell_state == CellState::Filled
    {
        game_state.toggle_square(cell.coords);
    }

    //update cell matl
    cell.cell_state = cursor_state.cell_state;
    match cursor_state.cell_state {
        CellState::Filled => *current_matl = cell_matl.filled.clone(),
        CellState::Empty => *current_matl = cell_matl.empty.clone(),
        CellState::Crossed => *current_matl = cell_matl.crossed.clone(),
    }
    if game_state.is_solved() {
        println!("solved :33333 UwU OwO");
    }
}

