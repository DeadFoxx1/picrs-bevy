mod grid;
mod layout;

use crate::grid::GridPlugin;
use crate::layout::LayoutPlugin;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, GridPlugin, LayoutPlugin))
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2d);
}
