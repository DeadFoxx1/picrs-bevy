pub mod top_hints;
pub mod left_hints;
pub mod hint_text;

use bevy::prelude::*;
use top_hints::TopHintsPlugin;
use left_hints::LeftHintsPlugin;
use hint_text::HintTextPlugin;

pub struct HintsPlugin;
impl Plugin for HintsPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((TopHintsPlugin, LeftHintsPlugin, HintTextPlugin));
    }
}
