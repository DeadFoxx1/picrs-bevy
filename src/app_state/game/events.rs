use bevy::prelude::*;

pub mod cells;
mod game;
pub mod hints;

use crate::app_state::game::events::{
    cells::CellEventsPlugin, game::GameEventsPlugin, hints::HintEventsPlugin,
};

pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CellEventsPlugin, GameEventsPlugin, HintEventsPlugin))
            .add_message::<Highlight>();
    }
}
#[derive(Message)]
pub struct Highlight {
    pub x: usize,
    pub y: usize,
}
