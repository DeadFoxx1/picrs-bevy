use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_grid)
            .add_systems(Update, redraw_grid);
    }
}

const NCOL: usize = 5;
const NROW: usize = 5;
const SIDE_MARGIN_RATIO: (f32, f32) = (2., 1.); //2:1
const TOP_MARGIN_PERCENT: f32 = 0.05; //5% of the screen's height per margin
//in the case that screen width < screen height
const MIN_SIZE_OF_SQUARE_PERCENT: f32 = 0.70; //70%

#[derive(Component)]
struct Grid;

#[derive(Message)]
struct RedrawGridEvent;

fn draw_grid(
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let window_height = window.height();
    let window_width = window.width();
    let top_margin = window.height() * TOP_MARGIN_PERCENT;
    let size = f32::min(
        window_height - (2. * top_margin),
        window_width * MIN_SIZE_OF_SQUARE_PERCENT,
    );
    let left_margin =
        ((window_width - size) * SIDE_MARGIN_RATIO.0) / (SIDE_MARGIN_RATIO.0 + SIDE_MARGIN_RATIO.1);
    commands.spawn((
        Grid,
        Mesh2d(mesh.add(Rectangle::default())),
        MeshMaterial2d(material.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(
            -(window_width / 2.) + left_margin + (size / 2.),
            (window_height / 2.) - top_margin - (size / 2.),
            0.,
        )
        .with_scale(Vec3::new(size, size, 0.)),
    ));
}

fn redraw_grid(
    mut redraw_grid_events: MessageReader<WindowResized>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mesh: ResMut<Assets<Mesh>>,
    material: ResMut<Assets<ColorMaterial>>,
    grid: Single<Entity, With<Grid>>,
) {
    if !redraw_grid_events.is_empty() {
        redraw_grid_events.clear();
        commands.entity(*grid).despawn();
        draw_grid(window, commands, mesh, material);
    }
}
