use bevy::prelude::*;

use crate::{
    app_state::{
        game::board::hints::{
            left_hints::{draw_left_hints, LeftHint},
            top_hints::{draw_top_hints, TopHint},
        },
        AppState,
    },
    GameState,
};

pub struct HintTextPlugin;
impl Plugin for HintTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            draw_text.after(draw_left_hints).after(draw_top_hints),
        );
    }
}

#[derive(Component)]
struct TopHintText;

#[derive(Component)]
struct LeftHintText;

fn draw_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
    top_hints: Query<(Entity, &TopHint, &Transform)>,
    left_hints: Query<(Entity, &LeftHint, &Transform)>,
) {
    let font = asset_server.load("font/MinecraftTen-VGORe.ttf");
    let font_size = 100.;
    for (entity, index, transform) in top_hints.iter() {
        commands.spawn((
            TopHintText,
            ChildOf(entity),
            Transform::from_xyz(0., 0., 3.).with_scale(Vec3::new(
                1. / font_size,
                1. / font_size * transform.scale.x / transform.scale.y * 2.,
                3.,
            )),
            Text2d::new(
                game_state.columns[index.0]
                    .hints
                    .clone()
                    .into_iter()
                    .map(|i| format!("{}\n", i))
                    .collect::<String>(),
            ),
            TextColor(Color::BLACK),
            TextFont::from(font.clone()).with_font_size(font_size),
            TextLayout::new(Justify::Center, LineBreak::AnyCharacter),
        ));
    }
    for (entity, index, transform) in left_hints.iter() {
        commands.spawn((
            LeftHintText,
            ChildOf(entity),
            Transform::from_xyz(0., 0., 3.).with_scale(Vec3::new(
                1. / font_size * transform.scale.y / transform.scale.x * 2.,
                1. / font_size,
                3.,
            )),
            Text2d::new(
                game_state.rows[index.0]
                    .hints
                    .clone()
                    .into_iter()
                    .map(|i| format!("  {}", i))
                    .collect::<String>(),
            ),
            TextColor(Color::BLACK),
            TextFont::from(font.clone()).with_font_size(font_size),
            TextLayout::new(Justify::Right, LineBreak::AnyCharacter),
        ));
    }
}
