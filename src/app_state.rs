use bevy::prelude::*;

use crate::app_state::game::GameStatePlugin;

mod game;
mod main_menu;

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_plugins(GameStatePlugin);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
    Paused,
    Settings,
}
