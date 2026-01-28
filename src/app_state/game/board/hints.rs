pub mod hint_text;
pub mod left_hints;
pub mod top_hints;

use bevy::prelude::*;
use hint_text::HintTextPlugin;
use left_hints::LeftHintsPlugin;
use top_hints::TopHintsPlugin;

use crate::app_state::{
    game::board::{HINTS_FG_EMPTY_COLOR, HINTS_FG_HIGHLIGHT_COLOR},
    AppState,
};

pub struct HintsPlugin;
impl Plugin for HintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TopHintsPlugin, LeftHintsPlugin, HintTextPlugin))
            .add_systems(OnEnter(AppState::InGame), HintMatl::init);
    }
}

#[derive(Resource)]
pub struct HintMatl {
    pub empty: Handle<ColorMaterial>,
    pub highlight: Handle<ColorMaterial>,
}

impl HintMatl {
    pub fn init(mut commands: Commands, mut material: ResMut<Assets<ColorMaterial>>) {
        let empty = material.add(Color::srgb_from_array(HINTS_FG_EMPTY_COLOR));
        let highlight = material.add(Color::srgb_from_array(HINTS_FG_HIGHLIGHT_COLOR));
        let hint_matl = HintMatl { empty, highlight };
        commands.insert_resource(hint_matl);
    }
}
