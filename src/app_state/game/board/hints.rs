pub mod hint_text;
pub mod left_hints;
pub mod top_hints;

use bevy::prelude::*;
use hint_text::HintTextPlugin;
use left_hints::LeftHintsPlugin;
use top_hints::TopHintsPlugin;

pub struct HintsPlugin;
impl Plugin for HintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TopHintsPlugin, LeftHintsPlugin, HintTextPlugin));
    }
}
