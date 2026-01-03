use bevy::prelude::*;

use crate::{
    CellCount,
    board::{
        BORDER_TO_HINTS_FG_RATIO, HINTS_FG_COLOR,
        bg::{TopHintBg, draw_board_bg},
    },
};

pub struct TopHintsPlugin;
impl Plugin for TopHintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_hints.after(draw_board_bg));
    }
}

#[derive(Component)]
struct Hint;

fn draw_hints(
    cell_count: Res<CellCount>,
    hint_bg: Single<(Entity, &Transform), With<TopHintBg>>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let n = cell_count.ncol;
    let left_of_board = -0.5;
    let border_size = ((BORDER_TO_HINTS_FG_RATIO.0
        / (BORDER_TO_HINTS_FG_RATIO.1 + BORDER_TO_HINTS_FG_RATIO.0))
        / n as f32)
        / 2.;
    let border_size = border_size + ((n - 1) as f32 * border_size) / (n as f32 + 1.);
    let fg_size = (BORDER_TO_HINTS_FG_RATIO.1
        / (BORDER_TO_HINTS_FG_RATIO.0 + BORDER_TO_HINTS_FG_RATIO.1))
        / n as f32;
    for x in 0..cell_count.ncol {
        commands.spawn((
            Hint,
            Mesh2d(mesh.add(Rectangle::default())),
            MeshMaterial2d(material.add(Color::srgb_from_array(HINTS_FG_COLOR))),
            Transform::from_translation(Vec3::new(
                left_of_board + (border_size + fg_size / 2.) + (border_size + fg_size) * x as f32,
                0. - (hint_bg.1.scale.x * border_size) * 1.5,
                1.,
            ))
            .with_scale(Vec3::new(fg_size, 1. - (hint_bg.1.scale.x * border_size) * 3., 1.)),
            ChildOf(hint_bg.0),
        ));
    }
}
