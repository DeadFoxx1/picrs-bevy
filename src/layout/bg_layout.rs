use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

pub struct BgLayoutPlugin;
impl Plugin for BgLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_bg_dimensions)
            .add_systems(Update, update_bg_dimensions);
    }
}

const SIDE_MARGIN_RATIO: (f32, f32) = (9., 1.); //2:1
const TOP_MARGIN_PERCENT: f32 = 0.05; //5% of the screen's height per margin

//in the case that screen width < screen height
const MIN_SIZE_OF_SQUARE_PERCENT: f32 = 0.70; //70% of the screens width

#[derive(Resource)]
pub struct BgDimensions {
    pub top_margin: f32,
    pub board_size: f32,
    pub left_margin: f32,
    pub top_of_screen: f32,
    pub left_of_screen: f32,
}
impl BgDimensions {
    fn new(window_height: f32, window_width: f32) -> Self {
        let top_margin = window_height * TOP_MARGIN_PERCENT;
        let board_size = f32::min(
            window_height - (2. * top_margin),
            window_width * MIN_SIZE_OF_SQUARE_PERCENT,
        );
        let left_margin = ((window_width - board_size) * SIDE_MARGIN_RATIO.0)
            / (SIDE_MARGIN_RATIO.0 + SIDE_MARGIN_RATIO.1);
        let top_of_screen = window_height / 2.;
        let left_of_screen = -(window_width / 2.);

        BgDimensions {
            top_margin,
            board_size,
            left_margin,
            top_of_screen,
            left_of_screen,
        }
    }
}

pub fn init_bg_dimensions(window: Single<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    commands.insert_resource(BgDimensions::new(window.height(), window.width()));
}

fn update_bg_dimensions(
    mut window_dimensions: ResMut<BgDimensions>,
    mut resized_events: MessageReader<WindowResized>,
) {
    for event in resized_events.read() {
        *window_dimensions = BgDimensions::new(event.height, event.width)
    }
}
