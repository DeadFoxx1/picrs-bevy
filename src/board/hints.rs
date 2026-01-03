pub mod top_hints;

use bevy::prelude::*;
use top_hints::TopHintsPlugin;

pub struct HintsPlugin;
impl Plugin for HintsPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(TopHintsPlugin);
    }
}
