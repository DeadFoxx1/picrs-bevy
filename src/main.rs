use bevy::prelude::*;
use picrs_bevy::GamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MeshPickingPlugin, GamePlugin))
        .run();
}
