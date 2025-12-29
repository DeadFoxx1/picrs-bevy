pub mod bg_layout;

use bevy::prelude::*;
use bg_layout::BgLayoutPlugin;

pub struct LayoutPlugin;
impl Plugin for LayoutPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(BgLayoutPlugin);
    }
}
