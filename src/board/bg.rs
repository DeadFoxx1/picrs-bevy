use super::{
    GRID_BG_COLOR, HINT_BG_COLOR, HORI_MARGIN_PERCENT, LEFT_HINTS_PERCENT, SIDE_MARGIN_RATIO,
    TOP_HINTS_PERCENT, VERT_MARGIN_PERCENT,
};
use bevy::{prelude::*, window::WindowResized};

pub struct BgPlugin;
impl Plugin for BgPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_board_bg)
            .add_systems(Update, update_bg);
    }
}

#[derive(Component)]
pub struct GridBg;

#[derive(Component)]
pub struct TopHintBg;

#[derive(Component)]
pub struct BottomHintBg;

pub fn draw_board_bg(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        GridBg,
        Mesh2d(mesh.add(Rectangle::default())),
        MeshMaterial2d(material.add(Color::srgb_from_array(GRID_BG_COLOR))),
    ));
    commands.spawn((
        TopHintBg,
        Mesh2d(mesh.add(Rectangle::default())),
        MeshMaterial2d(material.add(Color::srgb_from_array(HINT_BG_COLOR))),
    ));

    commands.spawn((
        BottomHintBg,
        Mesh2d(mesh.add(Rectangle::default())),
        MeshMaterial2d(material.add(Color::srgb_from_array(HINT_BG_COLOR))),
    ));
}
#[allow(clippy::type_complexity)]
fn update_bg(
    mut grid_bg: Single<&mut Transform, (With<GridBg>, Without<TopHintBg>, Without<BottomHintBg>)>,
    mut top_hint_bg: Single<
        &mut Transform,
        (With<TopHintBg>, Without<GridBg>, Without<BottomHintBg>),
    >,
    mut left_hint_bg: Single<
        &mut Transform,
        (With<BottomHintBg>, Without<GridBg>, Without<TopHintBg>),
    >,
    mut resized_events: MessageReader<WindowResized>,
) {
    for event in resized_events.read() {
        let window_width = event.width;
        let window_height = event.height;

        let top_margin = window_height * VERT_MARGIN_PERCENT / 2.;

        let grid_size = f32::min(
            window_height * (1. - VERT_MARGIN_PERCENT),
            window_width * (1. - HORI_MARGIN_PERCENT),
        );
        let top_hint = grid_size * TOP_HINTS_PERCENT;
        let left_hint = grid_size * LEFT_HINTS_PERCENT;
        let grid_size = grid_size - top_hint;
        let left_margin = ((window_width - grid_size) * SIDE_MARGIN_RATIO.0)
            / (SIDE_MARGIN_RATIO.0 + SIDE_MARGIN_RATIO.1);
        let top_of_screen = window_height / 2.;
        let left_of_screen = -(window_width / 2.);

        grid_bg.translation.x = left_of_screen + left_margin + (grid_size / 2.);
        grid_bg.translation.y = top_of_screen - top_hint - top_margin - (grid_size / 2.);
        grid_bg.scale.x = grid_size;
        grid_bg.scale.y = grid_size;

        top_hint_bg.translation.x = grid_bg.translation.x;
        top_hint_bg.translation.y = grid_bg.translation.y + (grid_size / 2.) + (top_hint / 2.);
        top_hint_bg.scale.x = grid_bg.scale.x;
        top_hint_bg.scale.y = top_hint;

        left_hint_bg.translation.x = grid_bg.translation.x - (grid_size / 2.) - (left_hint / 2.);
        left_hint_bg.translation.y = grid_bg.translation.y;
        left_hint_bg.scale.x = left_hint;
        left_hint_bg.scale.y = grid_bg.scale.y;
    }
}
