use bevy::prelude::*;

use crate::{
    GameState,
    app_state::{
        AppState,
        game::board::cells::{Cell, CellMatl, CellState},
    },
};

pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorState {
            cell_state: CellState::Empty,
        })
        .add_message::<GameWin>()
        .add_systems(Update, game_win.run_if(in_state(AppState::InGame)));
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
    Commands,
) {
    move |event, mut query, mut cursor_state, mut game_state, cell_matl, mut commands| {
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
                &mut commands,
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
    Commands,
) {
    move |event, mut query, cell_matl, cursor_state, mut game_state, mut commands| {
        if let Ok(mut query) = query.get_mut(event.entity) {
            update_cell(
                &mut query.0,
                &mut query.1,
                cell_matl,
                &cursor_state,
                &mut game_state,
                &mut commands,
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
    commands: &mut Commands,
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
        commands.write_message(GameWin);
    }
}

#[derive(Message)]
struct GameWin;

fn game_win(
    mut messages: MessageReader<GameWin>,
    mut query: Query<(&Cell, &mut MeshMaterial2d<ColorMaterial>)>,
    cell_matl: Res<CellMatl>,
) {
    for _message in messages.read() {
        for (cell, mut matl) in query.iter_mut() {
            if cell.cell_state == CellState::Filled {
                **matl = cell_matl.green.clone();
            }
        }
    }
}
