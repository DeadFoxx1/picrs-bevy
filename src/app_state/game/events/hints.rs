use bevy::prelude::*;

use crate::app_state::{
    game::{
        board::hints::{left_hints::LeftHint, top_hints::TopHint, HintMatl},
        events::Highlight,
    },
    AppState,
};

pub struct HintEventsPlugin;
impl Plugin for HintEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, highlight.run_if(in_state(AppState::InGame)));
    }
}
fn highlight(
    mut messages: MessageReader<Highlight>,
    top_hints: Query<(Entity, &TopHint)>,
    left_hints: Query<(Entity, &LeftHint)>,
    hint_matl: Res<HintMatl>,
    mut commands: Commands,
) {
    for message in messages.read() {
        for (entity, hint) in top_hints {
            if hint.index == message.x {
                commands
                    .entity(entity)
                    .insert(MeshMaterial2d(hint_matl.highlight.clone()));
            } else {
                commands
                    .entity(entity)
                    .insert(MeshMaterial2d(hint_matl.empty.clone()));
            }
        }
        for (entity, hint) in left_hints {
            if hint.index == message.y {
                commands
                    .entity(entity)
                    .insert(MeshMaterial2d(hint_matl.highlight.clone()));
            } else {
                commands
                    .entity(entity)
                    .insert(MeshMaterial2d(hint_matl.empty.clone()));
            }
        }
    }
}
