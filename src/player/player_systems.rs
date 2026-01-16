use bevy::prelude::*;

use crate::player::player_types::Player;

pub fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Player,
        Sprite::from_image(
        asset_server.load("player.png"),
    )));
}

pub fn move_player(time: Res<Time>, mut players_pos: Query<&mut Transform, With<Player>>) {
     for mut players_pos in &mut players_pos {
        players_pos.translation.x += 150. * time.delta_secs();
     }
}