use bevy::prelude::*;
use player::player_plugin::PlayerPlugin;

mod player;

#[bevy_main]
fn main() {
    run_game();
}

pub fn run_game() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
