use crate::layout::WindowDimensions;
use bevy::prelude::*;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_board_bg);
    }
}

const NCOL: usize = 5;
const NROW: usize = 5;
const GRID_BG_COLOR: [f32; 3] = [0., 0., 0.];

#[derive(Component)]
struct Grid;

fn draw_board_bg(
    option_window_dimensions: Option<Res<WindowDimensions>>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(window_dimensions) = option_window_dimensions {
        let left_of_screen = window_dimensions.left_of_screen;
        let left_margin = window_dimensions.left_margin;
        let size = window_dimensions.board_size;
        let top_of_screen = window_dimensions.top_of_screen;
        let top_margin = window_dimensions.top_margin;
        commands.spawn((
            Grid,
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
}
