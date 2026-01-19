use bevy::prelude::*;

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
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
