use bevy::prelude::*;

use crate::app_state::{game::{board::BoardPlugin, events::EventPlugin}, AppState};

mod board;
mod events;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EventPlugin, BoardPlugin)).add_systems(OnExit(AppState::InGame), erase);
    }
}

#[derive(Component)]
struct GameComponent;

fn erase(mut query: Query<Entity, With<GameComponent>>, mut commands: Commands){
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
