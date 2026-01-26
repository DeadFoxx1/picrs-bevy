use crate::app_state::{game::GameComponent, AppState};

use super::{
    GRID_BG_COLOR, HINTS_PERCENT, HINT_BG_COLOR, HORI_MARGIN_PERCENT, HORI_MARGIN_RATIO,
    VERT_MARGIN_PERCENT, VERT_MARGIN_RATIO,
};
use bevy::{prelude::*, window::WindowResized};

pub struct BgPlugin;
impl Plugin for BgPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (draw_board_bg, init).chain())
            .add_observer(update_bg)
            .add_systems(
                Update,
                handle_window_resized.run_if(in_state(AppState::InGame)),
            );
    }
}

fn init(mut commands: Commands) {
    commands.trigger(UpdateBg);
}

#[derive(Component)]
pub struct GridBg;

#[derive(Component)]
pub struct TopHintBg;

#[derive(Component)]
pub struct LeftHintBg;

pub fn draw_board_bg(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        GameComponent,
        GridBg,
        Mesh2d(mesh.add(Rectangle::default())),
        MeshMaterial2d(material.add(Color::srgb_from_array(GRID_BG_COLOR))),
    ));
    commands.spawn((
        GameComponent,
        TopHintBg,
        Mesh2d(mesh.add(Rectangle::default())),
        MeshMaterial2d(material.add(Color::srgb_from_array(HINT_BG_COLOR))),
    ));

    commands.spawn((
        GameComponent,
        LeftHintBg,
        Mesh2d(mesh.add(Rectangle::default())),
        MeshMaterial2d(material.add(Color::srgb_from_array(HINT_BG_COLOR))),
    ));
}

#[derive(Event)]
struct UpdateBg;

fn handle_window_resized(mut resized_events: MessageReader<WindowResized>, mut commands: Commands) {
    for _ in resized_events.read() {
        commands.trigger(UpdateBg);
    }
}

#[allow(clippy::type_complexity)]
fn update_bg(
    _: On<UpdateBg>,
    mut grid_bg: Single<&mut Transform, (With<GridBg>, Without<TopHintBg>, Without<LeftHintBg>)>,
    mut top_hint_bg: Single<
        &mut Transform,
        (With<TopHintBg>, Without<GridBg>, Without<LeftHintBg>),
    >,
    mut left_hint_bg: Single<
        &mut Transform,
        (With<LeftHintBg>, Without<GridBg>, Without<TopHintBg>),
    >,
    window: Single<&Window>,
) {
    let window_width = window.width();
    let window_height = window.height();

    let grid_size = f32::min(
        window_height * (1. - VERT_MARGIN_PERCENT),
        window_width * (1. - HORI_MARGIN_PERCENT),
    );
    let hint_size = grid_size * HINTS_PERCENT;
    let grid_size = grid_size - hint_size;

    let top_margin = ((window_height - grid_size - hint_size) * VERT_MARGIN_RATIO.0)
        / (VERT_MARGIN_RATIO.0 + VERT_MARGIN_RATIO.1);
    let left_margin = ((window_width - grid_size - hint_size) * HORI_MARGIN_RATIO.0)
        / (HORI_MARGIN_RATIO.0 + HORI_MARGIN_RATIO.1);

    let top_of_screen = window_height / 2.;
    let left_of_screen = -(window_width / 2.);

    grid_bg.translation.x = left_of_screen + hint_size + left_margin + (grid_size / 2.);
    grid_bg.translation.y = top_of_screen - hint_size - top_margin - (grid_size / 2.);
    grid_bg.scale.x = grid_size;
    grid_bg.scale.y = grid_size;

    top_hint_bg.translation.x = grid_bg.translation.x;
    top_hint_bg.translation.y = grid_bg.translation.y + (grid_size / 2.) + (hint_size / 2.);
    top_hint_bg.scale.x = grid_bg.scale.x;
    top_hint_bg.scale.y = hint_size;

    left_hint_bg.translation.x = grid_bg.translation.x - (grid_size / 2.) - (hint_size / 2.);
    left_hint_bg.translation.y = grid_bg.translation.y;
    left_hint_bg.scale.x = hint_size;
    left_hint_bg.scale.y = grid_bg.scale.y;
}
