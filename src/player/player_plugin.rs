use bevy::prelude::*;

use crate::player::player_systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player)
            .add_systems(Update, move_player)
            .add_systems(Update, mouse_click_system);
    }
}