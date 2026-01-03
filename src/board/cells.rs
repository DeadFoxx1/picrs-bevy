use super::{BORDER_TO_CELL_FG_RATIO, CELL_FG_COLOR, CELL_FILLED_COLOR, CELL_HOVER_COLOR};
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
    let border_size = border_size + ((n - 1) as f32 * border_size) / (n as f32 + 1.);
    let fg_size = (BORDER_TO_CELL_FG_RATIO.1
        / (BORDER_TO_CELL_FG_RATIO.0 + BORDER_TO_CELL_FG_RATIO.1))
        / n as f32;
    //+ (((n - 1) as f32 * border_size) / n as f32);

    let top_of_board = 1. / 2.;
    let left_of_board = -top_of_board;

    let white_matl = material.add(Color::WHITE);
    let hover_matl = material.add(Color::srgb_from_array(CELL_HOVER_COLOR));
    let pressed_matl = material.add(Color::srgb_from_array(CELL_FILLED_COLOR));


    for x in 0..cell_count.ncol {
        for y in 0..cell_count.nrow {
            commands
                .spawn((
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
                ))
                .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
                .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
                .observe(update_material_on::<Pointer<Press>>(pressed_matl.clone()))
                .observe(update_material_on::<Pointer<Release>>(hover_matl.clone()));
        }
    }
}
fn update_material_on<E: EntityEvent>(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial2d<ColorMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}
