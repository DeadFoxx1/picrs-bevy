use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_grid_bg)
            .add_systems(Update, redraw_grid);
    }
}

const NCOL: usize = 5;
const NROW: usize = 5;
const SIDE_MARGIN_RATIO: (f32, f32) = (2., 1.); //2:1
const TOP_MARGIN_PERCENT: f32 = 0.05; //5% of the screen's height per margin

//in the case that screen width < screen height
const MIN_SIZE_OF_SQUARE_PERCENT: f32 = 0.70; //70% of the screens width
const GRID_BG_COLOR: [f32; 3] = [0., 0., 0.];

#[derive(Component)]
struct Grid;

#[derive(Message)]
struct RedrawGridEvent;

fn draw_grid_bg(
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
    let top_of_screen = window_height / 2.;
    let left_of_screen = -(window_width / 2.);
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
        draw_grid_bg(window, commands, mesh, material);
    }
}
