use bevy::prelude::*;

use crate::grid::GridPlugin;
mod grid;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, GridPlugin))
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2d);
}
