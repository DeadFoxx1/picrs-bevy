use bevy::prelude::*;

use crate::app_state::game::{board::BoardPlugin, events::EventPlugin};

mod board;
mod events;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EventPlugin, BoardPlugin));
    }
}
