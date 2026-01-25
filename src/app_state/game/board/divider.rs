use bevy::prelude::*;

use crate::{
    app_state::{
        game::board::{
            bg::{draw_board_bg, GridBg},
            BORDER_TO_CELL_FG_RATIO, DIVIDER_COLOR,
        },
        AppState,
    },
    CellCount,
};

pub struct DividerPlugin;

impl Plugin for DividerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), draw_divider.after(draw_board_bg));
    }
}

#[derive(Component)]
struct Divider;

fn draw_divider(
    cell_count: Res<CellCount>,
    board_bg: Single<Entity, With<GridBg>>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let n = usize::max(cell_count.nrow, cell_count.ncol);
    let top_of_board = 1. / 2.;
    let left_of_board = -top_of_board;

    let border_size = ((BORDER_TO_CELL_FG_RATIO.0
        / (BORDER_TO_CELL_FG_RATIO.1 + BORDER_TO_CELL_FG_RATIO.0))
        / n as f32)
        / 2.;
    let border_size = border_size + ((n - 1) as f32 * border_size) / (n as f32 + 1.);
    let fg_size = (BORDER_TO_CELL_FG_RATIO.1
        / (BORDER_TO_CELL_FG_RATIO.0 + BORDER_TO_CELL_FG_RATIO.1))
        / n as f32;

    for mut x in 0..cell_count.ncol {
        if x % 5 == 0 || x + 1 == cell_count.ncol {
            if x + 1 == cell_count.ncol {
                x += 1;
            }
            commands.spawn((
                Divider,
                Mesh2d(mesh.add(Rectangle::default())),
                MeshMaterial2d(material.add(Color::srgb_from_array(DIVIDER_COLOR))),
                Transform::from_translation(Vec3::new(
                    left_of_board
                        + (border_size + fg_size / 2.)
                        + (border_size + fg_size) * x as f32
                        - fg_size / 2.
                        - border_size / 2.,
                    0.,
                    3.,
                ))
                .with_scale(Vec3::new(border_size, 1., 1.)),
                ChildOf(*board_bg),
            ));
        }
    }
    for mut y in 0..cell_count.nrow {
        if y % 5 == 0 || y + 1 == cell_count.nrow {
            if y + 1 == cell_count.nrow {
                y += 1;
            }
            commands.spawn((
                Divider,
                Mesh2d(mesh.add(Rectangle::default())),
                MeshMaterial2d(material.add(Color::srgb_from_array(DIVIDER_COLOR))),
                Transform::from_translation(Vec3::new(
                    0.,
                    top_of_board
                        - (border_size + fg_size / 2.)
                        - (border_size + fg_size) * y as f32
                        + fg_size / 2.
                        + border_size / 2.,
                    3.,
                ))
                .with_scale(Vec3::new(1., border_size, 1.)),
                ChildOf(*board_bg),
            ));
        }
    }
}
