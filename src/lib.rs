use bevy::prelude::*;
// your dependencies
// ...

#[bevy_main]
fn main() {
    run_game();
}

pub fn run_game() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // your code
        // ...
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_color(Color::WHITE, Vec2::new(10., 10.)));
}
