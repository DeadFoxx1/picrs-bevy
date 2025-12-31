mod draw;
mod layout;

use crate::draw::DrawPlugin;
use crate::layout::LayoutPlugin;
use bevy::prelude::*;

const START_NROW: usize = 10;
const START_NCOL: usize = 10;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, LayoutPlugin, DrawPlugin))
        .add_systems(Startup, init).insert_resource(CellCount{nrow: START_NROW, ncol: START_NCOL})
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource)]
pub struct CellCount {
    nrow: usize,
    ncol: usize
}
