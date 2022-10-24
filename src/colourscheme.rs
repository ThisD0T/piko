use bevy::prelude::*;

use crate::GameState;

pub struct ColourPlugin;

impl Plugin for ColourPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing)
        )
    }
}

pub struct ColourScheme {
    colour_0: Color,
    colour_1: Color,
    colour_2: Color,
}



