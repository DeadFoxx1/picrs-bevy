use crate::layout::bg_layout::{BgDimensions, init_bg_dimensions};
use bevy::prelude::*;

pub struct GridBgPlugin;
impl Plugin for GridBgPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_board_bg.after(init_bg_dimensions))
            .add_systems(Update, update_board_bg);
    }
}

const GRID_BG_COLOR: [f32; 3] = [0., 0., 0.];

#[derive(Component)]
pub struct Bg;

fn draw_board_bg(
    bg_dimensions: Res<BgDimensions>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let left_of_screen = bg_dimensions.left_of_screen;
    let left_margin = bg_dimensions.left_margin;
    let size = bg_dimensions.board_size;
    let top_of_screen = bg_dimensions.top_of_screen;
    let top_margin = bg_dimensions.top_margin;
    commands.spawn((
        Bg,
        Mesh2d(mesh.add(Rectangle::default())),
        MeshMaterial2d(material.add(Color::srgb_from_array(GRID_BG_COLOR))),
        Transform::from_xyz(
            //account for origin of mesh being in the center with (size/2.)
            left_of_screen + left_margin + (size / 2.),
            top_of_screen - top_margin - (size / 2.),
            0., //set the bottom of the z index
        )
        .with_scale(Vec3::new(size, size, 0.)),
    ));
}

fn update_board_bg(
    mut bg: Single<&mut Transform, With<Bg>>,
    bg_dimensions: Res<BgDimensions>,
) {
    let left_of_screen = bg_dimensions.left_of_screen;
    let left_margin = bg_dimensions.left_margin;
    let size = bg_dimensions.board_size;
    let top_of_screen = bg_dimensions.top_of_screen;
    let top_margin = bg_dimensions.top_margin;

    bg.translation.x = left_of_screen + left_margin + (size / 2.);
    bg.translation.y = top_of_screen - top_margin - (size / 2.);
    bg.scale.x = size;
    bg.scale.y = size;
}

