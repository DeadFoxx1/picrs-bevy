use bevy::{prelude::*, window::WindowResized};

pub struct GridBgPlugin;
impl Plugin for GridBgPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_board_bg)
            .add_systems(Update, update_board_bg);
    }
}

const GRID_BG_COLOR: [f32; 3] = [0., 0., 0.];
const SIDE_MARGIN_RATIO: (f32, f32) = (9., 1.); //9:1
const TOP_MARGIN_PERCENT: f32 = 0.05; //5% of the screen's height per margin
const TOP_HINT_PERCENT: f32 = 0.25;

//in the case that screen width < screen height
const MIN_SIZE_OF_SQUARE_PERCENT: f32 = 0.70; //70% of the screens width

#[derive(Component)]
pub struct Bg;

pub fn draw_board_bg(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Bg,
        Mesh2d(mesh.add(Rectangle::default())),
        MeshMaterial2d(material.add(Color::srgb_from_array(GRID_BG_COLOR))),
    ));
}

fn update_board_bg(
    mut bg: Single<&mut Transform, With<Bg>>,
    mut resized_events: MessageReader<WindowResized>,
) {
    for event in resized_events.read() {
        let window_width = event.width;
        let window_height = event.height;
        let top_margin = window_height * TOP_MARGIN_PERCENT;
        let top_hint = window_height * TOP_HINT_PERCENT;
        let board_size = f32::min(
            window_height - (2. * top_margin) - top_hint,
            window_width * MIN_SIZE_OF_SQUARE_PERCENT,
        );
        let left_margin = ((window_width - board_size) * SIDE_MARGIN_RATIO.0)
            / (SIDE_MARGIN_RATIO.0 + SIDE_MARGIN_RATIO.1);
        let top_of_screen = window_height / 2.;
        let left_of_screen = -(window_width / 2.);

        bg.translation.x = left_of_screen + left_margin + (board_size / 2.);
        bg.translation.y = top_of_screen - top_hint - top_margin - (board_size / 2.);
        bg.scale.x = board_size;
        bg.scale.y = board_size;
    }
}
